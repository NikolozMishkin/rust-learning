fn main() {
    let mut some_str = String::from("I am String");
    let ref1 = &some_str;
    println!("{ref1}"); // move this line only
    let ref2 = &mut some_str;
    ref2.push_str(" additional information");
    println!("{ref2}");
}
