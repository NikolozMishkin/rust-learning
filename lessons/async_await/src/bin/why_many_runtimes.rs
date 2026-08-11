//! Что std даёт, а что обязан написать рантайм.
//!
//! Вся асинхронность в std — это КОНТРАКТ: Future, Poll, Context, Waker, Pin
//! и синтаксис async/await. Ни одной строчки, которая бы этот контракт
//! ИСПОЛНЯЛА, в std нет. Здесь мы пишем недостающее целиком — и видно,
//! что это ровно три независимых механизма, у каждого из которых
//! несколько разумных реализаций. Отсюда и разные рантаймы.
//!
//! Запуск: cargo run -p async_await --bin why_many_runtimes

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

fn main() {
    part1_what_std_gives();
    part2_mini_runtime();
    part3_where_runtimes_diverge();
}

fn part1_what_std_gives() {
    println!("\n=== 1. Инвентаризация ===\n");
    println!("  core (работает даже без ОС и без аллокатора):");
    println!("    trait Future     — что значит «незавершённая работа»");
    println!("    enum Poll        — язык ответов");
    println!("    struct Context   — конверт для Waker");
    println!("    struct Waker     — type-erased vtable: «разбуди меня»");
    println!("    struct Pin       — гарантия неподвижности");
    println!("    async / .await   — синтаксис -> стейт-машина");
    println!();
    println!("  Чего в std НЕТ:");
    println!("    executor  — кто и когда вызывает poll");
    println!("    reactor   — кто узнаёт от ОС, что IO готово, и дёргает Waker");
    println!("    timers    — sleep / timeout");
    println!("    spawn, AsyncRead/AsyncWrite, async-каналы, async-мьютексы");
    println!();
    println!("  Future — это PULL-интерфейс: poll должен кто-то вызвать.");
    println!("  Waker сделан vtable-объектом именно для того, чтобы этот");
    println!("  «кто-то» подключался снаружи. std определяет разъём, не вилку.");
}

// ---------------------------------------------------------------------------
// 2. Пишем недостающее. Всё ниже — то, что даёт tokio; только наивно.
// ---------------------------------------------------------------------------

/// Задача = future + способ вернуть себя в очередь готовых.
struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    ready: Sender<Arc<Task>>,
    name: &'static str,
}

/// Вот и весь Waker: «положи меня обратно в очередь».
/// std даёт трейт Wake и конверсию Arc<T: Wake> -> Waker. Политику — мы.
impl Wake for Task {
    fn wake(self: Arc<Self>) {
        let _ = self.ready.clone().send(self);
    }
}

/// ЧАСТЬ 1 рантайма: исполнитель. Однопоточная FIFO-очередь.
struct MiniRuntime {
    tx: Sender<Arc<Task>>,
    rx: Receiver<Arc<Task>>,
}

impl MiniRuntime {
    fn new() -> Self {
        let (tx, rx) = channel();
        MiniRuntime { tx, rx }
    }

    fn spawn(&self, name: &'static str, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            ready: self.tx.clone(),
            name,
        });
        let _ = self.tx.send(task);
    }

    fn run(self) {
        let MiniRuntime { tx, rx } = self;
        drop(tx); // иначе recv() никогда не вернёт Err и мы не выйдем
        let mut polls = 0;
        while let Ok(task) = rx.recv() {
            let mut slot = task.future.lock().unwrap();
            let Some(mut fut) = slot.take() else { continue };

            polls += 1;
            println!("    poll #{polls:<2} -> {}", task.name);

            // Waker собирается из самой задачи: разбудить = вернуть в очередь
            let waker = Waker::from(task.clone());
            let mut cx = Context::from_waker(&waker);

            match fut.as_mut().poll(&mut cx) {
                Poll::Pending => *slot = Some(fut), // ждём wake
                Poll::Ready(()) => println!("            {} завершилась", task.name),
            }
        }
        println!("    очередь пуста, всего poll: {polls}");
    }
}

