struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Container<T> {
        // something wrong here
        Container { value }
    }
}
impl Container<i32> {
    fn display(&self) {
        println!("The value inside the container is: {}", self.value);
    }

    fn create(value: i32) -> Container<i32> {
        Container { value }
    }
}

fn main() {}
