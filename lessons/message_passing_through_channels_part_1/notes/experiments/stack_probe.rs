use std::thread;
extern "C" {
    fn pthread_self() -> *mut u8;
    fn pthread_get_stacksize_np(t: *mut u8) -> usize;
}
fn stack_mib(label: &str) {
    unsafe {
        let sz = pthread_get_stacksize_np(pthread_self());
        println!("{label}: {} bytes = {:.2} MiB", sz, sz as f64 / 1048576.0);
    }
}
fn main() {
    stack_mib("main");
    thread::spawn(|| stack_mib("spawn() default"))
        .join()
        .unwrap();
    thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| stack_mib("stack_size(64 KiB)"))
        .unwrap()
        .join()
        .unwrap();
}
