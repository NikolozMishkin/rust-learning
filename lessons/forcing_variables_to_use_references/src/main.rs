fn some_fn(ref s: String) {
    let r = s;
}
fn main() {
    let tuple = (String::from("Nouman"), String::from("Azam"));

    let x = &tuple.0;

    // ref variable: SomeType
    let ref x: i32 = 5;

    let s1 = String::from("");
    let ref s = s1;

    some_fn(s1);
    // println!("{s1}");
}
