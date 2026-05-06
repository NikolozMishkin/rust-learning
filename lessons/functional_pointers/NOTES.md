# Функциональные указатели в Rust

## Три способа передавать функции

---

### 1. `fn(&str, &str) -> bool` — Function Pointer (указатель на функцию)

Это **голый указатель** на конкретную функцию. Только `fn`-функции и лямбды **без захвата** окружения.

```rust
fn validate_simple(name: &str, banned: &str) -> bool {
    name != banned
}

fn run(validator: fn(&str, &str) -> bool) {
    println!("{}", validator("alice", "banned"));
}

fn main() {
    run(validate_simple);              // ✅ fn-функция

    let min_len = 3;                   // переменная из окружения
    run(|a, b| a != b);               // ✅ лямбда БЕЗ захвата
    run(|a, b| a.len() > min_len && a != b); // ❌ ОШИБКА — захватывает min_len
}
```

**Память:** только один указатель (8 байт) — всё на **стеке**.

---

### 2. `impl Fn(&str, &str) -> bool` — impl Trait (статическая диспетчеризация)

Это **дженерик под капотом** — компилятор знает точный тип в compile-time и генерирует отдельную версию функции. Принимает **любой** тип реализующий `Fn`: и `fn`-указатели, и лямбды с захватом.

```rust
fn run(validator: impl Fn(&str, &str) -> bool) {
    println!("{}", validator("alice", "banned"));
}

fn main() {
    let min_len = 3;
    let banned_list = vec!["admin", "root"];  // захват из окружения

    run(validate_simple);                                     // ✅ fn-функция
    run(|a, b| a != b);                                      // ✅ лямбда без захвата
    run(|a, b| a.len() > min_len && a != b);                 // ✅ захватывает min_len
    run(|a, b| !banned_list.contains(&a) && a != b);         // ✅ захватывает banned_list
}
```

**Память:** захваченные переменные — на **стеке** (если размер известен в compile-time). Нет heap-аллокации.

**Ограничение:** нельзя хранить в `struct` с разными типами лямбд одновременно — каждая лямбда это уникальный тип.

```rust
// ❌ ПРОБЛЕМА: impl Fn в struct требует дженерик-параметр
struct Validator<F: Fn(&str) -> bool> {
    check: F,
}

fn main() {
    let v1 = Validator { check: |s| s.len() > 3 };
    let v2 = Validator { check: |s| s != "banned" };

    // ❌ Нельзя положить в один Vec — у v1 и v2 РАЗНЫЕ типы!
    // Validator<[closure@line_5]> ≠ Validator<[closure@line_6]>
    let validators = vec![v1, v2]; // ОШИБКА КОМПИЛЯЦИИ
}
```

```
error[E0308]: mismatched types
  --> src/main.rs:10:32
   |
   | let validators = vec![v1, v2];
   |                           ^^ expected closure found a different closure
```

`impl Fn` — это синтаксический сахар для дженерика. Компилятор разворачивает в:

```rust
// impl Fn под капотом — это:
fn run<F: Fn(&str) -> bool>(f: F) { ... }
//     ^^^^^^^^^^^^^^^^^^^^ каждая лямбда = отдельный тип F
```

Поэтому `Vec<F>` может хранить только один конкретный тип `F`.

---

### 3. `&'static dyn Fn(&str, &str) -> bool` — Trait Object (динамическая диспетчеризация)

Это **fat pointer** — два указателя: один на данные, второй на vtable (таблицу методов). Тип известен только в **runtime**.

```rust
fn run(validator: &dyn Fn(&str, &str) -> bool) {
    println!("{}", validator("alice", "banned"));
}

fn main() {
    let min_len = 3;

    run(&validate_simple);                      // ✅
    run(&|a, b| a.len() > min_len && a != b);  // ✅

    // &'static — значит живёт всю программу:
    let v: &'static dyn Fn(&str) -> bool = &|s| s.len() > 3;
    // ❌ Нельзя: лямбда может не прожить 'static если захватывает не-static данные
}
```

**Практичнее использовать `Box<dyn Fn(...)>`** — когда нужно хранить в struct или Vec:

