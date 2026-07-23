# Ownership and Threads: How Data Moves Between Stacks

A deep dive into what physically happens when a closure is passed to
`thread::spawn`, and how data crosses from the `main` stack to a child
thread's stack.

## 1. A closure is just a struct

The commented-out code at the top of `main.rs` captures the right intuition.
The compiler desugars this:

```rust
let my_closure = move || {
    println!("{x}");
};
```

into roughly this anonymous struct + trait impl:

```rust
struct __my_closure {
    x: String,   // the captured variable becomes a field
}

impl FnOnce<()> for __my_closure {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: ()) {
        println!("thread spawned");
        println!("{}", self.x);
        println!("thread ended");
    }
}
```

So `my_closure` is an ordinary struct value living on `main`'s stack. It has
a single field `x` of type `String`.

## 2. What actually sits on the stack

A `String` is a "fat pointer" made of 3 machine words (24 bytes on 64-bit):

```
main's stack:
┌─────────────────────────────┐
│ my_closure (struct)         │
│  └─ x: String               │
│      ├─ ptr:  0x5A00 ────────┼──┐   pointer to the data
│      ├─ len:  11            │  │
│      └─ cap:  11            │  │
└─────────────────────────────┘  │
                                  ▼
Heap:
        0x5A00: "some string"   ← the actual string bytes
```

Key point: the bytes `"some string"` live on the **heap**, not the stack.
The stack only holds the metadata (pointer + length + capacity).

## 3. What `move` does

`move` changes the capture mode: instead of borrowing (`&x`), the closure
takes `x` **by value** (moves ownership). The heap bytes are neither copied
nor relocated — only the 24-byte `String` header (ptr/len/cap) moves from
`main`'s stack into the `my_closure` struct.

After the closure is created, `x` no longer exists as a usable binding in
`main` — ownership has moved into `my_closure`. That is why
`println!("{x}")` after the spawn would not compile (and is commented out).

## 4. Passing into the thread — the stack-to-stack transition

```rust
let thread = thread::spawn(my_closure);
```

`thread::spawn`'s signature (simplified):

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
```

`f: F` is taken **by value**, so the `my_closure` struct (24 bytes) is
**moved** into `spawn`. The runtime hands it to the new thread, and it ends
up on the **child thread's stack**.

Step by step:

```
1. my_closure lives on main's stack
        │  spawn(my_closure) — move
        ▼
2. The struct (ptr/len/cap) is byte-copied onto the new thread's stack;
   the old copy on main's stack is considered "moved-out" (unusable).
        ▼
3. The heap bytes "some string" do NOT move!
   The ptr in the copy on the child stack simply points at them.
```

Resulting memory layout:

```
main's stack:                 child thread's stack:
┌──────────────────┐          ┌──────────────────────┐
│ my_closure       │          │ my_closure (moved     │
│ (moved-out,      │          │  copy)                │
│  unusable)       │          │   x: String           │
└──────────────────┘          │    ├─ ptr: 0x5A00 ────┼──┐
                              │    ├─ len: 11         │  │
                              │    └─ cap: 11         │  │
                              └──────────────────────┘  │
                                                         ▼
              Heap (shared by all threads in the process):
                    0x5A00: "some string"  ← never moved
```

## 5. Why this is safe — two key bounds

Note the bounds on `spawn`: `Send + 'static`.

- **`'static`** — the closure must not hold borrowed references with a
  limited lifetime. The thread may outlive `main`, so it cannot be given
  `&x` — only full ownership. Hence `move` is required. Without `move`, the
  compiler errors: the closure would try to capture `&x`, but the thread may
  outlive `x`.

- **`Send`** — the type can be safely transferred to another thread.
  `String` is `Send`, so its ownership can be handed over. `Rc<T>` is **not**
  `Send` (non-atomic refcount), so `spawn` would refuse to compile with it.

## 6. Summary: the two levels of memory

When a closure is moved into a thread, think in two layers:

1. **Stack part** of `my_closure` (the fields themselves: `String`'s
   ptr/len/cap, primitives, nested inline structs) is **byte-copied** onto
   the new thread's stack. A cheap fixed-size `memcpy`.

2. **Heap data** pointed to by those fields (the string characters, `Vec`
   contents, etc.) is **not copied or moved**. The heap is shared across the
   whole process; only which stack's pointer references it changes.

`move` in Rust does not mean "physically relocate everything" — it means
"transfer **ownership**". At the machine level that is a small stack copy
plus a logical ban on using the old location. That is why a move is almost
always cheap, regardless of how large the heap-allocated string is.

## Next step

Compare with capturing `Rc<String>` (won't compile — not `Send`) vs
`Arc<String>` (compiles, shared ownership via an atomic refcount).
