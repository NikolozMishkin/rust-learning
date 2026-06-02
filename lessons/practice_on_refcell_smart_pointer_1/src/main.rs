use std::cell::RefCell;
fn main() {
    let data: RefCell<Option<i32>> = RefCell::new(Some(42));
    *data.borrow_mut() = None;

    if data.borrow().is_some() {
        println!("Final value: {:?}", data.borrow());
    } else {
        println!("No value present.");
    }
}
