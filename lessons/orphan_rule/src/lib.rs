pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub trait Printable {
    fn print(&self);
}

pub trait Addition<T, U> {
    fn add(self, rhs: T) -> U;
}
