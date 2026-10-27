// Расширенные примеры к RustMemoryLayouts.html:
// closures, сложные аллокации, ссылки на структуры и массивы.
// Каждый пример печатает size_of и адреса, а `region()` подсказывает,
// где лежат данные: stack / heap / static (.rodata/.data).

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::{align_of, size_of, size_of_val};
use std::rc::Rc;
use std::sync::Arc;

static MARKER: u8 = 0;

/// Грубая эвристика: стек — рядом с адресом локальной переменной,
/// static — рядом с адресом MARKER, иначе — heap.
fn region<T: ?Sized>(p: *const T) -> &'static str {
    let local = 0u8;
    let addr = p as *const u8 as usize;
    let stack = &local as *const u8 as usize;
    let stat = &MARKER as *const u8 as usize;
    if addr.abs_diff(stack) < 8 * 1024 * 1024 {
        "stack"
    } else if addr.abs_diff(stat) < 16 * 1024 * 1024 {
        "static"
    } else {
        "heap"
    }
}

fn show<T: ?Sized>(name: &str, p: *const T) {
    println!("    {name:<28} {:p}  [{}]", p as *const u8, region(p));
}

// ───────────────────────────── CLOSURES ─────────────────────────────

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // замыкание = анонимная структура { n: i32 }, 4 B, без кучи
}

fn make_boxed(prefix: String) -> Box<dyn Fn(&str) -> String> {
    // структура { prefix: String } (24 B) кладётся в кучу,
    // Box<dyn Fn> = fat pointer (data ptr + vtable ptr) = 16 B
    Box::new(move |s| format!("{prefix}{s}"))
}

pub fn closures() {
    println!("\n=== CLOSURES ===");

    // 1. Ничего не захватывает → ZST, 0 байт. Можно привести к fn-pointer.
    let no_capture = |x: i32| x * 2;
    println!("no_capture:            {} B", size_of_val(&no_capture)); // 0
    let as_fn_ptr: fn(i32) -> i32 = no_capture;
    println!("fn(i32)->i32 pointer:  {} B", size_of_val(&as_fn_ptr)); // 8

    // 2. Захват по ссылке (Fn) → структура из ссылок, 8 B на каждую.
    let a = 10i32;
    let b = 20i64;
    let by_ref = || a as i64 + b; // { a: &i32, b: &i64 }
    println!("by_ref (&a, &b):       {} B", size_of_val(&by_ref)); // 16

    // 3. Захват по значению примитивов (move) → копии внутри структуры.
    let by_move = move || a as i64 + b; // { b: i64, a: i32 } + padding
    println!("by_move (a, b):        {} B", size_of_val(&by_move)); // 16

    // 4. move String/Vec → сама структура на стеке (48 B), буферы — в куче.
    let name = String::from("Ferris");
    let nums = vec![1, 2, 3];
    let name_heap = name.as_ptr();
    let owning = move || name.len() + nums.len(); // { name: String, nums: Vec<i32> }
    println!("owning (String, Vec):  {} B", size_of_val(&owning)); // 48
    show("&owning (closure struct)", &owning);
    show("name buffer", name_heap);

    // 5. FnMut — захват &mut, счётчик живёт снаружи.
    let mut counter = 0u32;
    let mut inc = || counter += 1; // { counter: &mut u32 }
    inc();
    inc();
    println!("FnMut inc (&mut u32):  {} B, counter = {counter}", size_of::<&mut u32>());

    // 6. Mixed: часть по ссылке, часть по значению (move только для v).
    let big = [0u8; 100];
    let v = vec![1u8];
    let big_ref = &big;
    let mixed = move || big_ref.len() + v.len(); // { big_ref: &[u8;100], v: Vec<u8> }
    println!("mixed (&[u8;100], Vec): {} B", size_of_val(&mixed)); // 32

    // 7. move большого массива → копируется весь массив в замыкание!
    let copy_big = move || big.len(); // { big: [u8; 100] }
    println!("copy_big ([u8;100]):   {} B", size_of_val(&copy_big)); // 100

    // 8. impl Fn из функции — тип известен статически, стек.
    let add5 = make_adder(5);
    println!("impl Fn (n: i32):      {} B, add5(1) = {}", size_of_val(&add5), add5(1));

    // 9. Box<dyn Fn> — data в куче, vtable в .rodata.
    let boxed = make_boxed(String::from(">> "));
    println!("Box<dyn Fn>:           {} B (fat ptr)", size_of_val(&boxed)); // 16
    show("Box<dyn Fn> data", &*boxed as *const dyn Fn(&str) -> String);
    println!("    size_of_val(*boxed) = {} B (String внутри)", size_of_val(&*boxed));

    // 10. &dyn Fn — fat pointer на замыкание на стеке, кучи нет.
    let dyn_ref: &dyn Fn() -> usize = &owning;
    println!("&dyn Fn:               {} B", size_of_val(&dyn_ref)); // 16
    show("&dyn Fn data", dyn_ref as *const dyn Fn() -> usize);

    // 11. Vec<Box<dyn Fn>> — 3 уровня: стек → буфер fat-ptr'ов → объекты в куче.
    let handlers: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),               // ZST → Box не аллоцирует!
        Box::new(move |x| x * a),          // { a: i32 } → 4 B в куче
        Box::new(make_adder(100)),         // { n: i32 } → 4 B в куче
    ];
    println!("Vec<Box<dyn Fn>>:      {} B stack, buffer {} B", size_of_val(&handlers),
        handlers.capacity() * size_of::<Box<dyn Fn(i32) -> i32>>());
    for (i, h) in handlers.iter().enumerate() {
        println!("    [{i}] size_of_val = {} B, {:p}, result = {}",
            size_of_val(&**h), &**h as *const dyn Fn(i32) -> i32 as *const u8, h(2));
    }
}

