//! Где физически лежит стейт стейт-машины у future из async-замыкания.
//!
//! Запуск: cargo run -p async_await --bin state_machine_layout

use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

/// Минимальная точка приостановки: первый poll -> Pending, второй -> Ready.
/// Нужна, чтобы увидеть промежуточное состояние стейт-машины.
struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn printing() {
    println!("      [printing] I am async function");
}

/// Печатает сырые байты значения с разбивкой по 8.
fn dump<T: ?Sized>(label: &str, v: &T) {
    let n = size_of_val(v);
    let ptr = v as *const T as *const u8;
    let bytes = unsafe { std::slice::from_raw_parts(ptr, n) };
    print!("{label:<26} @ {ptr:p} [{n:>2}] ");
    for (i, b) in bytes.iter().enumerate() {
        if i != 0 && i % 8 == 0 {
            print!("| ");
        }
        print!("{b:02x} ");
    }
    println!();
}

fn poll_once<F: Future>(fut: &mut Pin<&mut F>) -> Poll<F::Output> {
    let mut cx = Context::from_waker(Waker::noop());
    fut.as_mut().poll(&mut cx)
}

fn main() {
    part1_why_16();
    part2_where_the_state_lives();
    part3_what_actually_changes_the_size();
}

// ---------------------------------------------------------------------------
// 1. Почему ровно 16 байт
// ---------------------------------------------------------------------------
fn part1_why_16() {
    println!("=== 1. Почему future из async-замыкания = 16 байт ===\n");

    let my_str = "Hello, World!".to_string();

    let my_clogure = async || {
        let x = printing(); // future, живёт ЧЕРЕЗ await -> в стейт-машине
        println!("      [closure] The future has not been polled yet");
        x.await;
        let my_str = my_str.clone(); // создаётся ПОСЛЕ await -> НЕ в стейт-машине
        println!("      [closure] {my_str}");
    };

    let fut = my_clogure();

    println!("String                       = {:>3} байт", size_of::<String>());
    println!("&String                      = {:>3} байт", size_of::<&String>());
    println!("printing() future            = {:>3} байт", size_of_val(&printing()));
    println!("my_clogure (само замыкание)  = {:>3} байт", size_of_val(&my_clogure));
    println!("my_clogure() future          = {:>3} байт  <-- вот эти 16", size_of_val(&fut));
    println!();
    dump("байты future", &fut);
    println!("&my_str                      = {:p}", &my_str);
    println!();
    println!("  offset 0..8   &my_str            8  указатель на ОДИН захват");
    println!("  offset 8      x: printing()      1  future, живущий через await");
    println!("  offset 9      дискриминант       1  номер состояния");
    println!("  offset 10..16 padding            6  выравнивание до align = 8");
    println!("                                  --");
    println!("                                  16\n");

    println!("Локальный `my_str` (String, 24 байта) создаётся ПОСЛЕ await,");
    println!("поэтому он живёт в обычном стековом фрейме poll(), а не в стейт-машине.\n");
}

