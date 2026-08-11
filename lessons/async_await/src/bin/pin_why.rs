//! Почему подпись именно `fn poll(self: Pin<&mut Self>, ...)`.
//!
//! Запуск: cargo run -p async_await --bin pin_why

use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

fn main() {
    part1_self_reference_is_real();
    part2_what_mut_self_would_allow();
    part3_manual_desugaring();
    part4_who_is_unpin();
}

// ---------------------------------------------------------------------------
// 1. Самоссылка внутри future — не теория. Вот её адреса.
// ---------------------------------------------------------------------------
fn part1_self_reference_is_real() {
    println!("\n=== 1. Future реально указывает внутрь себя ===\n");

    let mut fut = pin!(async {
        let x: u64 = 0xAABB_CCDD_EEFF_0011;
        let r: &u64 = &x; // борроу переживает await -> оба лежат В future
        YieldOnce::new().await;
        std::hint::black_box(r);
    });

    let base = &*fut as *const _ as *const u8;
    println!("  адрес самого future : {base:p}");
    println!("  размер future       : {} байт", size_of_val(&*fut));

    let mut cx = Context::from_waker(Waker::noop());
    let _ = fut.as_mut().poll(&mut cx); // доходим до точки приостановки

    // читаем сырые байты: local .x лежит по offset 0, local .r сразу за ним
    let bytes = unsafe { std::slice::from_raw_parts(base, size_of_val(&*fut)) };
    let x_val = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
    let r_val = u64::from_le_bytes(bytes[8..16].try_into().unwrap());

    println!("\n  После первого poll, байты внутри future:");
    println!("    offset 0..8  = {x_val:#018x}   <- сама переменная x");
    println!("    offset 8..16 = {r_val:#018x}   <- ссылка r");
    println!("    адрес future = {:#018x}", base as usize);
    println!(
        "\n  r указывает на offset {} ОТ НАЧАЛА future. Это самоссылка.",
        r_val as usize - base as usize
    );
    println!("  Сдвинь эти байты в памяти — r станет висячим, а компилятор");
    println!("  об этом не узнает: внутри стейт-машины это сырой указатель.");
    println!("  Именно поэтому poll требует Pin: обещание «адрес больше не меняется».");
}

// ---------------------------------------------------------------------------
// 2. Что позволил бы &mut self
// ---------------------------------------------------------------------------
fn part2_what_mut_self_would_allow() {
    println!("\n=== 2. Почему не `&mut self` ===\n");

    /// Самоссылающийся тип «как у компилятора»: ptr смотрит на своё же поле data.
    struct SelfRef {
        data: u64,
        ptr: *const u64,
    }

    impl SelfRef {
        fn new(data: u64) -> Self {
            SelfRef { data, ptr: std::ptr::null() }
        }
        /// аналог «первого poll»: зафиксировали внутренний указатель
        fn init(&mut self) {
            self.ptr = &self.data;
        }
        fn read(&self) -> u64 {
            unsafe { *self.ptr }
        }
    }

    let mut a = SelfRef::new(0x1111_1111);
    let mut b = SelfRef::new(0x2222_2222);
    a.init();
    b.init();

    println!("  до swap:");
    println!("    a.data = {:#010x}   a.read() = {:#010x}   совпадает: {}", a.data, a.read(), a.data == a.read());
    println!("    b.data = {:#010x}   b.read() = {:#010x}   совпадает: {}", b.data, b.read(), b.data == b.read());

    // &mut даёт право на mem::swap. Данные переехали, внутренние указатели — нет.
    std::mem::swap(&mut a, &mut b);

    println!("  после mem::swap(&mut a, &mut b):");
    println!("    a.data = {:#010x}   a.read() = {:#010x}   совпадает: {}", a.data, a.read(), a.data == a.read());
    println!("    b.data = {:#010x}   b.read() = {:#010x}   совпадает: {}", b.data, b.read(), b.data == b.read());
    println!();
    println!("  Оба объекта сломаны: read() лезет по адресу, где данные лежали ДО swap.");
    println!("  Байты переехали, а внутренний указатель остался смотреть на старое место.");
    println!();
    println!("  swap не unsafe. mem::replace, take, *self = ... — тоже.");
    println!("  Всё, что нужно — это &mut. Если бы poll брал &mut self, любой");
    println!("  безопасный код мог бы так сломать приостановленный future.");
    println!("  Pin<&mut T> — это &mut T, у которого отобрали право отдавать");
    println!("  наружу &mut T (пока T: !Unpin). Нет &mut — нет swap.");
}

