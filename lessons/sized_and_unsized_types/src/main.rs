use std::mem::size_of;
trait Some_trait {}
fn main() {
    // Sized Types
    println!("i32 size is: {}", size_of::<i32>());
    println!("(i32,i32) size is: {}", size_of::<(i32, i32)>());
    println!("[i32: 3] size is: {}", size_of::<[i32; 3]>());

    struct Point {
        x: bool,
        y: i64,
    }
    println!("Struct size is: {}", size_of::<Point>());
    println!("i32 reference is: {}", size_of::<&i32>());
    println!("i32 mutable reference is: {}", size_of::<&mut i32>());
    println!("Machine word size is: {}", size_of::<&()>());
    println!("Box<i32> is: {}", size_of::<Box<i32>>());
    println!("fn(i32) -> i32 is: {}", size_of::<fn(i32) -> i32>());

    // Unsized Types
    println!("[i32] size is: {}", size_of::<&[i32]>());
    let a: [i32; 3];
    //println!("str size is: {}", size_of::<str>());
    println!(
        "The size of the trait object is: {}",
        size_of::<&dyn Some_trait>()
    );
}
