# vtable, Dispatch, and Process Memory Layout

Notes from breaking down `&dyn Shape` (lines 152–154 in `main.rs`): how a vtable is
built and how everything is laid out in the compiled program.

See also the per-type diagrams: [MEMORY_LAYOUT.md](MEMORY_LAYOUT.md).

---

## 1. Static vs Dynamic Dispatch

A function in a compiled program is just bytes of machine code at some address.
A "function reference" = 8 bytes holding the address of its first instruction.

- **Static dispatch** — the compiler knows the concrete type, so it emits a direct
  call. The address is known at compile time; there is no runtime lookup.
- **Dynamic dispatch** (`dyn Shape`) — the concrete type is erased; a `&dyn Shape`
  could be behind a `Circle`, a `Rectangle`, etc. The method's address must be looked
  up at runtime — that's what the **vtable** is for.

### On the "hardcoded address" and ASLR

The program is loaded at a random base address every run (ASLR protection). So what's
hardcoded is **not an absolute address, but a relative offset**:

- A direct `call` uses **RIP-relative** addressing: `call rip + 0x1234` — "jump
  0x1234 bytes forward." The relative layout of code within `.text` never changes, so
  the offset is invariant with respect to `base`. This is called **PIE**
  (Position-Independent Executable).
- Absolute addresses inside the vtable are **fixed up by relocations** at load time:
  the loader walks the relocation table and adds the real `base` to each address.

---

## 2. Fat Pointer: Why `&dyn Shape` = 16 bytes

A regular reference `&Circle` = 8 bytes (just an address). But `&dyn Shape` = a
**fat pointer**:

```
&dyn Shape  (16 bytes)
┌────────────────────────┬────────────────────────┐
│   data pointer (8)     │   vtable pointer (8)    │
│  → address of Circle   │  → address of vtable    │
└────────────────────────┴────────────────────────┘
```

The vtable pointer lives in the **pointer**, not inside the object (unlike C++). This
is why the same `Circle` can be coerced to different trait objects — a different
vtable is substituted each time.

---

## 3. vtable Structure

For **each pair** `(concrete type, trait)` the compiler builds a SEPARATE vtable.
`(Circle, Shape)`, `(Rectangle, Shape)`, `(Circle, Display)` — three different tables.

```
vtable for (Circle, Shape) — 5 slots of 8 bytes each:
┌──────────────────────────────────────────────────────────────┐
│ [0] drop_in_place  → POINTER to function (code address .text)  │  pointer
│ [1] size    = 0    → the usize value itself, sits in the slot  │  data
│ [2] align   = 1    → the usize value itself, sits in the slot  │  data
│ [3] print          → POINTER to function                       │  pointer
│ [4] my_super_fn    → POINTER to function                       │  pointer
└──────────────────────────────────────────────────────────────┘
```

**The service header `[0]..[2]` is present in EVERY vtable — these are NOT trait methods:**

- `drop_in_place` — how to destroy the object (needed for `Box<dyn Shape>`); a pointer to code.
- `size` / `align` — plain numbers written into the table. Needed to free heap memory
  (`Box<dyn>` → `dealloc(ptr, Layout{size, align})`) and for `size_of_val` /
  `align_of_val`, since the type is erased and the size can't be known otherwise.

**size ≠ align:**
- `size` — how many bytes an instance occupies (`Circle` = ZST → 0 bytes).
- `align` — which address it may live at (address % align == 0). For a ZST it's 1 (no
  constraint; align is never 0, minimum is 1). Example: `struct{a:u8,b:u32}` →
  `size=8` (3 bytes of padding!), `align=4`.

`size`/`align` are hardcoded by the compiler and do NOT take part in relocations (they
are plain numbers, independent of any address). Function addresses do.

### The `link.my_super_fn()` call
1. Take the vtable pointer from the fat pointer.
2. Read the method's slot from the vtable → the function address is there.
3. Take the data pointer → this becomes `&self`.
4. `call <address>(data pointer)`.

Double indirection → one extra memory load compared to static dispatch.

---

## 4. Type Code vs Type Instance

An important distinction for `Atomic`, `Mutex`, and any type:

- **Method code** (`fetch_add`, `lock`, …) → always in `.text`, ONE copy for all instances.
- **The instance** (data) → lives wherever you declared it.

`AtomicU64` is internally just an `UnsafeCell<u64>` (8 bytes of data). `UnsafeCell`
allows mutating the data through `&`, which is why such a `static` goes into
`.bss`/`.data` rather than `.rodata`.

---

## 5. const vs static

| | `const` | `static` |
|---|---|---|
| What it is | a named compile-time **value** | a real **variable** with an address |
| Address | no guaranteed address | one fixed address (`'static`) |
| Usage | **inlined** (copied) at each use site | accessed via its single address |

