use std::thread;
extern "C" {
    fn fork() -> i32;
    fn waitpid(p: i32, s: *mut i32, o: i32) -> i32;
}

fn main() {
    // --- THREADS: one heap per process ---
    let b = Box::new(111u64);
    let addr = &*b as *const u64 as usize;
    println!("[threads] main:  address 0x{addr:x}, value {}", *b);
    thread::spawn(move || {
        let mut b = b; // the Box moved into another thread
        let a2 = &*b as *const u64 as usize;
        *b = 222; // writing to the SAME address
        println!(
            "[threads] child: address 0x{a2:x} -> same one? {}, wrote {}",
            a2 == addr,
            *b
        );
    })
    .join()
    .unwrap();

    // --- PROCESSES: each one has its own heap ---
    let mut p = Box::new(111u64);
    let addr = &*p as *const u64 as usize;
    unsafe {
        if fork() == 0 {
            *p = 999; // the child writes into ITS OWN copy
            println!(
                "[processes] child  pid={}: 0x{:x} = {}",
                std::process::id(),
                &*p as *const u64 as usize,
                *p
            );
            std::process::exit(0);
        }
        waitpid(-1, std::ptr::null_mut(), 0);
    }
    println!(
        "[processes] parent pid={}: 0x{addr:x} = {}  <- the child's write is NOT visible",
        std::process::id(),
        *p
    );
}
