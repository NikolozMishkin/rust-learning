fn main() {
    let f = String::from("hello");

    let s = "hello"; // &'static str
    let slice = &s[0..2]; // &str
    println!("f = {}, s = {}, slice = {}", f, s, slice);
}

// String is a growable, heap-allocated data structure whereas &str is an immutable reference to a string slice.
