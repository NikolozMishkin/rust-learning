//! Сколько байт занимает future и от чего это зависит.
//!
//! Запуск: cargo run -p async_await --bin future_sizes

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

async fn nop() {}

fn row(label: &str, v: usize) {
    println!("  {label:<52} {v:>6}");
}

fn frow<F: Future>(label: &str, f: F) {
    println!(
        "  {label:<52} {:>6}   (align {})",
        size_of_val(&f),
        align_of_val(&f)
    );
}

fn main() {
    part_a_minimum();
    part_b_what_grows();
    part_c_nesting_sums();
    part_d_runtime_types();
}

// ---------------------------------------------------------------------------
// A. Минимум
// ---------------------------------------------------------------------------
fn part_a_minimum() {
    println!("\n=== A. Минимальный размер future ===\n");

    frow("async {} (пустой блок, нет await)", async {});
    frow("async { 1 + 1 } (значение не переживает await)", async { 1 + 1 });
    frow("async fn nop() -> ()", nop());
    frow("async { nop().await }  (одна точка await)", async {
        nop().await
    });
    frow("async { nop().await; nop().await } (две точки)", async {
        nop().await;
        nop().await;
    });
    println!();
    row("std::future::ready(())", size_of_val(&std::future::ready(())));
    row("std::future::pending::<()>()", size_of_val(&std::future::pending::<()>()));
    row("std::future::ready(0u64)", size_of_val(&std::future::ready(0u64)));
    println!();
    println!("  Ниже 1 байта нельзя: у coroutine всегда есть дискриминант,");
    println!("  и у него минимум 3 варианта — Unresumed / Returned / Panicked.");
    println!("  ZST (0 байт) не выйдет: 3 состояния не влезают в 0 бит.");
}

// ---------------------------------------------------------------------------
// B. Что растит размер
// ---------------------------------------------------------------------------
fn part_b_what_grows() {
    println!("\n=== B. Что именно попадает внутрь ===\n");

    frow("локал [u8; 1024] создан ПОСЛЕ await", async {
        nop().await;
        let a = [7u8; 1024];
        std::hint::black_box(&a);
    });
    frow("локал [u8; 1024] живёт ЧЕРЕЗ await", async {
        let a = [7u8; 1024];
        nop().await;
        std::hint::black_box(&a);
    });
    // не-Copy, чтобы drop реально забирал значение
    struct Buf([u8; 1024]);
    frow("буфер 1 KiB задропан ДО await, адрес НЕ утёк", async {
        let a = Buf([7u8; 1024]);
        drop(a);
        nop().await;
    });
    frow("тот же drop, но адрес утёк в black_box(&a)", async {
        let a = Buf([7u8; 1024]);
        std::hint::black_box(&a);
        drop(a);
        nop().await;
    });
    println!();
    frow("два [u8;1024] в РАЗНЫХ, непересекающихся диапазонах", async {
        {
            let a = [7u8; 1024];
            std::hint::black_box(&a);
            nop().await;
        }
        {
            let b = [9u8; 1024];
            std::hint::black_box(&b);
            nop().await;
        }
    });
    frow("два [u8;1024] ОБА живут через один await", async {
        let a = [7u8; 1024];
        let b = [9u8; 1024];
        nop().await;
        std::hint::black_box((&a, &b));
    });
    println!();
    println!("  Вывод 1: размер = max по состояниям, а не сумма всех локалов.");
    println!("  Слоты переменных, чьи времена жизни не пересекаются, переиспользуются.");
    println!("  Вывод 2: анализ живости КОНСЕРВАТИВЕН. Стоит адресу переменной утечь");
    println!("  наружу — компилятор оставляет ей слот в стейт-машине даже там, где она");
    println!("  уже мертва. Отсюда «future раздулся, а я ничего не держал через await».");
}

// ---------------------------------------------------------------------------
// C. Вложенность складывается
// ---------------------------------------------------------------------------
fn part_c_nesting_sums() {
    println!("\n=== C. Вложенные await складываются (та самая проблема раздутия) ===\n");

    async fn leaf() {
        let buf = [0u8; 1024];
        nop().await;
        std::hint::black_box(&buf);
    }
    async fn level1() {
        leaf().await;
    }
    async fn level2() {
        level1().await;
    }
    async fn level3() {
        level2().await;
    }
    async fn two_sequential_leaves() {
        leaf().await;
        leaf().await;
    }
    async fn two_concurrent_leaves() {
        tokio::join!(leaf(), leaf());
    }

    frow("leaf()   (1 KiB через await)", leaf());
    frow("level1() = await leaf()", level1());
    frow("level2() = await level1()", level2());
    frow("level3() = await level2()", level3());
    frow("await leaf(); await leaf();  (последовательно)", two_sequential_leaves());
    frow("join!(leaf(), leaf())        (одновременно)", two_concurrent_leaves());
    println!();
    println!("  Вложенный future — обычное ПОЛЕ внутри внешнего. Никакой косвенности.");
    println!("  Пока дочерний future жив через await, его байты сидят внутри родителя:");
    println!("  один Box::pin на входе в задачу держит ВЕСЬ стек вызовов целиком.");
    println!("  Последовательные await переиспользуют слот, join! — нет (оба живы разом).");

    async fn boxed_leaf() {
        let f: Pin<Box<dyn Future<Output = ()>>> = Box::pin(leaf());
        f.await;
    }
    frow("Box::pin(leaf()).await  (разрыв косвенностью)", boxed_leaf());
    println!("  ^ так режут раздувание: в родителе остаётся только толстый указатель.");
}

// ---------------------------------------------------------------------------
// D. Типы рантайма
// ---------------------------------------------------------------------------
fn part_d_runtime_types() {
    println!("\n=== D. Типы вокруг poll ===\n");

    row("Poll<()>", size_of::<Poll<()>>());
    row("Poll<u64>", size_of::<Poll<u64>>());
    row("Poll<Box<u8>> (ниша: Pending = нулевой указатель)", size_of::<Poll<Box<u8>>>());
    row("Waker", size_of::<Waker>());
    row("&mut Context<'_>", size_of::<&mut Context<'_>>());
    row("Context<'_>", size_of::<Context<'_>>());
    println!();
    row("&mut MyFuture (тонкий указатель)", size_of::<&mut nopFutAlias>());
    row("Pin<&mut MyFuture> (столько же — Pin прозрачен!)", size_of::<Pin<&mut nopFutAlias>>());
    row("Box<dyn Future<Output=()>> (толстый)", size_of::<Box<dyn Future<Output = ()>>>());
    row("Pin<Box<dyn Future<Output=()>>>", size_of::<Pin<Box<dyn Future<Output = ()>>>>());
    println!();
    row("tokio::time::Sleep", size_of::<tokio::time::Sleep>());
    row("tokio::task::JoinHandle<()>", size_of::<tokio::task::JoinHandle<()>>());
    row("tokio::sync::Mutex<u64>", size_of::<tokio::sync::Mutex<u64>>());
    println!();
    println!("  Pin<P> — это #[repr(transparent)] обёртка над P. Ноль рантайм-стоимости,");
    println!("  чистое обещание системе типов: «на это место больше не двигают».");
}

// заглушка, чтобы взять размер конкретного анонимного типа
#[allow(non_camel_case_types)]
type nopFutAlias = std::future::Ready<()>;
