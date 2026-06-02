use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let x_ref1 = x.borrow();
    let x_ref2 = x.borrow();
    println!("x_ref1: {}, x_ref2: {}", x_ref1, x_ref2);
    drop(x_ref1);
    drop(x_ref2);

    let mut x_ref3 = x.borrow_mut();
    *x_ref3 = 6;
    drop(x_ref3);

    println!("Stored value: {:?}", x);
}