```rust
// ✅ РЕШЕНИЕ: Box<dyn Fn(...)> — один тип для всех лямбд
struct Validator {
    check: Box<dyn Fn(&str) -> bool>,
}

fn main() {
    let min_len = 5;

    // Три РАЗНЫЕ лямбды — но тип у всех одинаковый: Box<dyn Fn(&str) -> bool>
    let validators: Vec<Validator> = vec![
        Validator { check: Box::new(|s| s.len() > 3) },
        Validator { check: Box::new(|s| s != "banned") },
        Validator { check: Box::new(move |s| s.len() > min_len) }, // захват
    ];

    let name = "alice";
    let all_valid = validators.iter().all(|v| (v.check)(name));
    println!("All valid: {}", all_valid);
}
```

`Box<dyn Fn>` стирает тип через vtable — и все лямбды становятся `Box<dyn Fn(&str) -> bool>`.

**Память:** fat pointer (16 байт) на **стеке**, сама лямбда с захваченными данными — в **heap**.

---

## Таблица сравнения

| Характеристика | `fn(...)` | `impl Fn(...)` | `&dyn Fn(...)` / `Box<dyn Fn(...)>` |
|---|---|---|---|
| **Диспетчеризация** | статическая | статическая | динамическая (runtime) |
| **Лямбды с захватом** | ❌ нет | ✅ да | ✅ да |
| **Чистые fn-функции** | ✅ да | ✅ да | ✅ да |
| **Хранение в struct** | ✅ (просто) | ❌ (нужен дженерик) | ✅ через `Box<dyn>` |
| **Vec из разных лямбд** | ❌ | ❌ | ✅ `Vec<Box<dyn Fn(...)>>` |
| **Размер на стеке** | 8 байт (ptr) | размер захвата | 16 байт (fat ptr) |
| **Heap-аллокация** | ❌ | ❌ | ✅ (у `Box<dyn>`) |
| **Overhead runtime** | нет | нет | vtable lookup |
| **Монорфизация** | нет | ✅ да | нет |

---

## Что где хранится в памяти

```
СТЕК                           ХИП
────────────────────           ────────────────────
fn ptr (8 bytes)               (ничего)
────────────────────
impl Fn: сама лямбда
  + захваченные копии          (ничего, если данные Copy)
  (всё inline на стеке)
────────────────────
&dyn Fn: fat pointer           лямбда + захваченные данные
  [data_ptr | vtable_ptr]  →   [min_len, banned_list, ...]
  (16 bytes на стеке)
Box<dyn Fn>: то же самое   →   то же самое
```

---

## Как работает vtable изнутри

### Структура fat pointer

Обычный указатель — 8 байт (адрес данных).
**Fat pointer** — 16 байт (два указателя):

```
&dyn Fn(&str) -> bool
       │
       ├── [0..8]  data_ptr   →  адрес самой лямбды (её захваченные данные)
       └── [8..16] vtable_ptr →  адрес vtable (таблицы с методами)
```

### Что внутри vtable

vtable — это статический массив указателей на функции, созданный компилятором:

```
vtable для closure |s| s.len() > 3
┌─────────────────────────────────────────┐
│ drop_in_place   → 0x...  (деструктор)  │
│ size            → 8      (размер данных)│
│ align           → 8      (выравнивание) │
│ call (Fn::call) → 0x...  (сама логика)  │
└─────────────────────────────────────────┘
```

Когда ты вызываешь `(validator)("alice")` через `dyn Fn` — Rust делает:
1. Берёт `vtable_ptr` из fat pointer
2. Читает из vtable адрес `call`
3. Прыгает по этому адресу, передавая `data_ptr` как первый аргумент

### Пример: как это выглядит в памяти

```rust
fn main() {
    let min_len: usize = 5;  // захваченная переменная

    let check = move |s: &str| s.len() > min_len;

    let boxed: Box<dyn Fn(&str) -> bool> = Box::new(check);

    println!("{}", boxed("hello"));  // false (5 > 5 = false)
    println!("{}", boxed("hello!")); // true  (6 > 5 = true)
}
```

```
СТЕК                     ХИП
──────────────────       ──────────────────────────────
boxed (fat ptr):         адрес A: [данные лямбды]
  data_ptr  → A            min_len = 5  (usize, 8 байт)
  vtable_ptr → V
                          адрес V: [vtable]
                            drop_fn  → 0x401020
                            size     = 8
                            align    = 8
                            call_fn  → 0x401080  ← логика лямбды
```

### impl Fn vs dyn Fn — как компилятор обрабатывает по-разному

