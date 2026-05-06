trait Addition<Rhs, Output> {
    // type Rhs;
    // type Output;
    fn add(self, rhs: Rhs) -> Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        println!("  [impl Addition<Point, Point>] called: ({},{}) + ({},{})", self.x, self.y, rhs.x, rhs.y);
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        println!("  [impl Addition<i32, Point>] called: ({},{}) + scalar {}", self.x, self.y, rhs);
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        println!("  [impl Addition<Point, Line>] called: start=({},{}) end=({},{})", self.x, self.y, rhs.x, rhs.y);
        Line {
            start: self,
            end: rhs,
        }
    }
}

fn main() {
    // --- Case 1: Point + Point = Point ---
    // Rust picks the impl Addition<Point, Point> for Point
    // because the return type is annotated as Point.
    println!("=== Case 1: Point + Point => Point ===");
    println!("KEY IDEA: generic Rhs=Point, Output=Point => coordinates are summed");
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3: Point = p1.add(p2);
    println!("  result: p3 = ({}, {})", p3.x, p3.y);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    println!();

    // --- Case 2: Point + i32 = Point ---
    // Rust picks the impl Addition<i32, Point> for Point
    // because the right-hand side is an i32 scalar.
    println!("=== Case 2: Point + i32 => Point ===");
    println!("KEY IDEA: generic Rhs=i32, Output=Point => scalar added to both x and y");
    let p1 = Point { x: 1, y: 1 };
    let p3 = p1.add(2);
    println!("  result: p3 = ({}, {})", p3.x, p3.y);
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
    println!();

    // --- Case 3: Point + Point = Line ---
    // Rust picks the impl Addition<Point, Line> for Point
    // because the return type is annotated as Line.
    // Same Rhs type as Case 1, but DIFFERENT Output => different impl chosen!
    println!("=== Case 3: Point + Point => Line ===");
    println!("KEY IDEA: generic Rhs=Point, Output=Line => points become start/end of a line");
    println!("NOTICE: Rhs is Point just like Case 1, but Output=Line picks a DIFFERENT impl");
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let line: Line = p1.add(p2);
    println!("  result: line.start=({},{}) line.end=({},{})", line.start.x, line.start.y, line.end.x, line.end.y);
    assert!(line.start.x == 1 && line.start.y == 1 && line.end.x == 2 && line.end.y == 2);
    println!();

    println!("=== WHY generic types on the trait (not associated types)? ===");
    println!("Associated types allow only ONE impl per type.");
    println!("Generic type params on the trait allow MULTIPLE impls (different Rhs/Output combos).");
    println!("Point needs 3 different Addition impls, so generics on the trait are the right choice.");
}
