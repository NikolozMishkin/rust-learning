use std::mem::size_of;
trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Struct1<'a> {
    my_str: &'a str,
    my_string: String,
    my_vec: Vec<i32>,
}

struct Struct2 {
    my_u8: u8,
    my_string1: String,
    my_vec1: Vec<i32>,
    my_string2: String,
    my_vec2: Vec<i32>,
    my_string3: String,
    my_vec3: Vec<i32>,
    my_string4: String,
    my_vec4: Vec<i32>,
    my_string5: String,
    my_vec5: Vec<i32>,
    my_string6: String,
    my_vec6: Vec<i32>,
    my_string7: String,
    my_vec7: Vec<i32>,
    my_string8: String,
    my_vec8: Vec<i32>,
    my_string9: String,
    my_vec9: Vec<i32>,
    my_string10: String,
    my_vec10: Vec<i32>,
    my_string11: String,
    my_vec11: Vec<i32>,
    my_string12: String,
    my_vec12: Vec<i32>,
    my_string13: String,
    my_vec13: Vec<i32>,
    my_string14: String,
    my_vec14: Vec<i32>,
    my_string15: String,
    my_vec15: Vec<i32>,
    my_string16: String,
    my_vec16: Vec<i32>,
    my_string17: String,
    my_vec17: Vec<i32>,
    my_string18: String,
    my_vec18: Vec<i32>,
    my_string19: String,
    my_vec19: Vec<i32>,
}

fn main() {
    println!(
        "Size of a reference to sized type: {}",
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to unsized type: {}",
        size_of::<&[i32]>()
    );

    let num_1: &[i32; 3] = &[10, 12, 30];
    let num_2: &[i32] = &[10, 12, 30];

    let mut sum = 0;
    for num in num_1 {
        sum += num;
    }

    for num in num_2 {
        sum += num;
    }
    println!("Size of &[String; 3] is: {}", size_of::<&[String; 3]>());
    println!("Size of &[String] is: {}", size_of::<&[String]>());
    println!("Size of &[i32; 3] is: {}", size_of::<&[i32; 3]>());
    println!("Size of &[i32] is: {}", size_of::<&[i32]>());
    let a: Vec<i32> = vec![1, 2, 3];
    println!("Size of Vec<i32> is: {}", size_of::<Vec<i32>>());
    println!("Size of &Vec<i32> is: {}", size_of::<&Vec<i32>>());
    let mut num_3: [i32; 3] = [10, 12, 30];
    println!("Size of [i32; 10] is: {}", size_of::<[i32; 10]>());

    println!("Size of &Cricle is: {}", size_of::<&Circle>());
    println!("Size of &Rectangle is: {}", size_of::<&Rectangle>());
    println!("Size of Point is: {}", size_of::<Point>());
    println!("Size of String is: {}", size_of::<String>());
    println!("Size of &String is: {}", size_of::<&String>());
    println!("Size of &str is: {}", size_of::<&str>());
    println!("Size of u64 is: {}", size_of::<u64>());
    println!("Size of &u64 is: {}", size_of::<&u64>());
    println!("Size of u32 is: {}", size_of::<u32>());
    println!("Size of &u32 is: {}", size_of::<&u32>());
    println!("Size of u8 is: {}", size_of::<u8>());
    println!("Size of &u8 is: {}", size_of::<&u8>());
    println!("Size of bool is: {}", size_of::<bool>());
    println!("Size of &bool is: {}", size_of::<&bool>());
    println!("Size of &Point is: {}", size_of::<&Point>());
    println!("Size of Struct1 is: {}", size_of::<Struct1>());
    println!("Size of &Struct1 is: {}", size_of::<&Struct1>());
    println!("Size of Struct2 is: {}", size_of::<Struct2>());
    println!("Size of &Struct2 is: {}", size_of::<&Struct2>());
    println!("Size of &Shape is: {}", size_of::<&dyn Shape>());
}
