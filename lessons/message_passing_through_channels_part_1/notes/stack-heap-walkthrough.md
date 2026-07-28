# Line-by-line walkthrough of `src/main.rs` — what is on the stack, what is in the heap, where the closure lives

The subject is Example 2 from [`../src/main.rs`](../src/main.rs), lines 32–59.
All sizes measured on macOS 14.5 / Apple M3 Max / rustc 1.97.1.

## The main rule

> **The heap is one per process. The stack is one per thread.**

So "the data went into another thread" almost always means: the 24-byte `String` struct was copied
from one thread's stack into the channel's buffer, while the byte `b'5'` in the heap **did not move at
all** — only its owner changed.

## Memory layout

```
        PROCESS
┌─────────────────────────────────────────────────────────────────┐
│ .rodata (static, read-only, baked into the binary)              │
│   "5"        ← string literal. NOT the stack, NOT the heap!     │
│   "Sending value ", "Received value is: " ...                   │
└─────────────────────────────────────────────────────────────────┘

┌── main STACK (7.98 MiB) ──────┐   ┌── thread t STACK (2 MiB) ────┐
│ tx: Sender<String>   16 bytes │   │  (its own mmap region,       │
│    └ ptr ──────────┐          │   │   created by pthread_create) │
│    └ flavor: List  │          │   │                              │
│ rx: Receiver<String> 16 bytes │   │ i: String        24 bytes    │
│    └ ptr ──────────┤          │   │   ptr ──────────────┐        │
│ t: JoinHandle<()>  24 bytes   │   │   cap: 1            │        │
│ received_status: bool 1 byte  │   │   len: 1            │        │
│ received_value: String 24 b.  │   │ tmp (i.clone())  24 b│       │
│    ptr ────────────┼──┐       │   │   ptr ───────────┐  │        │
└────────────────────┼──┼───────┘   └──────────────────┼──┼────────┘
                     │  │                              │  │
┌── HEAP (shared) ───▼──▼──────────────────────────────▼──▼────────┐
│                                                                  │
│  Counter<List<String>>  ← the channel body itself, a Box,        │
│    senders: AtomicUsize      created by mpsc::channel(). tx      │
│    receivers: AtomicUsize    and rx are just pointers to THIS.   │
│    chan: List<String>                                            │
│      head/tail: AtomicPtr ──┐                                    │
│                             │                                    │
│  Block (lazily allocated)  ◄┘  32 slots of (String + state)      │
│    slot[0]: String{ptr,1,1} ──► [b'5']  ← 1 byte, from i.clone() │
│    slot[1]: String{ptr,1,1} ──► [b'5']  ← 1 byte, from i         │
│    slot[2..32]: empty                                            │
│                                                                  │
│  Box<dyn FnOnce()>  ← the CLOSURE moved here (16 bytes)          │
│                                                                  │
│  Packet<()>  (Arc) ← how JoinHandle reports the result           │
└──────────────────────────────────────────────────────────────────┘
```

## Measured sizes

```
Sender<String>    = 16 bytes  (8 for the ptr to Counter + 8 for the SenderFlavor tag)
Receiver<String>  = 16 bytes
String            = 24 bytes  (ptr, cap, len)
JoinHandle<()>    = 24 bytes
closure           = 16 bytes  ← exactly size_of::<Sender<String>>(), no overhead
```

## Line by line

### `let (tx, rx) = mpsc::channel();`

One heap allocation: `Box::new(Counter { senders, receivers, chan: List::new() })`.
Inside it — two atomic reference counters (how many live `Sender`s/`Receiver`s) and the queue itself.

`tx` and `rx` on `main`'s stack are **16 bytes** each: 8 bytes of pointer to that `Counter` + 8 bytes
for the `enum SenderFlavor { Array | List | Zero }` tag. `channel()` = unbounded → `List`.

The block for messages is **not allocated yet** — it is allocated lazily, on the first `send`.

### `thread::spawn(move || { ... })`

The interesting part is where the closure lives.

**1. A closure is an anonymous struct.** The compiler looks at what the body captures and generates
roughly:

```rust
struct Closure_37 { tx: Sender<String> }   // 16 bytes
impl FnOnce<()> for Closure_37 { fn call_once(self) { /* body */ } }
```

`i` is **not** captured — it is declared *inside* the body, it is a local variable of the future call.
Only `tx` is captured.

**2. `move` moves `tx` into that struct.** Byte for byte: 16 bytes from the `tx` slot are copied into
the field `Closure_37.tx`. The `senders` counter in the heap **does not change** (this is a move, not a
clone). After this line `tx` in `main` is statically dead.

**3. The closure struct is born on `main`'s stack** — as a temporary, the argument to `spawn`.

**4. `spawn` puts it in the heap.** The new thread cannot reference `main`'s stack frame (that frame may
disappear first), and `pthread_create` accepts only a single `void*`. So inside std:

```rust
let main = Box::new(move || { /* call your closure + write the result into Packet */ });
let ptr = Box::into_raw(main);          // ownership leaks into a raw pointer
pthread_create(..., thread_start, ptr as *mut c_void);
```

