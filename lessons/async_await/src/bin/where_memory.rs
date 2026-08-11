//! Что лежит в стеке, а что в хипе, и сколько именно аллокаций делает tokio.
//!
//! Запуск: cargo run -p async_await --bin where_memory

use std::alloc::{GlobalAlloc, Layout, System};
use std::future::Future;
use std::pin::pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};

// ---------------------------------------------------------------------------
// Аллокатор-счётчик: включается на время интересующего нас участка
// ---------------------------------------------------------------------------
static WATCH: AtomicBool = AtomicBool::new(false);
static COUNT: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);
static BIGGEST: AtomicUsize = AtomicUsize::new(0);

/// журнал размеров: чтобы видеть каждую аллокацию, а не только самую большую
const LOG_CAP: usize = 16;
static LOG: [AtomicUsize; LOG_CAP] = [const { AtomicUsize::new(0) }; LOG_CAP];
static LOG_ALIGN: [AtomicUsize; LOG_CAP] = [const { AtomicUsize::new(0) }; LOG_CAP];

fn log_sizes() -> Vec<usize> {
    let n = COUNT.load(Relaxed).min(LOG_CAP);
    (0..n).map(|i| LOG[i].load(Relaxed)).collect()
}

fn log_aligns() -> Vec<usize> {
    let n = COUNT.load(Relaxed).min(LOG_CAP);
    (0..n).map(|i| LOG_ALIGN[i].load(Relaxed)).collect()
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if WATCH.load(Relaxed) {
            let i = COUNT.fetch_add(1, Relaxed);
            if i < LOG_CAP {
                LOG[i].store(l.size(), Relaxed);
                LOG_ALIGN[i].store(l.align(), Relaxed);
            }
            BYTES.fetch_add(l.size(), Relaxed);
            BIGGEST.fetch_max(l.size(), Relaxed);
        }
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static A: Counting = Counting;

/// Замеряет аллокации внутри f.
fn measure<R>(label: &str, f: impl FnOnce() -> R) -> R {
    COUNT.store(0, Relaxed);
    BYTES.store(0, Relaxed);
    BIGGEST.store(0, Relaxed);
    WATCH.store(true, Relaxed);
    let r = f();
    WATCH.store(false, Relaxed);
    println!(
        "  {label:<46} аллокаций: {:>2}   всего: {:>5} B   макс.блок: {:>5} B",
        COUNT.load(Relaxed),
        BYTES.load(Relaxed),
        BIGGEST.load(Relaxed)
    );
    r
}

// ---------------------------------------------------------------------------
// Подопытный future известного размера
// ---------------------------------------------------------------------------
async fn nop() {}

/// ~1 KiB держим через await -> ровно столько уедет в стейт-машину
async fn fat() -> u8 {
    let buf = [7u8; 1024];
    nop().await;
    buf[0]
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    println!("\n=== размеры участников ===\n");
    println!("  size_of_val(fat())                             {:>6} B", size_of_val(&fat()));
    println!("  size_of::<JoinHandle<u8>>()                    {:>6} B", size_of::<tokio::task::JoinHandle<u8>>());

    println!("\n=== A. Где живёт сам future ===\n");

    measure("let f = fat();            (создание значения)", || {
        let f = fat();
        std::hint::black_box(&f);
    });
    println!("    ^ ноль аллокаций: future — это ЗНАЧЕНИЕ. Оно родилось в стеке вызывающего.");

    measure("pin!(fat())               (закрепление в стеке)", || {
        let f = pin!(fat());
        std::hint::black_box(&f);
    });
    println!("    ^ тоже ноль: pin! — это просто let + запрет двигать. Тот же стековый слот.");

    measure("Box::pin(fat())           (переезд в хип)", || {
        let f = Box::pin(fat());
        std::hint::black_box(&f);
    });
    println!("    ^ одна аллокация ровно на размер future. В стеке остался указатель (8 B).");

    println!("\n=== B. block_on против spawn ===\n");

    // разогрев: первый block_on поднимает драйверы рантайма и аллоцирует своё
    rt.block_on(nop());

    measure("rt.block_on(fat())        (рантайм разогрет)", || rt.block_on(fat()));
    println!("    ^ future живёт в стековом фрейме block_on. Задачи нет — аллокаций нет.");

    measure("rt.block_on(async { spawn(fat()).await })", || {
        rt.block_on(async {
            let h = tokio::spawn(fat());
            h.await.unwrap()
        })
    });
    println!("    ^ вот она, единственная аллокация задачи: заголовок + future в ОДНОМ блоке.");
    println!("      spawn требует 'static именно потому, что future переезжает в этот блок");
    println!("      и живёт независимо от твоего стекового фрейма.");

    // калибровка накладных расходов задачи: future известного размера -> размер блока
    async fn sized<const N: usize>() {
        let buf = [0u8; N];
        nop().await;
        std::hint::black_box(&buf);
    }
    println!();
    let mut calib = |label: &str, fut_size: usize, run: &mut dyn FnMut()| {
        COUNT.store(0, Relaxed);
        WATCH.store(true, Relaxed);
        run();
        WATCH.store(false, Relaxed);
        let sizes = log_sizes();
        println!(
            "  {label:<22} future {fut_size:>5} B -> аллокации {sizes:?}   накладные {} B",
            sizes.iter().copied().max().unwrap_or(0) as isize - fut_size as isize
        );
    };
    calib("spawn(nop())", size_of_val(&nop()), &mut || {
        rt.block_on(async { tokio::spawn(nop()).await.unwrap() })
    });
    calib("spawn(sized::<64>)", size_of_val(&sized::<64>()), &mut || {
        rt.block_on(async { tokio::spawn(sized::<64>()).await.unwrap() })
    });
    calib("spawn(sized::<512>)", size_of_val(&sized::<512>()), &mut || {
        rt.block_on(async { tokio::spawn(sized::<512>()).await.unwrap() })
    });
    calib("spawn(sized::<4096>)", size_of_val(&sized::<4096>()), &mut || {
        rt.block_on(async { tokio::spawn(sized::<4096>()).await.unwrap() })
    });
    println!("    ^ одна аллокация на задачу: Header + твой future + Trailer подряд.");
    println!(
        "      BOX_FUTURE_THRESHOLD = {} B в этой сборке: сверх него tokio сам делает",
        if cfg!(debug_assertions) { 2048 } else { 16384 }
    );
    println!("      Box::pin ДО создания задачи — отсюда вторая аллокация у 4 KiB future.");

    println!("\n=== B2. Размер блока задачи как функция от размера future ===\n");
    macro_rules! sweep {
        ($($n:literal),*) => {$({
            let fut_size = size_of_val(&sized::<$n>());
            COUNT.store(0, Relaxed);
            WATCH.store(true, Relaxed);
            rt.block_on(async { tokio::spawn(sized::<$n>()).await.unwrap() });
            WATCH.store(false, Relaxed);
            let block = log_sizes().into_iter().max().unwrap_or(0);
            let align = log_aligns().into_iter().max().unwrap_or(0);
            println!(
                "  future {fut_size:>5} B -> блок {block:>5} B   ALIGN {align:>4}   накладные {:>4} B",
                block as isize - fut_size as isize
            );
        })*};
    }
    sweep!(0, 8, 24, 56, 64, 120, 128, 248, 504, 1016);
    println!();
    println!("  Все блоки кратны 128 и выравнены на 128 — см. ALIGN выше.");
    println!("  Причина в исходнике tokio (runtime/task/core.rs): struct Cell помечен");
    println!("  repr(align(128)) на x86_64/aarch64/powerpc64 — защита от false sharing,");
    println!("  ведь Header с атомарным состоянием дёргают сразу несколько потоков.");
    println!();
    println!("  struct Cell {{ header: Header, core: Core<F, S>, trailer: Trailer }},");
    println!("  где Core держит enum Stage {{ Running(F), Finished(Result<Output>), Consumed }}.");
    println!("  Итого: блок = round_up(Header + max(future, результат) + Trailer, 128).");
    println!("  Отсюда пол в 128 байт на задачу, даже если твой future — 1 байт.");

    println!("\n=== C. Waker: клонирование — не аллокация ===\n");

    measure("100 x waker.clone()", || {
        rt.block_on(async {
            let w = std::task::Waker::noop().clone();
            for _ in 0..100 {
                std::hint::black_box(w.clone());
            }
        })
    });
    println!("    ^ Waker = 2 указателя (data + vtable). clone у tokio — это ++ refcount");
    println!("      на тот же Arc задачи. Разбудить задачу = положить её в очередь рантайма.");

    println!("\n=== D. Сколько глубины стека съедает await ===\n");
    rt.block_on(async {
        deep_stack_probe().await;
    });
}

// ---------------------------------------------------------------------------
// D. Await не растит стек: вложенные poll — да, но между poll стек пуст
// ---------------------------------------------------------------------------
async fn deep_stack_probe() {
    /// Текущая вершина стека. Обычная функция, поэтому её локал точно в стеке,
    /// а не в стейт-машине.
    #[inline(never)]
    fn stack_here() -> usize {
        let probe = 0u8;
        std::hint::black_box(&probe) as *const u8 as usize
    }

    /// Лист, который сообщает глубину стека в момент своего poll.
    struct Report {
        depth: u32,
        base: usize,
        done: bool,
    }
    impl Future for Report {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            let used = self.base - stack_here();
            if self.done {
                println!("    poll #2 (после пробуждения): стек ушёл на {used:>5} B");
                std::task::Poll::Ready(())
            } else {
                println!("    poll #1 на глубине {} await: стек ушёл на {used:>5} B", self.depth);
                self.done = true;
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }

    // цепочка вложенных async fn, ведущая к листу
    fn level(n: u32, depth: u32, base: usize) -> std::pin::Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async move {
            if n == 0 {
                Report { depth, base, done: false }.await;
            } else {
                level(n - 1, depth, base).await;
            }
        })
    }

    let base = stack_here();
    println!("    точка входа: {base:#x}\n");
    Report { depth: 0, base, done: false }.await;
    level(8, 8, base).await;
    println!();
    println!("    Стек расходуется только ВНУТРИ одного прохода poll: цепочка poll->poll->poll");
    println!("    от корня задачи до листа. Как только лист вернул Pending, ВЕСЬ этот стек");
    println!("    разматывается — состояние-то уже сохранено в стейт-машине.");
    println!("    Поэтому 100k задач у tokio стоят 100k стейт-машин в хипе,");
    println!("    а не 100k стеков по 2 MiB, как было бы с потоками.");
}
