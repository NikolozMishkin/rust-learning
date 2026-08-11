//! Где именно внутри future лежит «ссылка на самого себя».
//!
//! pin_why.rs показывает ПОСЛЕДСТВИЯ самоссылки. Здесь — само поле,
//! в которое пишется адрес соседнего поля того же объекта.
//!
//! Запуск: cargo run -p async_await --bin self_reference

use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

fn main() {
    part1_no_self_ref_at_all();
    part2_borrow_across_await();
    part3_awaitee_borrows_parent_local();
    part4_move_breaks_it();
}

/// Лист-future: первый poll -> Pending, второй -> Ready.
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
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// ---------------------------------------------------------------------------
// 1. Стейт-машина БЕЗ самоссылки. Такую можно двигать сколько угодно.
// ---------------------------------------------------------------------------

/// ```ignore
/// async fn fetch(id: u32) -> String {
///     let conn = format!("conn-{id}");   // живёт через await, но НИКТО на него не ссылается
///     YieldOnce::new().await;
///     format!("{conn}/body")             // используется по значению
/// }
/// ```
enum PlainState {
    Unresumed { id: u32 },
    Suspend0 { conn: String, awaitee: YieldOnce },
    Done,
}

struct PlainFetch {
    state: PlainState,
}

impl Future for PlainFetch {
    type Output = String;

    // никакого unsafe: тип Unpin, Pin<&mut Self> сам отдаёт &mut Self
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = self.get_mut();
        loop {
            match &mut this.state {
                PlainState::Unresumed { id } => {
                    let conn = format!("conn-{id}");
                    this.state = PlainState::Suspend0 { conn, awaitee: YieldOnce::new() };
                }
                PlainState::Suspend0 { conn, awaitee } => {
                    match Pin::new(awaitee).poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(()) => {
                            let out = format!("{conn}/body");
                            this.state = PlainState::Done;
                            return Poll::Ready(out);
                        }
                    }
                }
                PlainState::Done => panic!("poll после Ready"),
            }
        }
    }
}

fn part1_no_self_ref_at_all() {
    println!("\n=== 1. Стейт-машина без самоссылки ===\n");

    println!("  enum PlainState {{");
    println!("      Suspend0 {{ conn: String, awaitee: YieldOnce }}   // указателей нет");
    println!("  }}");
    println!();
    println!("  Ровно этот случай — твой FetchManual из pin_why.rs.");
    println!("  String, bool, u32 — ни одного поля, которое смотрит внутрь себя.");
    println!("  Докажем: остановим два таких future посередине и поменяем местами.\n");

    let mut cx = Context::from_waker(Waker::noop());
    let mut a = PlainFetch { state: PlainState::Unresumed { id: 1 } };
    let mut b = PlainFetch { state: PlainState::Unresumed { id: 2 } };

    let _ = Pin::new(&mut a).poll(&mut cx); // оба встали на Suspend0
    let _ = Pin::new(&mut b).poll(&mut cx);

    std::mem::swap(&mut a, &mut b); // безопасный код, никакого unsafe

    let ra = drive(&mut a, &mut cx);
    let rb = drive(&mut b, &mut cx);
    println!("  после swap: a -> {ra}, b -> {rb}");
    println!("  Всё корректно. PlainFetch: Unpin, Pin ему нужен только формально.");
    println!("  Вывод: сам факт «это стейт-машина» самоссылку не создаёт.");
}

