# Dereferencing in Rust: `*`, `.deref()`, and Place Expressions

## What Does "Dereference" Mean?

Dereferencing means **"follow the pointer to the data at that address"**. It is about memory addressing, not ownership.

```
r: &i32
│
│  this is just a number — a memory address
│  e.g. 0x7fff1234
│
└──► [0x7fff1234]: 42   ← the actual value
```

`*r` says: "go to the address stored in `r` and give me what is there."

---

## Why `.deref()` Returns a Reference

The `Deref` trait signature:

```rust
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

`deref` borrows `self` and returns a **reference** to the inner data — not the data itself. If it returned `T` directly, it would either copy (only works for `Copy` types) or move the value out of the container, leaving it invalid.

So `i.deref()` gives you `&i32` — an immutable reference. You cannot assign through it.
`*i =` goes one step further (see below).

---

## How `*` Actually Works (Two Steps)

The `*` operator is compiler magic on top of `.deref()`:

```
*s
 │
 ├─ step 1: call s.deref()  →  get &T
 │
 └─ step 2: dereference &T  →  get T as a place expression
```

This is why `i.deref() = value` does not compile (you stop at step 1, holding `&i32`),
but `*i = value` does — the compiler performs both steps and produces a writable memory location.

---

## With Primitives (`i32` — Copy type)

```rust
let x: i32 = 42;
let r: &i32 = &x;

let y = *r;   // dereference → read 42 → COPY into y (i32 is Copy)
```

```
STACK
─────────────────────────
x:  [42]  ← address 0x100
r:  [0x100]

*r → go to 0x100 → see 42 → i32 is Copy → copy the bits

y:  [42]  ← independent copy, x still alive
```

No ownership transfer — just bit-copying. `x` remains valid.

---

## With `String` (non-Copy type)

```rust
let s = String::from("hello");
let r: &String = &s;

let t = *r;   // ❌ ERROR: cannot move out of `*r` which is behind a shared reference
```

```
STACK                   HEAP
──────────────────      ──────────────
s:  ptr ─────────────► "hello"
    len: 5
    cap: 5

r:  [address of s]
```

`*r` → go to the address → land on `s`. To put it into `t` we would need a **move** (since `String` is not `Copy`).
A move would transfer `ptr + len + cap` from `s` to `t` and invalidate `s` — but `r` still points to `s`.
That would make `r` a dangling reference. **Rust forbids this at compile time.**

```
STACK                   HEAP
──────────────────      ──────────────
s:  [INVALID ☠]
t:  ptr ─────────────► "hello"
    len: 5
    cap: 5

r:  [address of s]    // r now points to invalid memory ☠
```

---

## Place Expressions and `*i =`

When `*r` appears on the **left side of `=`**, it is not "read the value" — it is **"name this memory location"**:

```rust
let mut x = 42i32;
let r: &mut i32 = &mut x;

*r = 100;
// *r = "the memory cell at r's address"
// write 100 there
// no move, no copy — direct write through the pointer
```

```
BEFORE:                 AFTER:
x: [42]                 x: [100]
r: [address of x]       r: [address of x]
```

Equivalent to `*ptr = 100` in C.

---

## `*i = *i * 2` — Full Breakdown

```rust
let mut vec_1 = vec![4, 5, 6, 9, 8];
for i in vec_1.iter_mut() {   // i: &mut i32
    *i = *i * 2;
}
```

| Sub-expression | What happens |
|----------------|--------------|
| `*i` on the right | dereference → copy the `i32` value (Copy type) |
| `*i * 2` | multiply the copied value by 2 |
| `*i =` on the left | place expression → write result back to that memory location |

---

## What to Do With `&mut String` When You Need the Old Value

You cannot move out of a reference, but you have two safe options:

```rust
let mut vec = vec![String::from("hello"), String::from("world")];

for i in vec.iter_mut() {
    // i: &mut String

    let old = i.clone();              // clone — heap allocation, keeps original intact
    let old = std::mem::take(i);      // take — moves value out, replaces it with String::default() ("")
    *i = String::from("new");         // write a brand-new String in place
}
```

`std::mem::take` memory layout:

```
BEFORE take:             AFTER take
String[0]: ptr → "hello" String[0]: ptr → ""   (valid empty String)
                         old:       ptr → "hello" (you own it now)
```

No holes, no dangling pointers — the Vec stays valid.

---

## Summary

| Expression | Context | What happens |
|------------|---------|--------------|
| `*r` (right side), `r: &i32` | Copy type | copy the bits, produce a new value |
| `*r` (right side), `r: &String` | non-Copy | **error** — move through reference is forbidden |
| `*r =` (left side), `r: &mut T` | any type | write to the memory location, no move |
| `.deref()` | any | returns `&T` — step 1 of `*` only |
| `std::mem::take(r)` | `r: &mut T where T: Default` | moves value out, leaves `T::default()` behind |

**Mental model:** dereferencing = follow the pointer. What happens next (copy / move / place write) depends on the type and whether the expression is on the left or right side of `=`.