// ---------------------------------------------------------------------------
// 3. Ручная десугаризация async fn
// ---------------------------------------------------------------------------

/// Лист-future: первый poll -> Pending, второй -> Ready. Заменяет «настоящее» IO.
struct YieldOnce {
    polled: bool,
}

impl YieldOnce {
    fn new() -> Self {
        YieldOnce { polled: false }
    }
}

impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.polled {
            Poll::Ready(())
        } else {
            self.polled = true;
            cx.waker().wake_by_ref(); // «IO готово, разбуди меня снова»
            Poll::Pending
        }
    }
}

/// Что мы десугарим:
///
/// ```ignore
/// async fn fetch(id: u32) -> String {
///     let conn = format!("conn-{id}");   // живёт через ОБА await
///     YieldOnce::new().await;            // точка приостановки 0
///     let head = format!("{conn}/head");
///     YieldOnce::new().await;            // точка приостановки 1
///     format!("{head}+body")
/// }
/// ```
async fn fetch_async(id: u32) -> String {
    let conn = format!("conn-{id}");
    YieldOnce::new().await;
    let head = format!("{conn}/head");
    YieldOnce::new().await;
    format!("{head}+body")
}

/// Ровно то же самое, написанное руками. Имена вариантов — как у rustc.
enum FetchState {
    Unresumed { id: u32 },
    Suspend0 { conn: String, awaitee: YieldOnce },
    Suspend1 { head: String, awaitee: YieldOnce },
    Returned,
    Panicked,
}

struct FetchManual {
    state: FetchState,
    // компилятор помечает свои coroutine как !Unpin; повторяем вручную
    _pin: PhantomPinned,
}

fn fetch_manual(id: u32) -> FetchManual {
    FetchManual {
        state: FetchState::Unresumed { id },
        _pin: PhantomPinned,
    }
}

impl Future for FetchManual {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        // Мы владеем всем типом и знаем, что ничего не двигаем -> берём &mut внутрь.
        // Это тот самый unsafe, который компилятор пишет за нас.
        let this = unsafe { self.get_unchecked_mut() };

        loop {
            match &mut this.state {
                FetchState::Unresumed { id } => {
                    // код до первого await
                    let conn = format!("conn-{id}");
                    println!("    [manual] state Unresumed -> Suspend0, conn = {conn}");
                    this.state = FetchState::Suspend0 {
                        conn,
                        awaitee: YieldOnce::new(),
                    };
                    // continue: сразу опрашиваем вложенный future
                }

                FetchState::Suspend0 { conn, awaitee } => {
                    // await = вызвать poll вложенного future; Pending -> вернуть Pending наружу
                    let awaitee = unsafe { Pin::new_unchecked(awaitee) };
                    match awaitee.poll(cx) {
                        Poll::Pending => {
                            println!("    [manual] state Suspend0: awaitee Pending -> отдаём Pending");
                            return Poll::Pending;
                        }
                        Poll::Ready(()) => {
                            let head = format!("{conn}/head");
                            println!("    [manual] state Suspend0 -> Suspend1, head = {head}");
                            this.state = FetchState::Suspend1 {
                                head,
                                awaitee: YieldOnce::new(),
                            };
                        }
                    }
                }

                FetchState::Suspend1 { head, awaitee } => {
                    let awaitee = unsafe { Pin::new_unchecked(awaitee) };
                    match awaitee.poll(cx) {
                        Poll::Pending => {
                            println!("    [manual] state Suspend1: awaitee Pending -> отдаём Pending");
                            return Poll::Pending;
                        }
                        Poll::Ready(()) => {
                            let out = format!("{head}+body");
                            println!("    [manual] state Suspend1 -> Returned");
                            this.state = FetchState::Returned;
                            return Poll::Ready(out);
                        }
                    }
                }

                FetchState::Returned => panic!("poll после Ready — это уже твой баг"),
                FetchState::Panicked => panic!("future запаниковал ранее"),
            }
        }
    }
}

