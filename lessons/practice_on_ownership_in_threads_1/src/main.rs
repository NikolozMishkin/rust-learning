use std::thread;

fn main() {
    let mut v = vec!["Nouman".to_string()];
    let handle = thread::spawn(move || {
        v.push("Azam".to_string());
    });
}
