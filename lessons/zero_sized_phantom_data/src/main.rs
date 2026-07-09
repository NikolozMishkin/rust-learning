use std::alloc::{GlobalAlloc, Layout, System};
use std::marker::PhantomData;
use std::mem::size_of;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// Счётчик аллокаций: оборачиваем системный аллокатор и считаем,
// сколько раз реально просили память в куче и сколько байт.
// Так мы наглядно увидим, кто "аллоцирует", а кто нет.
// ---------------------------------------------------------------------------
struct CountingAlloc;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOC_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

/// Выполняет `f` и возвращает (сколько раз аллоцировали в куче, сколько байт).
fn measure_allocs<F: FnOnce()>(f: F) -> (usize, usize) {
    let count_before = ALLOC_COUNT.load(Ordering::Relaxed);
    let bytes_before = ALLOC_BYTES.load(Ordering::Relaxed);
    f();
    let count = ALLOC_COUNT.load(Ordering::Relaxed) - count_before;
    let bytes = ALLOC_BYTES.load(Ordering::Relaxed) - bytes_before;
    (count, bytes)
}

// ---------------------------------------------------------------------------
// 1. Zero-sized маркер: PhantomData<Rc<()>>.
//    Убирает Send/Sync, размер = 0, куча НЕ трогается.
// ---------------------------------------------------------------------------
struct WithPhantom {
    _marker: PhantomData<Rc<()>>,
}

// ---------------------------------------------------------------------------
// 2. Настоящее поле Rc<()>: тоже убирает Send/Sync,
//    но занимает место (указатель) И аллоцирует в куче при создании.
// ---------------------------------------------------------------------------
struct WithRealRc {
    _marker: Rc<()>,
}

// ---------------------------------------------------------------------------
// 3. Сырой указатель: убирает Send/Sync, занимает место,
//    но в кучу НЕ ходит.
// ---------------------------------------------------------------------------
struct WithRawPtr {
    _marker: *const (),
}

// ---------------------------------------------------------------------------
// 4. Обычная структура с данными на стеке — размер есть, кучи нет.
// ---------------------------------------------------------------------------
struct PlainData {
    a: u64,
    b: u32,
    c: u8,
}

// ---------------------------------------------------------------------------
// 5. Структура, которая владеет данными в куче: Box, String, Vec.
//    Сама структура на стеке маленькая (набор указателей/длин),
//    но при создании РЕАЛЬНО аллоцирует.
// ---------------------------------------------------------------------------
struct OwnsHeap {
    boxed: Box<u64>,
    text: String,
    numbers: Vec<u32>,
}

fn main() {
    println!("=== Размеры типов (size_of) ===");
    println!("WithPhantom : {} байт", size_of::<WithPhantom>());
    println!("WithRealRc  : {} байт", size_of::<WithRealRc>());
    println!("WithRawPtr  : {} байт", size_of::<WithRawPtr>());
    println!("PlainData   : {} байт", size_of::<PlainData>());
    println!("OwnsHeap    : {} байт", size_of::<OwnsHeap>());
    println!(
        "  (для сравнения: Box<u64>={}, String={}, Vec<u32>={})",
        size_of::<Box<u64>>(),
        size_of::<String>(),
        size_of::<Vec<u32>>(),
    );

    println!("\n=== Аллокации в куче при создании ===");

    let (c, b) = measure_allocs(|| {
        let x = WithPhantom {
            _marker: PhantomData,
        };
        std::hint::black_box(&x);
    });
    println!("WithPhantom : {} аллокаций, {} байт  <- бесплатно", c, b);

    let (c, b) = measure_allocs(|| {
        let x = WithRealRc {
            _marker: Rc::new(()),
        };
        std::hint::black_box(&x);
    });
    println!("WithRealRc  : {} аллокаций, {} байт  <- Rc::new лезет в кучу", c, b);

    let (c, b) = measure_allocs(|| {
        let x = WithRawPtr {
            _marker: std::ptr::null(),
        };
        std::hint::black_box(&x);
    });
    println!("WithRawPtr  : {} аллокаций, {} байт  <- только стек", c, b);

    let (c, b) = measure_allocs(|| {
        let x = PlainData { a: 1, b: 2, c: 3 };
        std::hint::black_box(&x);
    });
    println!("PlainData   : {} аллокаций, {} байт  <- только стек", c, b);

    let (c, b) = measure_allocs(|| {
        let x = OwnsHeap {
            boxed: Box::new(42),
            text: String::from("привет из кучи"),
            numbers: vec![1, 2, 3, 4, 5],
        };
        std::hint::black_box(&x);
    });
    println!("OwnsHeap    : {} аллокаций, {} байт  <- Box + String + Vec", c, b);

    println!("\n=== Send/Sync проверяются на этапе компиляции ===");
    // Раскомментируй любую строку ниже — код НЕ скомпилируется,
    // потому что PhantomData<Rc<()>> / Rc / *const () снимают Send.
    //
    // require_send::<WithPhantom>();
    // require_send::<WithRealRc>();
    // require_send::<WithRawPtr>();
    //
    // А эти компилируются нормально:
    require_send::<PlainData>();
    require_send::<OwnsHeap>();
    println!("PlainData и OwnsHeap реализуют Send — ok");
    println!("WithPhantom/WithRealRc/WithRawPtr — НЕ Send (см. закомментированные строки)");
}

fn require_send<T: Send>() {}
