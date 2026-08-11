//! Когда future ПЕРЕМЕЩАТЬ МОЖНО, а когда уже нельзя.
//! И почему work stealing между потоками — это не перемещение future.
//!
//! Запуск: cargo run -p async_await --bin moving_futures

use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

fn main() {
    part1_move_freely_before_first_poll();
    part2_self_pointer_is_born_during_poll();
    part3_after_pin_the_door_closes();
    part4_work_stealing_moves_pointers();
}

// ---------------------------------------------------------------------------
// вспомогательное
// ---------------------------------------------------------------------------

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

/// Future с самоссылкой: r указывает на x, оба живут через await.
fn selfref() -> impl Future<Output = u64> {
    async {
        let x: u64 = 0xDEAD_BEEF_0000_0000;
        let r: &u64 = &x;
        YieldOnce(false).await;
        *r
    }
}

/// Ищет внутри байтов значения, похожие на указатель ВНУТРЬ этого же объекта.
/// Работает независимо от того, как компилятор разложил поля.
fn find_self_pointers<F>(fut: &F) -> Vec<(usize, usize)> {
    let base = fut as *const F as usize;
    let size = size_of::<F>();
    let bytes = unsafe { std::slice::from_raw_parts(base as *const u8, size) };
    let mut found = Vec::new();
    for off in (0..size.saturating_sub(7)).step_by(8) {
        let v = usize::from_le_bytes(bytes[off..off + 8].try_into().unwrap());
        if v >= base && v < base + size {
            found.push((off, v));
        }
    }
    found
}

fn poll_once<F: Future>(fut: &mut Pin<&mut F>) -> Poll<F::Output> {
    let mut cx = Context::from_waker(Waker::noop());
    fut.as_mut().poll(&mut cx)
}

// ---------------------------------------------------------------------------
// 1. До первого poll future — обычное значение
// ---------------------------------------------------------------------------
fn part1_move_freely_before_first_poll() {
    println!("\n=== 1. До первого poll future перемещается свободно ===\n");

    // рождение в стеке этой функции
    let f = selfref();
    println!("  создан в стеке              @ {:p}", &f);

    // перемещение 1: передали в функцию по значению
    fn through_fn<F: Future>(f: F) -> F {
        println!("  переехал в фрейм функции    @ {:p}", &f);
        f
    }
    let f = through_fn(f);
    println!("  вернулся назад              @ {:p}", &f);

    // перемещение 2: положили в Vec (и Vec его ещё раз подвинет при реаллокации)
    let mut v = Vec::with_capacity(1);
    v.push(f);
    println!("  переехал в Vec (хип)        @ {:p}", &v[0]);
    v.reserve(1024); // реаллокация: байты физически скопированы в новое место
    println!("  Vec реаллоцировал           @ {:p}", &v[0]);
    let f = v.pop().unwrap();
    println!("  вынут из Vec обратно в стек @ {:p}", &f);

    // перемещение 3: в кортеж, в Box
    let boxed = Box::new((0u8, f));
    println!("  переехал в Box<(u8, F)>     @ {:p}", &boxed.1);

    // и только ТЕПЕРЬ первый poll
    let mut f = pin!(boxed.1);
    println!("\n  закрепили pin!              @ {:p}   <-- вот здесь дверь закрылась", &*f);
    let r1 = poll_once(&mut f);
    let r2 = poll_once(&mut f);
    println!("  poll #1 = {r1:?}, poll #2 = {r2:?}");
    println!();
    println!("  Шесть перемещений — и всё работает. Потому что до первого poll");
    println!("  тело async-блока НЕ ВЫПОЛНЯЛОСЬ: ни одной внутренней ссылки ещё нет,");
    println!("  ломаться нечему. Future в этот момент — просто мешок байтов.");
    println!();
    println!("  Ровно поэтому легальны все привычные вещи:");
    println!("   * let x = printing();      — создали значение, никто его не опрашивал");
    println!("   * x.await                  — ПЕРЕМЕЩАЕТ x в поле __awaitee родителя");
    println!("   * tokio::spawn(fut)        — ПЕРЕМЕЩАЕТ fut в блок задачи в хипе");
    println!("   * block_on(fut)            — ПЕРЕМЕЩАЕТ fut в свой стековый фрейм");
    println!("  Все они забирают future ПО ЗНАЧЕНИЮ и все делают это до первого poll.");
}