// ───────────────────────── СЛОЖНЫЕ АЛЛОКАЦИИ ─────────────────────────

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

enum Message {
    Quit,                       // 0 B payload
    Move { x: i32, y: i32 },    // 8 B
    Write(String),              // 24 B
}

pub fn allocations() {
    println!("\n=== ALLOCATIONS ===");

    // Box<T> — 8 B на стеке, T в куче.
    let b = Box::new(42u64);
    println!("Box<u64>:              {} B", size_of_val(&b));
    show("*b", &*b);

    // Box<[i32]> и Box<str> — владеющие fat pointers (ptr + len), без cap.
    let bs: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    let bstr: Box<str> = "hi".into();
    println!("Box<[i32]>:            {} B", size_of_val(&bs)); // 16
    println!("Box<str>:              {} B", size_of_val(&bstr)); // 16

    // Option niche: None кодируется null-указателем → размер не растёт.
    println!("Option<Box<u64>>:      {} B", size_of::<Option<Box<u64>>>()); // 8
    println!("Option<&u8>:           {} B", size_of::<Option<&u8>>()); // 8
    println!("Option<String>:        {} B", size_of::<Option<String>>()); // 24
    println!("Option<u64>:           {} B (нет ниши → +tag)", size_of::<Option<u64>>()); // 16
    println!("Option<bool>:          {} B (ниша 2..=255)", size_of::<Option<bool>>()); // 1

    // enum: размер = max(вариант) + tag (или ниша).
    println!("enum Message:          {} B", size_of::<Message>()); // 24 (niche в String.cap)
    let _ = [Message::Quit, Message::Move { x: 1, y: 2 }, Message::Write("w".into())];

    // Vec<String> — две ступени кучи.
    let words = vec![String::from("alpha"), String::from("beta")];
    println!("Vec<String>:           {} B stack", size_of_val(&words));
    show("words buffer", words.as_ptr());
    show("words[0] bytes", words[0].as_ptr());

    // Vec<Vec<i32>> — внешний буфер из 24-байтных заголовков, каждый → свой буфер.
    let grid = vec![vec![1, 2], vec![3, 4, 5]];
    show("grid buffer", grid.as_ptr());
    show("grid[0] buffer", grid[0].as_ptr());
    show("grid[1] buffer", grid[1].as_ptr());

    // vs [[i32; 3]; 2] — один непрерывный кусок стека, никаких указателей.
    let flat = [[1, 2, 3], [4, 5, 6]];
    println!("[[i32;3];2]:           {} B", size_of_val(&flat)); // 24

    // Rc<T> — 8 B на стеке → RcBox { strong, weak, value } в куче.
    let rc1 = Rc::new(String::from("shared"));
    let rc2 = Rc::clone(&rc1);
    println!("Rc<String>:            {} B, strong = {}", size_of_val(&rc1), Rc::strong_count(&rc1));
    println!("    RcBox<String> ≈ {} B (strong 8 + weak 8 + String 24)", 8 + 8 + size_of::<String>());
    println!("    rc1 == rc2 ptr: {}", Rc::ptr_eq(&rc1, &rc2));
    show("RcBox value", &*rc1 as *const String);
    show("String bytes", rc1.as_ptr());

    // Rc<str> — fat (ptr + len), строка лежит прямо в RcBox, без 2-й аллокации.
    let rcs: Rc<str> = Rc::from("inline");
    println!("Rc<str>:               {} B (fat)", size_of_val(&rcs));

    // Arc<T> — то же, но счётчики атомарные.
    let arc = Arc::new([0u8; 16]);
    println!("Arc<[u8;16]>:          {} B", size_of_val(&arc));

    // Rc<RefCell<T>> — RcBox { strong, weak, RefCell { borrow_flag, value } }.
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    shared.borrow_mut().push(4);
    println!("RefCell<Vec<i32>>:     {} B (flag 8 + Vec 24)", size_of::<RefCell<Vec<i32>>>());

    // Связный список: каждый узел — отдельная аллокация.
    let list = Node { val: 1, next: Some(Box::new(Node { val: 2, next: Some(Box::new(Node { val: 3, next: None })) })) };
    println!("Node:                  {} B (i32 + pad + Option<Box>)", size_of::<Node>()); // 16
    let mut cur = &list;
    show("node 1", cur);
    while let Some(next) = &cur.next {
        cur = next;
        show(&format!("node {}", cur.val), cur);
    }

    // Box<dyn Shape> vs Vec<Box<dyn Shape>>.
    let shapes: Vec<Box<dyn crate::Shape>> = vec![Box::new(crate::Circle), Box::new(crate::Rectangle)];
    println!("Vec<Box<dyn Shape>>:   buffer = {} × 16 B", shapes.len());

    // HashMap<String, Vec<i32>> — заголовок на стеке, таблица в куче.
    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    map.insert("k".into(), vec![1]);
    println!("HashMap<String,Vec>:   {} B stack", size_of_val(&map)); // 48
}