// ---------------------------------------------------------------------------
// 2. Наглядно: смотрим на сырые байты стейт-машины
// ---------------------------------------------------------------------------
fn part2_where_the_state_lives() {
    println!("=== 2. Где физически сидит стейт ===\n");

    let my_str = "Hello, World!".to_string();

    let my_clogure = async || {
        let marker: u64 = 0x1122_3344_5566_7788; // живёт ЧЕРЕЗ await -> будет виден в байтах
        YieldOnce(false).await;
        println!("      [closure] marker = {marker:#x}, captured = {my_str}");
    };

    println!("&my_str     = {:p}   <-- сюда будет указывать future", &my_str);
    println!("&my_clogure = {:p}", &my_clogure);
    dump("окружение замыкания", &my_clogure);
    println!("  ^ само замыкание = 8 байт = один указатель на my_str\n");

    let mut fut = pin!(my_clogure());
    println!("size_of_val(future) = {}\n", size_of_val(&*fut));

    dump("state 0: создан", &*fut);
    println!("poll #1 -> {:?}", poll_once(&mut fut));
    dump("state 3: приостановлен", &*fut);
    println!("poll #2 -> {:?}", poll_once(&mut fut));
    dump("state 1: завершён", &*fut);

    println!();
    println!("Читаем байты (little-endian):");
    println!("  [ 0.. 8]  88 77 66 55 44 33 22 11  = marker 0x1122334455667788");
    println!("            до первого poll там мусор: локальная переменная ещё не создана");
    println!("  [ 8..16]  адрес my_str             = указатель на ЗАХВАЧЕННУЮ ПЕРЕМЕННУЮ");
    println!("            (сравни со строкой `&my_str` выше — те же байты; это НЕ адрес");
    println!("             замыкания: future одалживает каждый захват по отдельности)");
    println!("  [16]      поле YieldOnce.0         = мусор -> 01 (вложенный future тоже внутри!)");
    println!("  [17]      ДИСКРИМИНАНТ состояния   = 00 -> 03 -> 01");
    println!("            0 = ещё не запускался, 1 = завершён, 2 = запаниковал, 3+ = точки await");
    println!();
    println!("Итого: стейт-машина — это обычный enum на стеке (или в Box, если Box::pin).");
    println!("В ней лежат ТОЛЬКО переменные, живущие через await, вложенные future");
    println!("и один байт-дискриминант. Сами захваченные данные лежат СНАРУЖИ.\n");
}

// ---------------------------------------------------------------------------
// 3. Что реально меняет размер
// ---------------------------------------------------------------------------
fn part3_what_actually_changes_the_size() {
    println!("=== 3. Почему размер «всегда 16» и что его меняет ===\n");

    // (а) захватываем БОЛЬШЕ ДАННЫХ, но столько же переменных -> future не растёт
    let one_small = "x".to_string();
    let c_small = async || {
        printing().await;
        println!("{}", one_small.len());
    };
    let one_huge = vec![0u8; 4096];
    let c_huge = async || {
        printing().await;
        println!("{}", one_huge.len());
    };
    println!("1 захват, String (24 B):    closure = {:>4}, future = {:>4}", size_of_val(&c_small), size_of_val(&c_small()));
    println!("1 захват, Vec на 4 KiB:     closure = {:>4}, future = {:>4}  <-- те же 16", size_of_val(&c_huge), size_of_val(&c_huge()));

    // (б) РАСТЁТ от количества захваченных переменных: по указателю на каждую
    let (x1, x2, x3) = ("a".to_string(), "b".to_string(), "c".to_string());
    let c_three = async || {
        printing().await;
        println!("{x1} {x2} {x3}");
    };
    println!("3 захвата:                  closure = {:>4}, future = {:>4}  <-- 3 указателя", size_of_val(&c_three), size_of_val(&c_three()));

    // (б) большой локал ПОСЛЕ await -> не в стейт-машине
    let after = async || {
        printing().await;
        let arr = [7u8; 1024];
        println!("{}", arr[0]);
    };
    println!("локал 1 KiB ПОСЛЕ await:                     future = {:>4}", size_of_val(&after()));

    // (в) тот же локал ЧЕРЕЗ await -> внутри стейт-машины
    let across = async || {
        let arr = [7u8; 1024];
        printing().await;
        println!("{}", arr[0]);
    };
    println!("локал 1 KiB ЧЕРЕЗ await:                     future = {:>4}  <-- вот это растит", size_of_val(&across()));

    // (г) async move блок против async замыкания: блок ВЛАДЕЕТ захватами
    let s = "Hello, World!".to_string();
    let block = async move {
        printing().await;
        println!("{s}");
    };
    println!("async move БЛОК с захватом String:           future = {:>4}  (владеет, а не одалживает)", size_of_val(&block));

    println!();
    println!("Ключ: async-замыкание — lending closure (AsyncFnMut).");
    println!("Возвращаемый future ОДАЛЖИВАЕТ захваты — по 8 байт на переменную,");
    println!("независимо от того, сколько данных в ней лежит. У тебя захват один (my_str),");
    println!("отсюда и «всегда 16»: 8 (указатель) + 1 (вложенный future) + 1 (дискриминант) + padding.");
    println!("Размер меняется от числа захватов и от того, что живёт через точки await.");
}