/// ЧАСТЬ 2+3 рантайма: реактор и таймер. Самая тупая версия — поток на таймер.
/// Настоящий рантайм заменяет это одним epoll/kqueue/io_uring-потоком
/// плюс timer wheel. Вот здесь реализации и расходятся.
struct Sleep {
    dur: Duration,
    armed: bool,
}

fn sleep(ms: u64) -> Sleep {
    Sleep { dur: Duration::from_millis(ms), armed: false }
}

impl Future for Sleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.armed {
            return Poll::Ready(());
        }
        self.armed = true;
        let waker = cx.waker().clone();
        let dur = self.dur;
        std::thread::spawn(move || {
            std::thread::sleep(dur);
            waker.wake(); // <- «событие произошло», задача вернётся в очередь
        });
        Poll::Pending
    }
}

fn part2_mini_runtime() {
    println!("\n=== 2. Свой рантайм, ~60 строк ===\n");

    let rt = MiniRuntime::new();
    let t0 = Instant::now();

    rt.spawn("A (30ms, 30ms)", async {
        sleep(30).await;
        sleep(30).await;
    });
    rt.spawn("B (10ms x3)", async {
        for _ in 0..3 {
            sleep(10).await;
        }
    });

    rt.run();
    println!("    прошло {} ms (не 90 — задачи шли параллельно)", t0.elapsed().as_millis());
    println!();
    println!("  Это работающий рантайм. Он умеет spawn, ждёт события,");
    println!("  не жжёт CPU. Никакого tokio — только std.");
    println!("  Значит std действительно даёт всё необходимое для СОВМЕСТИМОСТИ:");
    println!("  async fn из чужой библиотеки крутится тут без единой правки.");
}

// ---------------------------------------------------------------------------
// 3. Почему их много
// ---------------------------------------------------------------------------
fn part3_where_runtimes_diverge() {
    println!("\n=== 3. Три места, где начинаются расхождения ===\n");

    println!("  a) ПЛАНИРОВЩИК");
    println!("     tokio         work-stealing по потокам -> задачи должны быть Send");
    println!("     monoio        thread-per-core, задача не покидает поток -> Send НЕ нужен");
    println!("     embassy       статичный набор задач, без аллокатора вообще");
    println!("     Это несовместимые API, а не разное качество: `spawn` в одном");
    println!("     требует Send + 'static, в другом принимает !Send. Унифицировать");
    println!("     нельзя — придётся выбрать проигравшего.");
    println!();

    println!("  b) РЕАКТОР");
    println!("     epoll/kqueue/IOCP  readiness: «сокет готов» -> ты сам делаешь read");
    println!("     io_uring           completion: «read уже выполнен»");
    println!("     Разница протекает в типы: completion-модели нужен буфер во");
    println!("     владении ядра, поэтому у monoio read(buf) -> (Result, buf),");
    println!("     а не read(&mut buf). Другая сигнатура — другой AsyncRead.");
    println!("     embassy вместо ОС вообще имеет прерывания МК.");
    println!();

    println!("  c) ЦЕНА СТАБИЛЬНОСТИ");
    println!("     std стабилен навсегда. Future стабилизировали в 1.36 — минимум,");
    println!("     в котором были уверены. io_uring тогда только появился, и");
    println!("     «очевидный» AsyncRead из futures 0.1 сегодня оказался бы");
    println!("     неподходящим для completion-моделей. Не положили — и повезло.");
    println!();

    println!("  И честная обратная сторона: это стоит экосистеме расколом.");
    println!("  AsyncRead/AsyncWrite нет в std -> у tokio и futures-rs они РАЗНЫЕ,");
    println!("  и библиотеки пишут `features = [\"tokio\"]`. Это реальная цена,");
    println!("  а не только достоинство дизайна.");
    println!();
    println!("  Итог: в std лежит ровно то, что обязано быть общим, чтобы");
    println!("  `async fn` в чужом крейте компилировался у всех. Всё, где есть");
    println!("  осмысленный выбор — политика планирования, модель IO, наличие ОС —");
    println!("  вынесено в библиотеки, потому что единственно верного ответа нет.");
}
