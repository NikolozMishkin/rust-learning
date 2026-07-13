use std::mem::size_of;
trait Shape {
    fn print(&self);
    fn my_super_fn(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
    fn my_super_fn(&self) {
        println!("This is a super function!");
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
    fn my_super_fn(&self) {
        println!("This is a super function!");
    }
}

struct Point {
    x: i32, //4
    y: i32, //4
}

struct Struct1<'a> {
    my_str: &'a str,   //16
    my_string: String, //24
    my_vec: Vec<i32>,  //24
}

struct Struct2 {
    my_u8: u8,           //1
    my_string1: String,  //24
    my_vec1: Vec<i32>,   //24
    my_string2: String,  //24
    my_vec2: Vec<i32>,   //24
    my_string3: String,  //24
    my_vec3: Vec<i32>,   //24
    my_string4: String,  //24
    my_vec4: Vec<i32>,   //24
    my_string5: String,  //24
    my_vec5: Vec<i32>,   //24
    my_string6: String,  //24
    my_vec6: Vec<i32>,   //24
    my_string7: String,  //24
    my_vec7: Vec<i32>,   //24
    my_string8: String,  //24
    my_vec8: Vec<i32>,   //24
    my_string9: String,  //24
    my_vec9: Vec<i32>,   //24
    my_string10: String, //24
    my_vec10: Vec<i32>,  //24
    my_string11: String, //24
    my_vec11: Vec<i32>,  //24
    my_string12: String, //24
    my_vec12: Vec<i32>,  //24
    my_string13: String, //24
    my_vec13: Vec<i32>,  //24
    my_string14: String, //24
    my_vec14: Vec<i32>,  //24
    my_string15: String, //24
    my_vec15: Vec<i32>,  //24
    my_string16: String, //24
    my_vec16: Vec<i32>,  //24
    my_string17: String, //24
    my_vec17: Vec<i32>,  //24
    my_string18: String, //24
    my_vec18: Vec<i32>,  //24
    my_string19: String, //24
    my_vec19: Vec<i32>,  //24
}

struct Struct3 {
    struct2: Struct2,
    bool1: bool,
    bool2: bool,
}
struct Struct4 {
    // bool1: bool,
    // bool2: bool,
    bool3: bool,
    i32_1: i16,
}

fn main() {
    println!(
        "Size of a reference to sized type: {}", //- 12
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to unsized type: {}", //- 16
        size_of::<&[i32]>()
    );

    let num_1: &[i32; 3] = &[10, 12, 30]; //8
    let num_2: &[i32] = &[10, 12, 30]; //16

    let my_point = Point { x: 10, y: 20 };

    let mut sum = 0;
    for num in num_1 {
        sum += num;
    }

    for num in num_2 {
        sum += num;
    }
    println!("Size of &[i32; 3] is: {}", size_of::<&[i32; 3]>()); //8
    println!("Size of [i32; 3] is: {}", size_of::<[i32; 3]>()); //12
    println!("Size of &[String; 3] is: {}", size_of::<&[String; 3]>()); //8
    println!("Size of &[String] is: {}", size_of::<&[String]>()); //16
    println!("Size of &[i32; 3] is: {}", size_of::<&[i32; 3]>()); //8
    println!("Size of &[i32] is: {}", size_of::<&[i32]>()); //16
    let a: Vec<i32> = vec![1, 2, 3];
    println!("Size of Vec<i32> is: {}", size_of::<Vec<i32>>()); //24
    println!("Size of &Vec<i32> is: {}", size_of::<&Vec<i32>>()); //8
    let mut num_3: [i32; 3] = [10, 12, 30];
    println!("Size of [i32; 10] is: {}", size_of::<[i32; 10]>()); //40

    println!("Size of &Cricle is: {}", size_of::<&Circle>()); //8
    println!("Size of &Rectangle is: {}", size_of::<&Rectangle>()); //8
    println!("Size of Point is: {}", size_of::<Point>()); // 8
    println!("Size of String is: {}", size_of::<String>()); //24
    println!("Size of &String is: {}", size_of::<&String>()); //8
    println!("Size of &str is: {}", size_of::<&str>()); //16
    println!("Size of u64 is: {}", size_of::<u64>()); //8
    println!("Size of &u64 is: {}", size_of::<&u64>()); //8
    println!("Size of u32 is: {}", size_of::<u32>()); //4
    println!("Size of &u32 is: {}", size_of::<&u32>()); //8
    println!("Size of u8 is: {}", size_of::<u8>()); //1
    println!("Size of &u8 is: {}", size_of::<&u8>()); //8
    println!("Size of bool is: {}", size_of::<bool>()); //1
    println!("Size of &bool is: {}", size_of::<&bool>()); //8
    println!("Size of &Point is: {}", size_of::<&Point>()); //8
    println!("Size of Struct1 is: {}", size_of::<Struct1>()); //64
    println!("Size of &Struct1 is: {}", size_of::<&Struct1>()); //8
    println!("Size of Struct2 is: {}", size_of::<Struct2>()); //920
    println!("Size of Struct3 is: {}", size_of::<Struct3>()); //928
    println!("Size of Struct4 is: {}", size_of::<Struct4>()); //2
    println!("Size of &Struct2 is: {}", size_of::<&Struct2>()); //8
    println!("Size of &Shape is: {}", size_of::<&dyn Shape>()); //16
    let link: &dyn Shape = &Circle;
    link.my_super_fn();
}
