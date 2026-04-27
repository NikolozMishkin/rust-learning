trait SquareRoot {
    fn square_root(&self) -> Self;
}

trait Displayable {
    fn to_display_string(&self) -> String;
}

fn get_square_root_str(input: impl SquareRoot + Displayable) -> String {
    // make changes to this line only
    let squared_rooted = input.square_root();
    squared_rooted.to_display_string()
}

impl SquareRoot for f64 {
    fn square_root(&self) -> Self {
        self.sqrt()
    }
}

impl Displayable for f64 {
    fn to_display_string(&self) -> String {
        format!("{:.2}", self)
    }
}

fn main() {
    let num = 9.0;
    let mut msg = format!("{num} square rooted is ");
    msg.push_str(&get_square_root_str(num));
    println!("{msg}");
}
