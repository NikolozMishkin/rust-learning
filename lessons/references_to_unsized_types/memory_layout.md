# Memory Layout: Stack & Heap Diagrams

> **Interactive visual version:** [https://claude.ai/code/artifact/032fcf80-f281-4216-8991-2be5be766a72](https://claude.ai/code/artifact/032fcf80-f281-4216-8991-2be5be766a72)

Types from `main.rs` lines 98–129.

> **Notation:** `ptr` = raw pointer (8 bytes on 64-bit), `len` = byte/element count in use, `cap` = total allocated capacity.
> Thin reference = 1 pointer (8 B). Fat reference = pointer + extra metadata (len or vtable ptr).

---

## Primitive Types — stack only

### `u8` — 1 byte

```
STACK
┌──────────────────────┐
│  value   │  1 byte   │
└──────────────────────┘
```

### `u32` — 4 bytes

```
STACK
┌──────────────────────┐
│  value   │  4 bytes  │
└──────────────────────┘
```

### `u64` — 8 bytes

```
STACK
┌──────────────────────┐
│  value   │  8 bytes  │
└──────────────────────┘
```

### `bool` — 1 byte

```
STACK
┌──────────────────────┐
│  value   │  1 byte   │
└──────────────────────┘
```

---

## Thin References — single pointer, 8 bytes

Applies to: `&u8`, `&u32`, `&u64`, `&bool`, `&Point`, `&String`, `&Vec<i32>`, `&Circle`, `&Rectangle`, `&Struct1`, `&Struct2`.

```
STACK                                    POINTEE (stack / .rodata)
┌──────────────────────┐                 ┌──────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  T  (the value)      │
└──────────────────────┘                 └──────────────────────┘
```

> No `len`, no `cap` — the reference does not own any buffer.

---

## Fat Pointer: `&str` — 16 bytes

```
STACK                                    .rodata / String heap buffer
┌──────────────────────┐                 ┌──────────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  h │ e │ l │ l │ o │ …  UTF-8│
├──────────────────────┤                 └──────────────────────────────┘
│  len     │  8 bytes  │                   (len bytes)
└──────────────────────┘
```

> No `cap` — `&str` is a read-only view, it does not own the buffer.

---

## Fat Pointer: `&[i32]` — 16 bytes

```
STACK                                    array / Vec<i32> buffer (heap or stack)
┌──────────────────────┐                 ┌───────────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  i32  │  i32  │  i32  │  …   │
├──────────────────────┤                 │  4 B  │  4 B  │  4 B  │      │
│  len     │  8 bytes  │                 └───────────────────────────────┘
└──────────────────────┘                   (len elements)
```

> No `cap` — a slice is a view, it does not own the buffer.

---

## Fat Pointer: `&[String]` — 16 bytes

```
STACK                                    array / Vec<String> buffer
┌──────────────────────┐                 ┌───────────────────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  String(24B) │ String(24B) │  …      │
├──────────────────────┤                 └───────────────────────────────────────┘
│  len     │  8 bytes  │                   (len elements, each String may point to heap)
└──────────────────────┘
```

---

## Thin Reference to Fixed-Size Array: `&[i32; 3]` — 8 bytes

```
STACK                                    array on stack (or .rodata)
┌──────────────────────┐                 ┌───────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  10   │  12   │  30   │
└──────────────────────┘                 │  4 B  │  4 B  │  4 B  │
                                         └───────────────────────┘
                                           total: 12 bytes
```

> Thin pointer — length `3` is baked into the type at compile time, no runtime `len` field.

---

## Thin Reference to Fixed-Size Array: `&[String; 3]` — 8 bytes

```
STACK                                    array on stack
┌──────────────────────┐                 ┌────────────────────────────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  String(24B) │  String(24B) │  String(24B)    │
└──────────────────────┘                 └────────────────────────────────────────────────┘
                                           total: 72 bytes (each String may point to heap)
```

> Thin pointer — no `len` field, size known from `[String; 3]`.

---

## Fixed-Size Array: `[i32; 10]` — 40 bytes, stack only

```
STACK
┌────────────────────────────────────────────────────────────────────┐
│  [0]  │  [1]  │  [2]  │  [3]  │  [4]  │  [5]  │  [6]  │  …  [9] │
│  4 B  │  4 B  │  4 B  │  4 B  │  4 B  │  4 B  │  4 B  │      4 B │
└────────────────────────────────────────────────────────────────────┘
Total: 10 × 4 = 40 bytes
```

> No pointer, no `len`, no `cap` — the array lives entirely on the stack.

---

## `Vec<i32>` — 24 bytes stack + heap

```
STACK                                    HEAP
┌──────────────────────┐                 ┌───────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  1    │  2    │  3    │…  │
├──────────────────────┤                 │  4 B  │  4 B  │  4 B  │   │
│  len     │  8 bytes  │                 └───────────────────────────┘
├──────────────────────┤                   len × 4 bytes in use
│  cap     │  8 bytes  │                   cap × 4 bytes allocated
└──────────────────────┘
Total stack: 24 bytes
```

---

## `String` — 24 bytes stack + heap

```
STACK                                    HEAP
┌──────────────────────┐                 ┌───────────────────────────┐
│  ptr     │  8 bytes  │────────────────►│  h  │  e  │  l  │  l  │… │
├──────────────────────┤                 │  1B │  1B │  1B │  1B │   │
│  len     │  8 bytes  │                 └───────────────────────────┘
├──────────────────────┤                   len bytes in use (UTF-8)
│  cap     │  8 bytes  │                   cap bytes allocated
└──────────────────────┘
Total stack: 24 bytes
```

---

## `Point` — 8 bytes, stack only

```
STACK
┌──────────────────────┐
│  x       │  4 bytes  │
├──────────────────────┤
│  y       │  4 bytes  │
└──────────────────────┘
Total: 8 bytes
```

---

## `Circle` / `Rectangle` — 0 bytes (ZST)

Both are zero-sized types. They carry no data.

```
STACK
┌──────────────────────┐
│  (no fields)   0 B   │
└──────────────────────┘
```

`&Circle` and `&Rectangle` are thin pointers (8 bytes). Rust gives ZSTs a valid non-null address.

---

## `&dyn Shape` — 16 bytes (wide/fat pointer)

```
STACK
┌──────────────────────┐
│  data ptr │  8 bytes │────────────────► (concrete Circle or Rectangle instance)
├──────────────────────┤
│  vtable   │  8 bytes │────────────────► ┌──────────────────────────┐
└──────────────────────┘                  │  drop_in_place │  8 B     │
                                          │  size_of       │  8 B     │
                                          │  align_of      │  8 B     │
                                          │  Shape::print  │  8 B     │
                                          └──────────────────────────┘
                                            (vtable lives in .rodata)
```

> The vtable pointer is the difference from a thin `&T` — it enables dynamic dispatch at runtime.

---

## `Struct1` — 64 bytes stack + heap

Fields: `my_str: &str` (16 B) + `my_string: String` (24 B) + `my_vec: Vec<i32>` (24 B)

```
STACK                                        HEAP / .rodata
┌──────────────────────────────────┐
│ my_str                           │
│   ptr    │  8 bytes              │────────► ┌──────────────────────────┐
│   len    │  8 bytes              │          │  "hello"  UTF-8 bytes    │  (.rodata)
├──────────────────────────────────┤          └──────────────────────────┘
│ my_string                        │
│   ptr    │  8 bytes              │────────► ┌──────────────────────────┐
│   len    │  8 bytes              │          │  heap bytes (UTF-8)      │  (heap)
│   cap    │  8 bytes              │          └──────────────────────────┘
├──────────────────────────────────┤
│ my_vec                           │
│   ptr    │  8 bytes              │────────► ┌──────────────────────────┐
│   len    │  8 bytes              │          │  i32 │ i32 │ i32 │  …   │  (heap)
│   cap    │  8 bytes              │          └──────────────────────────┘
└──────────────────────────────────┘
Total stack: 16 + 24 + 24 = 64 bytes
```

---

## `Struct2` — 920 bytes stack + heap

Fields: `my_u8: u8` + 7-byte alignment padding, then 19 × `String` + 19 × `Vec<i32>`.

```
STACK
┌──────────────────────────────────────────────┐
│ my_u8    │  1 byte                            │
│ padding  │  7 bytes  (align to 8 for String)  │
├──────────────────────────────────────────────┤
│ my_string1:  ptr │ len │ cap  │  24 bytes     │──► heap
│ my_vec1:     ptr │ len │ cap  │  24 bytes     │──► heap
├──────────────────────────────────────────────┤
│ my_string2:  ptr │ len │ cap  │  24 bytes     │──► heap
│ my_vec2:     ptr │ len │ cap  │  24 bytes     │──► heap
├──────────────────────────────────────────────┤
│              … × 19 pairs …                  │
├──────────────────────────────────────────────┤
│ my_string19: ptr │ len │ cap  │  24 bytes     │──► heap
│ my_vec19:    ptr │ len │ cap  │  24 bytes     │──► heap
└──────────────────────────────────────────────┘

   8  (u8 + padding)
+ 19 × 24  (Strings)  =  456
+ 19 × 24  (Vecs)     =  456
──────────────────────────────
Total stack: 920 bytes
```

> Every `String` and `Vec` on the stack points to its own separate heap allocation.

---

## Summary Table

| Type             | Stack (B) | Heap? | ptr | len | cap |
|------------------|-----------|-------|-----|-----|-----|
| `u8`             | 1         | no    | —   | —   | —   |
| `u32`            | 4         | no    | —   | —   | —   |
| `u64`            | 8         | no    | —   | —   | —   |
| `bool`           | 1         | no    | —   | —   | —   |
| `&u8`            | 8         | no    | yes | —   | —   |
| `&u32`           | 8         | no    | yes | —   | —   |
| `&u64`           | 8         | no    | yes | —   | —   |
| `&bool`          | 8         | no    | yes | —   | —   |
| `&str`           | 16        | no*   | yes | yes | —   |
| `&[i32; 3]`      | 8         | no    | yes | —   | —   |
| `&[String; 3]`   | 8         | no    | yes | —   | —   |
| `&[i32]`         | 16        | no    | yes | yes | —   |
| `&[String]`      | 16        | no    | yes | yes | —   |
| `[i32; 10]`      | 40        | no    | —   | —   | —   |
| `Vec<i32>`       | 24        | yes   | yes | yes | yes |
| `&Vec<i32>`      | 8         | no    | yes | —   | —   |
| `String`         | 24        | yes   | yes | yes | yes |
| `&String`        | 8         | no    | yes | —   | —   |
| `Point`          | 8         | no    | —   | —   | —   |
| `&Point`         | 8         | no    | yes | —   | —   |
| `Circle`         | 0 (ZST)   | no    | —   | —   | —   |
| `&Circle`        | 8         | no    | yes | —   | —   |
| `Rectangle`      | 0 (ZST)   | no    | —   | —   | —   |
| `&Rectangle`     | 8         | no    | yes | —   | —   |
| `&dyn Shape`     | 16        | no    | yes | —   | —   |
| `Struct1`        | 64        | yes   | —   | —   | —   |
| `&Struct1`       | 8         | no    | yes | —   | —   |
| `Struct2`        | 920       | yes   | —   | —   | —   |
| `&Struct2`       | 8         | no    | yes | —   | —   |

*`&str` points to `.rodata` (for string literals) or into a `String`'s heap buffer — it does not make its own heap allocation.
