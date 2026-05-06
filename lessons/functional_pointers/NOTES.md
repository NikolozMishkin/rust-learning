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

## Когда что использовать

**`fn(...)`** — когда точно знаешь, что никакого захвата не будет. Максимально просто, как в примере `is_valid_user`.

**`impl Fn(...)`** — **рекомендуемый дефолт** для аргументов функций. Принимает всё, zero-cost.

**`Box<dyn Fn(...)>`** — когда нужно хранить в struct/enum/Vec, или тип лямбды неизвестен в compile-time.

**`&'static dyn Fn(...)`** — редко нужен, в основном для глобальных валидаторов или конфигов.
