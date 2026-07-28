# Lesson notes: memory, threads, processes

A walkthrough of what happens under the hood of `thread::spawn` + `mpsc::channel` in
[`../src/main.rs`](../src/main.rs).

| File | About |
|---|---|
| [`stack-heap-walkthrough.md`](stack-heap-walkthrough.md) | Line-by-line walkthrough of Example 2: what lives on the stack, what in the heap, what in `.rodata`, how the closure moves into the heap and back out. Plus 5 remarks on the lesson's code. |
| [`os-process-thread.html`](os-process-thread.html) | The big picture: OS → program → process → thread. What is private and what is shared, where stack sizes come from, thread and process limits on Linux / macOS / Windows / iOS / Android. §6 is a Chrome case study: why one program turns into 49 processes and 1232 threads. Open in a browser. |
| [`experiments/`](experiments/) | The programs that produced every number in these documents. |

## Experiments

```sh
cd experiments && ./run.sh
```

| Program | What it shows |
|---|---|
| `stack_probe.rs` | Actual stack sizes: main, `spawn()` default, `stack_size()`. Via `pthread_get_stacksize_np`. |
| `max_threads.rs` | Spawns threads until `EAGAIN` at two different stack sizes. Shows that on macOS the limit is a kernel counter, not memory. |
| `heap_isolation.rs` | One `Box`: in another thread — same address and the write is visible; after `fork()` — same address, but its own copy. |
| `parallelism.rs` | `available_parallelism()` — how many threads actually run at the same time. |
| `pthread_default.c` | The stack size macOS itself hands out, with Rust out of the picture (512 KiB versus 2 MiB in std). |

`max_threads.rs` creates ~12k threads — the machine will be busy for a second or two. There is no
fork bomb here and never will be: the process limit is taken from `sysctl`, not measured.

## Measured values

Taken on macOS 14.5 / Apple M3 Max / rustc 1.97.1. On a different machine and OS the numbers will
differ — §8 of the HTML document has commands for checking them yourself.

```
main stack                7.98 MiB   (= ulimit -s 8176 KiB)
spawn() default stack     2.01 MiB   (an std constant, not an OS value)
pthread stack without Rust 512 KiB   (the macOS value)
max threads per process   12287      (kern.num_taskthreads 12288 − main)
max processes per user    8000       (kern.maxprocperuid)
cores                     16         (12 performance + 4 efficiency)

Sender<String>  16 bytes   String        24 bytes
JoinHandle<()>  24 bytes   closure       16 bytes
```

## Why notes/, not src/

The lesson is a workspace crate. Anything that lands in `src/` will be picked up by cargo as part of
`message_passing_through_channels_part_1`. The files here live outside `src/`, so `cargo build` does
not see them, and the experiments are built by hand with `rustc`.