fn drive<F: Future>(mut fut: Pin<&mut F>, label: &str) -> F::Output {
    let mut cx = Context::from_waker(Waker::noop());
    let mut n = 0;
    loop {
        n += 1;
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => {
                println!("  {label}: готово за {n} poll");
                return v;
            }
            Poll::Pending => {}
        }
    }
}

fn part3_manual_desugaring() {
    println!("\n=== 3. async fn против руками написанного enum ===\n");

    println!("  размер future от async fn : {} байт", size_of_val(&fetch_async(1)));
    println!("  размер ручного enum       : {} байт", size_of::<FetchManual>());
    println!();

    let out_m = drive(pin!(fetch_manual(7)), "manual");
    let out_a = drive(pin!(fetch_async(7)), "async fn");
    println!("\n  manual   -> {out_m}");
    println!("  async fn -> {out_a}");
    println!("  совпало  -> {}", out_m == out_a);
    println!();
    println!("  Три вещи, которые компилятор сделал за нас:");
    println!("   1. нарезал тело на варианты по точкам await;");
    println!("   2. решил, какие локалы обязаны переехать в стейт (те, что живут через await);");
    println!("   3. написал unsafe для проекции Pin внутрь полей.");
    println!("  Заметь: `loop` в poll — это то, почему await не тратит лишний");
    println!("  проход рантайма: Ready вложенного future продолжает работу сразу.");
}

// ---------------------------------------------------------------------------
// 4. Кто такой Unpin
// ---------------------------------------------------------------------------
fn part4_who_is_unpin() {
    println!("\n=== 4. Unpin: кому Pin вообще ничего не запрещает ===\n");

    fn is_unpin<T: Unpin>(_: &T, label: &str) {
        println!("  {label:<44} Unpin");
    }

    is_unpin(&YieldOnce::new(), "YieldOnce (написан руками, без самоссылок)");
    is_unpin(&std::future::ready(1u8), "std::future::Ready<u8>");
    is_unpin(&Box::pin(fetch_async(1)), "Pin<Box<Coroutine>> (сам бокс — Unpin)");

    // не компилируется — раскомментируй и почитай ошибку:
    // is_unpin(&fetch_async(1), "future из async fn");
    // is_unpin(&fetch_manual(1), "наш ручной enum с PhantomPinned");

    println!();
    println!("  Всё, что сгенерировал async, — !Unpin. Поэтому:");
    println!("   * Pin::new(&mut fut) для них НЕ доступен (он требует Unpin);");
    println!("   * нужен pin! (стек) или Box::pin (хип) — они дают Pin через unsafe;");
    println!("   * а YieldOnce: Unpin, поэтому внутри его poll можно писать");
    println!("     `self.polled = true` без всякого unsafe: Pin<&mut Self> у Unpin-типа");
    println!("     свободно отдаёт &mut Self через DerefMut.");
    println!();
    println!("  И главное: Pin<&mut Self> в poll — это arbitrary self type,");
    println!("  то есть ПОЛУЧАТЕЛЬ метода, а не первый аргумент. Ровно поэтому");
    println!("  Future остаётся dyn-совместимым: вызов идёт через vtable по указателю.");
    println!("  Был бы это обычный аргумент типа Pin<&mut Self> — метод перестал бы");
    println!("  быть диспетчеризуемым, и никакого dyn Future в Rust не существовало бы.");
    println!();

    // доказательство: опрашиваем future, не зная его типа
    let mut boxed: Pin<Box<dyn Future<Output = String>>> = Box::pin(fetch_async(42));
    let mut cx = Context::from_waker(Waker::noop());
    let mut n = 0;
    let out = loop {
        n += 1;
        // as_mut() даёт Pin<&mut dyn Future> — тип стёрт, размер неизвестен
        match boxed.as_mut().poll(&mut cx) {
            Poll::Ready(v) => break v,
            Poll::Pending => {}
        }
    };
    println!("  Pin<&mut dyn Future> опрошен {n} раза, результат: {out}");
    println!("  Размер Pin<Box<dyn Future>> = {} B (data + vtable), сам future — в хипе.",
        size_of::<Pin<Box<dyn Future<Output = String>>>>());
}