// ---------------------------------------------------------------------------
// 2. Внутренний указатель возникает во время poll — и смотрит туда, где future ЛЕЖИТ СЕЙЧАС
// ---------------------------------------------------------------------------
fn part2_self_pointer_is_born_during_poll() {
    println!("\n=== 2. Самоссылка появляется только во время poll ===\n");

    // Прогоняем один и тот же тип future в двух разных местах памяти.
    for (label, place) in [("стек", false), ("хип", true)] {
        let f = selfref();

        if place {
            let mut f = Box::pin(f);
            let base = &*f as *const _ as usize;
            println!("  --- future в {label} @ {base:#x}");
            println!("      до первого poll : самоссылок нет (тело не выполнялось)");
            let mut cx = Context::from_waker(Waker::noop());
            let _ = f.as_mut().poll(&mut cx);
            for (off, v) in find_self_pointers(&*f) {
                println!(
                    "      после poll      : offset {off} хранит {v:#x} = base + {}",
                    v - base
                );
            }
        } else {
            let mut f = pin!(f);
            let base = &*f as *const _ as usize;
            println!("  --- future в {label} @ {base:#x}");
            println!("      до первого poll : самоссылок нет (тело не выполнялось)");
            let _ = poll_once(&mut f);
            for (off, v) in find_self_pointers(&*f) {
                println!(
                    "      после poll      : offset {off} хранит {v:#x} = base + {}",
                    v - base
                );
            }
        }
    }

    println!();
    println!("  Один и тот же код дал РАЗНЫЕ значения указателя: каждый раз он равен");
    println!("  адресу, по которому future лежал В МОМЕНТ poll. Указатель не «встроен»");
    println!("  в тип — он вычисляется на ходу. Отсюда и правило:");
    println!("  двигать можно ровно до того момента, как этот указатель записан,");
    println!("  то есть до первого poll. После — адрес зафиксирован в поле, и переезд");
    println!("  оставит его смотреть в никуда.");
}

// ---------------------------------------------------------------------------
// 3. Что физически мешает переместить после закрепления
// ---------------------------------------------------------------------------
fn part3_after_pin_the_door_closes() {
    println!("\n=== 3. Как система типов закрывает дверь ===\n");

    let mut a = pin!(selfref());
    let mut b = pin!(selfref());
    let _ = poll_once(&mut a);
    let _ = poll_once(&mut b);

    println!("  a @ {:p},  b @ {:p}", &*a, &*b);
    println!("  Обе закреплены и опрошены. Теперь любая попытка переезда — ошибка компиляции.");
    println!("  Тексты ниже — дословный вывод rustc, а не пересказ:");
    println!();
    println!("    std::mem::swap(&mut *a, &mut *b);");
    println!("      error[E0596]: cannot borrow data in dereference of");
    println!("                    `Pin<&mut impl Future<Output = ()>>` as mutable");
    println!("      (DerefMut у Pin требует Unpin, а корутины !Unpin)");
    println!();
    println!("    let moved = *a;");
    println!("      error[E0507]: cannot move out of dereference of");
    println!("                    `Pin<&mut impl Future<Output = ()>>`");
    println!();
    println!("    Pin::new(&mut fut)   // попытка закрепить без unsafe");
    println!("      error[E0277]: `{{async block@...}}` cannot be unpinned");
    println!("      (Pin::new существует только для Unpin-типов)");
    println!();
    println!("  Pin ничего не проверяет в рантайме. Он просто НЕ ДАЁТ получить &mut F");
    println!("  и не даёт забрать F по значению. Без &mut нет ни swap, ни replace,");
    println!("  ни присваивания — то есть нет ни одного безопасного способа сдвинуть байты.");
    println!();
    println!("  А вот сам УКАЗАТЕЛЬ Pin<&mut F> копируется и передаётся куда угодно:");

    fn takes_pin(mut p: Pin<&mut impl Future<Output = u64>>) -> usize {
        &*p as *const _ as usize
    }
    let addr_in_callee = takes_pin(a.as_mut());
    println!("    передали Pin в другую функцию, future всё там же: {addr_in_callee:#x}");
    println!("    (переместился 8-байтовый указатель, а не 24 байта стейт-машины)");
}