Your closure ends up **nested inside that `Box`** — that is, in the heap. Plus a separate
`Arc<Packet<()>>` is allocated for the thread's result/panic.

**5. The OS gives the new thread its own stack** — a separate `mmap` region, 2 MiB by default
(the `DEFAULT_MIN_STACK_SIZE` constant in std, not an OS value; see `os-process-thread.html` §3).
The stacks of `main` and `t` are physically different pages, they do not overlap in any way.

**6. The new thread takes the `Box` back** (`Box::from_raw`) and calls `call_once(self)` — the closure
struct moves out of the heap onto the new thread's stack, and the `Box` is freed. The body runs, and
`self.tx` now lives in a frame on `t`'s stack.

### `let mut i = "5".to_string();`

Three different memory regions in one line:

| what | where |
|---|---|
| the literal `"5"` (1 byte) | `.rodata`, static, baked into the executable |
| `&str` = (ptr into .rodata, len=1), 16 bytes | thread `t`'s stack, an intermediate value |
| the buffer `[b'5']`, `cap=1` | **heap** — `alloc(1)`, called from thread `t` |
| the struct `i` = (ptr, cap, len), **24 bytes** | thread `t`'s stack |

`mut` is redundant here — `i` is never mutated, so there will be a warning. `to_string()` copies the
byte out of static memory into a fresh heap allocation, because a `String` has to own its buffer and be
able to grow it.

### `tx.send(i.clone()).unwrap();`

`i.clone()` → a **new** `alloc(1)` in the heap, with `b'5'` copied into it. A new 24-byte struct — a
temporary on `t`'s stack. Two independent buffers, two independent structs.

`send(value)` takes `value` **by value** (a move). Inside: on the first `send`, `List` allocates a
`Block` (32 slots) in the heap, atomically claims `slot[0]` and does a `ptr::write` — a **24-byte
memcpy** of the struct from `t`'s stack into the block's slot.

The key point: only the 24 bytes of the descriptor were copied. The byte `b'5'` in the heap was not
read, not copied, not moved. Only the owner changed: previously the stack temporary was responsible for
freeing it, now the channel is. The temporary's destructor is not run (the value *moved out* of it).

### `tx.send(i).unwrap();`

The same thing, but `i` itself is moved → `slot[1]`. After this line `i` cannot be used.
**That is exactly why the previous line needed `clone()`** — otherwise `i` would have gone into the
channel on the first `send`, and the second would not compile (`use of moved value`).

`drop(i)` at the end of the closure is **not** called — the compiler knows the value was moved, the
drop flag is cleared.

### End of the closure

`self.tx` is dropped (the only thing still alive): an atomic decrement of `senders` → 0 → the channel is
marked **disconnected**. The `Counter` is not freed at this point — `rx` still holds
`receivers == 1`.

### `while ... rx.try_recv()`

On success, `try_recv()` does a `ptr::read` from `slot[0]` — a **24-byte memcpy** from the block in the
heap into `received_value` on `main`'s stack. Ownership of the `[b'5']` buffer passes to `main`. At the
end of the iteration `received_value` goes out of scope → `Drop for String` → `dealloc` of that byte.

## What is worth fixing in this code

**1. The second message is lost.** `received_status = true` exits the loop after the **first** message.
The second `String` stays in `slot[1]`. It does not leak — when `main` exits, `rx` is dropped, both
counters hit zero, and the channel's destructor walks the unread slots, drops the `String`s it finds and
frees the `Block` along with the `Counter`. But logically the value is simply lost.

**2. `Err(_)` merges two different cases.** `try_recv` returns `Err(Empty)` ("nothing sent yet") and
`Err(Disconnected)` ("there are no senders left, nothing to wait for"). Here both print the same thing.
If the thread panicked before `send`, the loop would spin **forever** at 100% CPU:

```rust
match rx.try_recv() {
    Ok(v) => { println!("Received {v}"); received_status = true; }
    Err(mpsc::TryRecvError::Empty) => println!("I am doing some other stuff"),
    Err(mpsc::TryRecvError::Disconnected) => break,
}
```

**3. The busy-wait burns a core.** If there is no useful work to do instead of `println!` — `rx.recv()`
blocks and parks the thread via a futex, which is orders of magnitude cheaper.

**4. No `join`, so there is a race.** `t` is an unused variable (warning). Dropping a `JoinHandle`
**detaches** the thread, it does not wait for it. A realistic scenario: `main` received the first
message, left the loop and finished, dropping `rx`, while thread `t` has not reached the second `send`
yet → `send` returns `Err(SendError)` → `.unwrap()` **panics** in thread `t`. Usually the process dies
first, but you cannot rely on that. `t.join().unwrap()` removes the race.

**5. `let mut i` → `let i`.**

## How this ties into the big picture

`tx.send(i)` costs about a 24-byte memcpy precisely because the byte `b'5'` sits in the process's
**shared** heap — only ownership moves, not the data. Between processes the same code would be
impossible: a pointer inside a `String` means nothing in another address space. Details in
[`os-process-thread.html`](os-process-thread.html) §2.
