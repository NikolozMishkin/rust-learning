struct Point {
    x: i32,
    y: i32,
}

fn print_coord(Point { x, y }: Point) {
    //value: p or Point {x:5,y:7}
    //pattern: Point {x,y}
    println!("x: {x}, y: {y}");
}

fn print_coord_ignoring_other_parts(Point { x, .. }: Point) {
    //value: p or Point {x:5,y:7}
    //pattern: Point {x,y}
    println!("x: {x}");
}

fn main() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x: 0, y } => println!("on the y-axis at: {y}"),
        Point { x, y: 0 } => println!("on the x-axis at: {x}"),
        Point { x, y } => println!("at point ({x} {y})"),
    }

    let x = 5;
    if let x = 5 {}
    //value: 5
    //pattern: x

    //first arm
    //value: p or Point{x:0, y:7}
    //patern: Point {x:0, y}
    let p = Point { x: 5, y: 6 };
    print_coord(p);
}