**Where a `static` lands:**
```
static without mutation (u32, &str)     → .rodata (r--)
static mut / Atomic / Mutex, start ≠ 0  → .data   (rw-)
static mut / Atomic / Mutex, start = 0  → .bss    (rw-)
```

---

## 6. Executable Sections and Permissions

The `rw-`/`r-x` permissions belong to **the process's memory pages**, NOT to the file.
The on-disk file never changes. Writing to `.data` uses **copy-on-write**: a private
copy of the page in memory is edited, and the original file is untouched (which is why
you can run 10 copies of a program).

- **`.data`** = "data": mutable globals ≠ 0; initial values ARE stored in the file.
- **`.bss`** = "Block Started by Symbol" (from the IBM 704 assembler, 1950s): mutable
  globals = 0. Space is RESERVED, but no bytes are in the file — the OS zeroes it at
  load time (saves space; `static mut BUF: [u8; 1_000_000]` doesn't bloat the file).

**Why `.data`/`.bss` are needed if stack and heap exist:**
- the stack is temporary — it dies with the function;
- the heap requires runtime allocation;
- `.data`/`.bss` — global state with a fixed address, ready BEFORE the program starts,
  with no allocation at all.

**Where an instance lives is decided by its declaration:**

| How it's declared | Where the data goes |
|---|---|
| `static` / `static mut` | `.data` / `.bss` / `.rodata` |
| `let` (local) | **stack** |
| `Box`, `Vec`, `String`, `Rc` | **heap** (header on the stack, contents on the heap) |

---

## 7. Full Process Virtual Memory Map

```
HIGH ADDRESSES  0x7fff...
┌─────────────────────────────────────────────────────────────┐
│                    arguments, env variables                   │
├─────────────────────────────────────────────────────────────┤
│  STACK                                                 rw-    │
│  ├─ main() frame                                              │
│  ├─ called function's frame                                   │
│  │                                                            │
│  ▼ GROWS DOWN (toward lower addresses) on each function call  │
│                                                               │
│              ← large empty gap between stack and heap →        │
│                                                               │
│  ▲ GROWS UP (toward higher addresses) on allocation           │
│  │                                                            │
│  HEAP                                                  rw-    │
│  ├─ data of Box, Vec, String...                               │
├─────────────────────────────────────────────────────────────┤
│  .bss     zero-initialized globals                     rw-    │  ┐
├─────────────────────────────────────────────────────────────┤  │
│  .data    mutable globals ≠ 0                          rw-    │  │ comes
├─────────────────────────────────────────────────────────────┤  │ from the
│  .rodata  constants, strings, VTABLES                  r--    │  │ exec file
├─────────────────────────────────────────────────────────────┤  │
│  .text    machine code                                 r-x    │  ┘
└─────────────────────────────────────────────────────────────┘
LOW ADDRESSES   0x0  (bottom — the null page, catches null dereferences)
```

| Region | Source | When created | Size |
|---|---|---|---|
| `.text/.rodata/.data/.bss` | from the exec file | at load time, once | fixed |
| **stack** | created by the OS at startup | when the process starts | ~8 MB, auto grow/shrink |
| **heap** | requested from the OS at runtime | as allocations happen | grows on demand |

**Stack** — fast (bump the RSP register), automatic (the frame vanishes on return),
bounded (~8 MB → infinite recursion = stack overflow), grows down.

**Heap** — slower (goes through the allocator); in Rust it's freed automatically by the
ownership rules (Drop), grows up.

`Vec`/`String`/`Box` are split: the header (ptr+len+cap = 24 bytes for Vec/String)
lives on the stack, the contents on the heap. That's why `size_of::<Vec<i32>>() = 24`
— that's only the header.

---

## 8. All Together on `let link: &dyn Shape = &Circle;`

```
STACK (main frame):
   link (16 bytes):
     ├ data ptr   → points to Circle (ZST → dummy aligned address)
     └ vtable ptr → points into .rodata ──┐
                                          │
.rodata (r--):                            │
   vtable (Circle, Shape) ◄───────────────┘
     [0] drop  ─┐
     [1] size=0 │
     [2] align=1│  function addresses point into .text
     [3] print ─┤
     [4] my_fn ─┘
                │
.text (r-x): ◄──┘
   machine code for Circle::my_super_fn, print, drop
```

Summary by section:

| What | Where | Permissions |
|---|---|---|
| function code | `.text` | `r-x` |
| strings, **vtables**, immutable `static`s, constants | `.rodata` | `r--` |
| mutable `static`s ≠ 0 | `.data` | `rw-` |
| mutable `static`s = 0 | `.bss` | `rw-` |
| local `let`s, fat pointers, arrays | **stack** | `rw-` |
| contents of `Vec`/`String`/`Box` | **heap** | `rw-` |
