struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

trait OtherTrait {}
impl OtherTrait for Square {}
impl OtherTrait for Rectangle {}

trait SomeOtherTrait {}
impl SomeOtherTrait for Square {}
impl SomeOtherTrait for Rectangle {}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect: f32 = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect: f32 = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square: f32 = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}

fn shape_properties<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}

fn returns_shape() -> impl Shape {
    let sq: Square = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    sq
}

struct Circle {
    radius: f32,
}

fn main() {
    let r1: Rectangle = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: String::from("Red"),
    };

    let s1: Square = Square {
        side: 3.2,
        line_width: 1,
        color: String::from("Red"),
    };

    let c1 = Circle { radius: 5.0 };

    shape_properties(r1);
    shape_properties(s1);
    // shape_properties(c1);
}