// ─────────────────── ССЫЛКИ НА СТРУКТУРЫ И МАССИВЫ ───────────────────

#[derive(Debug)]
struct Player {
    id: u32,          // 4
    alive: bool,      // 1
    name: String,     // 24
    pos: [f32; 2],    // 8
}

struct View<'a> {
    player: &'a Player,     // 8  (thin)
    tags: &'a [&'a str],    // 16 (fat)
    row: &'a [i32; 4],      // 8  (thin)
}

pub fn struct_and_array_refs() {
    println!("\n=== REFS TO STRUCTS & ARRAYS ===");

    // Компилятор переупорядочивает поля: String(24) + pos(8) + id(4) + alive(1) + pad(3).
    println!("Player:                {} B, align {}", size_of::<Player>(), align_of::<Player>()); // 40
    let p = Player { id: 7, alive: true, name: "Neo".into(), pos: [1.0, 2.0] };
    let base = &p as *const Player as usize;
    println!("    offsets: name={} pos={} id={} alive={}",
        &p.name as *const _ as usize - base,
        &p.pos as *const _ as usize - base,
        &p.id as *const _ as usize - base,
        &p.alive as *const _ as usize - base);

    // &Player — 8 B, указывает на начало структуры (на стеке).
    let rp = &p;
    show("&p", rp);
    // &p.id — ссылка внутрь структуры: тот же регион, адрес со смещением.
    show("&p.id (interior)", &p.id);
    // &p.name — ссылка на 24-байтный заголовок String на стеке…
    show("&p.name (header)", &p.name);
    // …а &p.name[..] / &str — fat pointer прямо в кучу.
    let s: &str = &p.name;
    show("&str → bytes", s.as_ptr());

    // &&Player — двойная косвенность, всё ещё 8 B.
    let rrp = &rp;
    println!("&&Player:              {} B, (**rrp).id = {}", size_of_val(&rrp), (**rrp).id);

    // &Box<Player> vs &Player: первая — указатель на указатель.
    let bp = Box::new(Player { id: 9, alive: false, name: String::new(), pos: [0.0; 2] });
    show("&bp (Box на стеке)", &bp);
    show("&*bp (Player в куче)", &*bp);

    // Ссылки на массивы.
    let arr: [i32; 6] = [10, 20, 30, 40, 50, 60];
    let r_arr: &[i32; 6] = &arr;        // thin, 8 B
    let r_slice: &[i32] = &arr;         // fat, 16 B (unsize coercion)
    let r_mid: &[i32] = &arr[2..5];     // fat: ptr = &arr[2], len = 3
    let r_elem: &i32 = &arr[3];         // thin, указатель на 1 элемент
    println!("&[i32;6]: {} B, &[i32]: {} B", size_of_val(&r_arr), size_of_val(&r_slice));
    println!("    arr      @ {:p}", arr.as_ptr());
    println!("    r_mid    @ {:p} len {}  (смещение +{} B)",
        r_mid.as_ptr(), r_mid.len(), r_mid.as_ptr() as usize - arr.as_ptr() as usize);
    println!("    r_elem   @ {:p}  (смещение +{} B)",
        r_elem, r_elem as *const i32 as usize - arr.as_ptr() as usize);

    // &[i32; N] → &[i32; M] через try_into (длина проверяется в runtime).
    let first3: &[i32; 3] = arr[..3].try_into().unwrap();
    println!("    first3 = {first3:?}");

    // Двумерный массив: &[[i32; 3]; 2] — thin; &[[i32; 3]] — fat, len = кол-во строк.
    let m = [[1, 2, 3], [4, 5, 6]];
    let rows: &[[i32; 3]] = &m;
    println!("&[[i32;3]]:            {} B, rows.len() = {}", size_of_val(&rows), rows.len());

    // Массив ссылок: [&str; 3] = 3 × 16 B на стеке, байты — в .rodata.
    let names: [&str; 3] = ["a", "bb", "ccc"];
    println!("[&str; 3]:             {} B", size_of_val(&names)); // 48
    show("names[0] bytes", names[0].as_ptr());

    // Массив структур и ссылка на элемент.
    let team = [
        Player { id: 1, alive: true, name: "A".into(), pos: [0.0; 2] },
        Player { id: 2, alive: true, name: "B".into(), pos: [0.0; 2] },
    ];
    println!("[Player; 2]:           {} B", size_of_val(&team)); // 80
    let second = &team[1];
    println!("    &team[1] - &team[0] = {} B (= size_of::<Player>())",
        second as *const Player as usize - &team[0] as *const Player as usize);

    // &Vec<i32> vs &[i32]: первая → заголовок Vec (стек) → буфер (heap).
    let v = vec![1, 2, 3];
    let rv: &Vec<i32> = &v;
    let rs: &[i32] = &v;
    show("&Vec → Vec header", rv);
    show("&[i32] → buffer", rs.as_ptr());

    // Структура со ссылками: thin + fat + thin = 32 B.
    let tags = ["hero", "admin"];
    let row = [1, 2, 3, 4];
    let view = View { player: &p, tags: &tags, row: &row };
    println!("View<'a>:              {} B (8 + 16 + 8)", size_of::<View>());
    println!("    view.player.name = {}, tags = {:?}, row = {:?}", view.player.name, view.tags, view.row);
}