fn drive<F: Future + Unpin>(fut: &mut F, cx: &mut Context<'_>) -> F::Output {
    loop {
        if let Poll::Ready(v) = Pin::new(&mut *fut).poll(cx) {
            return v;
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Первый источник самоссылки: борроу локала переживает await.
// ---------------------------------------------------------------------------

/// ```ignore
/// async fn fetch(id: u32) -> String {
///     let conn = format!("conn-{id}");
///     let r: &String = &conn;    // <-- ССЫЛКА НА ЛОКАЛ
///     YieldOnce::new().await;    // <-- переживает точку приостановки
///     format!("{r}/body")        // <-- используется ПОСЛЕ пробуждения
/// }
/// ```
///
/// И `conn`, и `r` обязаны лежать в стейте. А `r` указывает на `conn`,
/// то есть на соседнее поле того же самого варианта enum.
enum SelfRefState {
    Unresumed {
        id: u32,
    },
    Suspend0 {
        conn: String,
        // Написать `r: &'??? String` нельзя: лайфтайм — «пока жив сам этот enum»,
        // а такого лайфтайма в системе типов Rust нет. Поэтому — сырой указатель.
        r: *const String,
        awaitee: YieldOnce,
    },
    Done,
}

struct SelfRefFetch {
    state: SelfRefState,
    _pin: PhantomPinned,
}

impl Future for SelfRefFetch {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.state {
                SelfRefState::Unresumed { id } => {
                    let conn = format!("conn-{id}");

                    // Шаг 1: кладём conn в стейт. ТОЛЬКО ТЕПЕРЬ у него финальный адрес.
                    this.state = SelfRefState::Suspend0 {
                        conn,
                        r: std::ptr::null(),
                        awaitee: YieldOnce::new(),
                    };

                    // Шаг 2: и только теперь можно взять адрес. Вот она, самоссылка.
                    let base = this as *const SelfRefFetch as usize;
                    if let SelfRefState::Suspend0 { conn, r, .. } = &mut this.state {
                        *r = conn as *const String;
                        println!("    адрес всего future      : {base:#x}");
                        println!("    адрес поля conn         : {:p}", *r);
                        println!("    смещение conn от начала : {} байт", *r as usize - base);
                    }
                }

                SelfRefState::Suspend0 { r, awaitee, .. } => {
                    let r = *r;
                    match unsafe { Pin::new_unchecked(awaitee) }.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(()) => {
                            // читаем conn ЧЕРЕЗ самоссылку, а не напрямую
                            let out = format!("{}/body", unsafe { &*r });
                            this.state = SelfRefState::Done;
                            return Poll::Ready(out);
                        }
                    }
                }

                SelfRefState::Done => panic!("poll после Ready"),
            }
        }
    }
}

fn part2_borrow_across_await() {
    println!("\n=== 2. Самоссылка №1: `let r = &conn;` переживает await ===\n");

    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = Box::pin(SelfRefFetch {
        state: SelfRefState::Unresumed { id: 7 },
        _pin: PhantomPinned,
    });

    let out = loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            break v;
        }
    };
    println!("    результат               : {out}\n");

    println!("  Ключевой момент: указатель нельзя посчитать заранее.");
    println!("  Пока future не опрошен ни разу, он ПУСТОЙ и его можно свободно");
    println!("  двигать: вернуть из функции, положить в Box, засунуть в Vec.");
    println!("  Самоссылка появляется на первом poll — ровно поэтому Pin требуется");
    println!("  начиная с poll, а конструктор future возвращает его по значению.");
}

// ---------------------------------------------------------------------------
// 3. Второй источник (в реальном коде — основной): вложенный future
//    держит ссылку на локал родителя.
// ---------------------------------------------------------------------------

/// Аналог `sock.read(&mut buf)`: лист-future, который пишет в ЧУЖОЙ буфер.
/// В настоящем коде это было бы `Read<'a> { dst: &'a mut [u8] }`.
struct WriteInto {
    dst: *mut String,
    polled: bool,
}

impl Future for WriteInto {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = unsafe { self.get_unchecked_mut() };
        if this.polled {
            unsafe { (*this.dst).push_str("payload") };
            Poll::Ready(())
        } else {
            this.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// ```ignore
/// async fn request() -> String {
///     let mut buf = String::from("[");
///     write_into(&mut buf).await;   // <-- awaitee ДЕРЖИТ &mut buf
///     buf.push(']');
///     buf
/// }
/// ```
enum ReqState {
    Unresumed,
    // buf и awaitee — соседние поля, и awaitee.dst смотрит на buf
    Suspend0 { buf: String, awaitee: WriteInto },
    Done,
}

struct Request {
    state: ReqState,
    _pin: PhantomPinned,
}

impl Future for Request {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.state {
                ReqState::Unresumed => {
                    this.state = ReqState::Suspend0 {
                        buf: String::from("["),
                        awaitee: WriteInto { dst: std::ptr::null_mut(), polled: false },
                    };
                    // и снова: адрес известен только после того, как buf уже в стейте
                    if let ReqState::Suspend0 { buf, awaitee } = &mut this.state {
                        awaitee.dst = buf as *mut String;
                        println!("    buf лежит по          : {:p}", awaitee.dst);
                        println!("    awaitee.dst указывает : внутрь того же future");
                    }
                }
                ReqState::Suspend0 { awaitee, .. } => {
                    match unsafe { Pin::new_unchecked(awaitee) }.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(()) => {
                            let ReqState::Suspend0 { mut buf, .. } =
                                std::mem::replace(&mut this.state, ReqState::Done)
                            else {
                                unreachable!()
                            };
                            buf.push(']');
                            return Poll::Ready(buf);
                        }
                    }
                }
                ReqState::Done => panic!("poll после Ready"),
            }
        }
    }
}

