// fn print(s: &str) {}
// fn print<'a>(s: &'a str) {}

// fn debug(v: usize, s: &str) {}
// fn debug<'a>(v: usize, s: &'a str) {}

// fn substr(s: &str, until: usize) -> &str {}
// fn substr<'a>(s: &'a str, until: usize) -> &'a str;

// fn get_str() -> &str {}

// fn frob(s: &str, t: &str) -> &str{}

// fn get_mut(&mut self) -> &mut T;
// fn get_mut<'a>(&'a mut self) -> &'a mut T;

// fn new(buf: &mut [u8]) -> BufWriter;
// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>;

fn main() {
    println!("HI")
}
