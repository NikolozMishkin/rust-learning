#!/bin/sh
# Builds and runs all the experiments. Artifacts go to ./build (no .gitignore entry needed, the
# directory is created and removed right here).
set -e
cd "$(dirname "$0")"
mkdir -p build

hdr() { printf '\n\033[1m── %s\033[0m\n' "$1"; }

hdr "Stack sizes (Rust)"
rustc -O stack_probe.rs -o build/stack_probe
./build/stack_probe

hdr "The stack size the OS itself hands out (no Rust)"
cc -O2 pthread_default.c -o build/pthread_default
./build/pthread_default

hdr "The same program with RUST_MIN_STACK overridden to 512 KiB"
RUST_MIN_STACK=$((512 * 1024)) ./build/stack_probe

hdr "How many threads actually run at the same time"
rustc -O parallelism.rs -o build/parallelism
./build/parallelism

hdr "Heap isolation: threads versus processes"
rustc -O heap_isolation.rs -o build/heap_isolation
./build/heap_isolation

hdr "Maximum number of threads (~12k, takes a second or two)"
rustc -O max_threads.rs -o build/max_threads
./build/max_threads

hdr "What the kernel says"
if command -v sysctl >/dev/null 2>&1 && sysctl -n kern.num_taskthreads >/dev/null 2>&1; then
  sysctl kern.num_taskthreads kern.num_threads kern.maxproc kern.maxprocperuid
  printf 'ulimit -s: %s KiB\nulimit -u: %s\n' "$(ulimit -s)" "$(ulimit -u)"
else
  # Linux
  for f in kernel/threads-max kernel/pid_max vm/max_map_count; do
    [ -r "/proc/sys/$f" ] && printf '%s: %s\n' "$f" "$(cat "/proc/sys/$f")"
  done
  printf 'ulimit -s: %s KiB\nulimit -u: %s\n' "$(ulimit -s)" "$(ulimit -u)"
fi

printf '\nDone. Binaries are in ./build, remove them with: rm -rf build\n'