```rust
// impl Fn — МОНОРФИЗАЦИЯ
// Компилятор генерирует ОТДЕЛЬНУЮ функцию для каждого типа лямбды
fn run_static(f: impl Fn(&str) -> bool, s: &str) -> bool {
    f(s)
}
// Компилятор создаёт ДВЕ версии:
// run_static::<closure_1>  — для |s| s.len() > 3
// run_static::<closure_2>  — для |s| s != "banned"
// Вызов прямой — никакого vtable!
```

```rust
// dyn Fn — ДИНАМИЧЕСКАЯ ДИСПЕТЧЕРИЗАЦИЯ
// Одна функция, вызов через vtable
fn run_dynamic(f: &dyn Fn(&str) -> bool, s: &str) -> bool {
    f(s)  // → читает vtable → прыгает по указателю
}
```

### Три лямбды — три vtable

```rust
fn main() {
    let min_len = 3usize;
    let banned = String::from("root");

    // Три разных типа — три разных vtable генерирует компилятор
    let v1: Box<dyn Fn(&str) -> bool> = Box::new(|s| s.len() > 0);
    let v2: Box<dyn Fn(&str) -> bool> = Box::new(move |s| s.len() > min_len);
    let v3: Box<dyn Fn(&str) -> bool> = Box::new(move |s| s != banned);

    // ✅ Vec хранит одинаковый тип: Box<dyn Fn(&str) -> bool>
    // Разные vtable — но fat pointer всегда одного размера (16 байт)
    let validators: Vec<Box<dyn Fn(&str) -> bool>> = vec![v1, v2, v3];

    for v in &validators {
        println!("{}", v("alice"));
    }
}
```

```
validators (Vec на стеке):
┌────────────────────────────────────────────────────────────┐
│ v1: [data_ptr_1 | vtable_ptr_1]  →  vtable_1 { call→fn_1 }│
│ v2: [data_ptr_2 | vtable_ptr_2]  →  vtable_2 { call→fn_2 }│
│ v3: [data_ptr_3 | vtable_ptr_3]  →  vtable_3 { call→fn_3 }│
└────────────────────────────────────────────────────────────┘
```

Все элементы Vec одного размера (16 байт) — поэтому Vec работает. Разные vtable обеспечивают правильный вызов для каждой лямбды.

---

## Почему vtable — это overhead

vtable сам по себе — просто массив. Overhead не от этого, а от того **как CPU работает с косвенными вызовами**.

### Прямой вызов (impl Fn)

```asm
call 0x401080    ; адрес зашит в инструкцию на этапе компиляции
```

CPU видит адрес **заранее** — он может:
- предсказать следующую инструкцию (branch prediction)
- загрузить код функции в i-cache заблаговременно
- выполнить спекулятивно

### Косвенный вызов (dyn Fn через vtable)

```asm
mov rax, [rbx + 24]   ; 1. читаем адрес из vtable (rbx = vtable_ptr)
call rax              ; 2. прыгаем по адресу
```

Проблема в `call rax` — CPU **не знает** значение `rax` пока не выполнит предыдущую инструкцию. Два узких места:

**1. Cache miss при чтении vtable**

vtable живёт в памяти. Если её нет в L1/L2 кэше — CPU ждёт загрузки из RAM:

```
L1 cache hit  →  ~4 cycles
L2 cache hit  →  ~12 cycles
RAM           →  ~200 cycles   ← вот откуда overhead
```

**2. Branch misprediction**

CPU пытается угадать куда прыгнет `call rax` чтобы выполнять инструкции спекулятивно. Если угадал неверно — выбрасывает ~15 cycles работы:

```
impl Fn:  call 0x401080  → CPU знает адрес → всегда угадывает
dyn Fn:   call rax       → адрес в регистре → CPU гадает
```

> **Важно:** если один и тот же `Box<dyn Fn>` вызывается в цикле миллион раз — CPU запомнит паттерн и предскажет правильно, overhead минимален. Но `Vec<Box<dyn Fn>>` с **разными** лямбдами — каждый вызов новый адрес, предсказание ломается.

---

## Когда что использовать

**`fn(...)`** — когда точно знаешь, что никакого захвата не будет. Максимально просто, как в примере `is_valid_user`.

**`impl Fn(...)`** — **рекомендуемый дефолт** для аргументов функций. Принимает всё, zero-cost.

**`Box<dyn Fn(...)>`** — когда нужно хранить в struct/enum/Vec, или тип лямбды неизвестен в compile-time.

**`&'static dyn Fn(...)`** — редко нужен, в основном для глобальных валидаторов или конфигов.
