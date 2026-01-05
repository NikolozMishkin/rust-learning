fn main() {
    let s1 = String::from("World");
    {
        let s2 = s1;
    }
    //println!("s2 is {}", s2);
    let a = 15;
    let b = a;
    println!("{a}")
}