fn part3_awaitee_borrows_parent_local() {
    println!("\n=== 3. Самоссылка №2: вложенный future держит ссылку на локал ===\n");

    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = Box::pin(Request { state: ReqState::Unresumed, _pin: PhantomPinned });
    let out = loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            break v;
        }
    };
    println!("    результат             : {out}\n");

    println!("  Это самый частый вид самоссылки на практике:");
    println!("    let mut buf = [0u8; 1024];");
    println!("    sock.read(&mut buf).await;   // Read<'_> лежит рядом с buf и держит &mut buf");
    println!();
    println!("  В твоём FetchManual этого нет только потому, что YieldOnce");
    println!("  не берёт вообще никаких ссылок. Замени его на что-нибудь");
    println!("  вроде WriteInto — и PhantomPinned перестанет быть декорацией.");
}

// ---------------------------------------------------------------------------
// 4. Что ломается при переезде. Тот же сценарий, что part2 в pin_why.rs,
//    но на настоящем future, а не на игрушечном SelfRef.
// ---------------------------------------------------------------------------

enum CountState {
    Unresumed,
    Suspend0 { n: u64, p: *const u64, awaitee: YieldOnce },
    Done,
}

struct CountFut {
    state: CountState,
    _pin: PhantomPinned,
}

impl Future for CountFut {
    type Output = u64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u64> {
        let this = unsafe { self.get_unchecked_mut() };
        loop {
            match &mut this.state {
                CountState::Unresumed => {
                    this.state = CountState::Suspend0 {
                        n: 0xAABB_CCDD,
                        p: std::ptr::null(),
                        awaitee: YieldOnce::new(),
                    };
                    if let CountState::Suspend0 { n, p, .. } = &mut this.state {
                        *p = n as *const u64;
                        println!("    первый poll: p зафиксирован на {:p}", *p);
                    }
                }
                CountState::Suspend0 { p, awaitee, .. } => {
                    let p = *p;
                    match unsafe { Pin::new_unchecked(awaitee) }.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(()) => {
                            println!("    второй poll: читаю по {p:p}");
                            let v = unsafe { *p }; // <- вот здесь всё и решается
                            this.state = CountState::Done;
                            return Poll::Ready(v);
                        }
                    }
                }
                CountState::Done => panic!("poll после Ready"),
            }
        }
    }
}

fn part4_move_breaks_it() {
    println!("\n=== 4. Переезд между двумя poll ===\n");

    let mut cx = Context::from_waker(Waker::noop());

    println!("  a) честно: future не двигаем");
    let mut ok = Box::pin(CountFut { state: CountState::Unresumed, _pin: PhantomPinned });
    let _ = ok.as_mut().poll(&mut cx);
    let v = loop {
        if let Poll::Ready(v) = ok.as_mut().poll(&mut cx) {
            break v;
        }
    };
    println!("    получили {v:#x} — ждали {:#x}\n", 0xAABB_CCDDu64);

    println!("  b) нечестно: опрашиваем future во вложенном фрейме и возвращаем наружу");
    // первый poll происходит внутри started(): p зафиксирован на ЕЁ стек-фрейме,
    // а сам future уезжает в наш фрейм и дальше в хип
    fn started(cx: &mut Context<'_>) -> CountFut {
        let mut f = CountFut { state: CountState::Unresumed, _pin: PhantomPinned };
        let _ = unsafe { Pin::new_unchecked(&mut f) }.poll(cx);
        f
    }
    /// затаптываем освободившийся кусок стека — как это сделал бы любой
    /// следующий вызов в реальной программе
    fn smash() {
        let junk = [0xEEu64; 64];
        std::hint::black_box(&junk);
    }

    let mut moved = Box::new(started(&mut cx));
    smash();
    let v = loop {
        if let Poll::Ready(v) = unsafe { Pin::new_unchecked(&mut *moved) }.poll(&mut cx) {
            break v;
        }
    };
    println!("    получили {v:#x} — ждали {:#x}", 0xAABB_CCDDu64);
    println!();
    println!();
    println!("  Адрес тот же — но принадлежит он уже другому фрейму.");
    println!("  Это UB: читаем стек, который никому больше не принадлежит.");
    println!("  Иногда значение будет совпадать — тем хуже, баг станет плавающим.");
    println!("  И заметь: чтобы это устроить, хватило `return f` и `Box::new(..)`.");
    println!("  Обычный безопасный код. Единственное, что стоит между тобой и этим,");
    println!("  — Pin<&mut Self> в сигнатуре poll: он не отдаёт наружу &mut Self,");
    println!("  а без &mut Self нельзя ни вернуть по значению, ни забоксить, ни swap.");
}