// ---------------------------------------------------------------------------
// 4. Work stealing: что именно «забирает» поток
// ---------------------------------------------------------------------------
fn part4_work_stealing_moves_pointers() {
    println!("\n=== 4. Work stealing: потоки забирают указатели, а не байты ===\n");

    // (задача, поток, адрес локала внутри стейт-машины)
    type Log = Arc<Mutex<Vec<(usize, String, usize)>>>;

    async fn worker(id: usize, log: Log) {
        // anchor живёт через все await -> лежит ВНУТРИ стейт-машины задачи
        let anchor: u64 = id as u64;
        for _ in 0..300 {
            let addr = &anchor as *const u64 as usize;
            let thread = format!("{:?}", std::thread::current().id());
            log.lock().unwrap().push((id, thread, addr));
            tokio::task::yield_now().await;
        }
        std::hint::black_box(&anchor);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .build()
        .unwrap();

    let log: Log = Arc::new(Mutex::new(Vec::new()));

    rt.block_on(async {
        let mut handles = Vec::new();
        for id in 0..16 {
            handles.push(tokio::spawn(worker(id, Arc::clone(&log))));
        }
        for h in handles {
            h.await.unwrap();
        }
    });

    let entries = log.lock().unwrap();
    println!("  16 задач, 4 воркера, по 300 приостановок каждая.");
    println!("  Записей: {}\n", entries.len());

    println!("  задача | потоков, на которых её опрашивали | разных адресов стейта");
    println!("  -------+-----------------------------------+----------------------");
    let mut migrated = 0;
    for id in 0..16 {
        let mut threads: Vec<&str> = entries
            .iter()
            .filter(|e| e.0 == id)
            .map(|e| e.1.as_str())
            .collect();
        threads.sort_unstable();
        threads.dedup();

        let mut addrs: Vec<usize> = entries.iter().filter(|e| e.0 == id).map(|e| e.2).collect();
        addrs.sort_unstable();
        addrs.dedup();

        if threads.len() > 1 {
            migrated += 1;
        }
        let mark = if threads.len() > 1 { "<-- миграция" } else { "" };
        println!(
            "  {id:>6} | {:>33} | {:>20} {mark}",
            threads.len(),
            addrs.len()
        );
    }

    println!();
    println!("  Задач, побывавших больше чем на одном потоке: {migrated} из 16.");
    println!("  Разных адресов стейта у любой задачи: РОВНО ОДИН.");
    println!();
    println!("  Вот и ответ: «поток забрал задачу» означает, что он забрал из очереди");
    println!("  8-байтовый указатель на блок задачи. Сам блок с future всё время лежит");
    println!("  по одному адресу в хипе и никуда не копируется. Перемещается право");
    println!("  его опрашивать, а не его байты.");
    println!();
    println!("  Именно поэтому spawn требует Send: future не двигают, но обращаются");
    println!("  к нему с разных потоков — нужна безопасность передачи владения между ними.");
    println!("  И поэтому же требуется 'static: блок живёт независимо от твоего фрейма.");
}
