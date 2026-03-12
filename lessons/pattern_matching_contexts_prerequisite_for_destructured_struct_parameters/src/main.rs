//value: the thing you are trying to match against
//pattern: the shape or structure you are matching
fn main() {
    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something Else"),
    };

    //value: x
    //pattern: 1,2,3,_

    //2. if let
    let x = 3;
    if let 5 = x {
        // if x == 65
        println!("matched five");
    }

    //value: x
    //pattern: 5

    if let x = 5 {
        //let x = 5
        println!("this always run");
        println!("x: inner {x}");
    }

    println!("x: outer {x}");

    //binding pattern
    //value: concrete value
    //pattern: variable

    //3.while let
    let number = vec![1, 2, 3, 4, 2, 0];
    let mut i = 0;

    while let 2 = number[i] {
        println!("found a value 2 at inde: {}", i);
        i += 1;
    }

    //value: number[i]
    //pattern: 2

    //4. let binding
    let (a, b) = (10, 20);

    //value: (10,20)
    //pattern: (a,b)
    //type: (i32,i32)

    //5,function parametrs
}

fn print_coords((x, y): (i32, i32)) {
    println!("x: {x}, y: {y}");
}
