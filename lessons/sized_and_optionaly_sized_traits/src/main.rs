// include as dependency in the cargo.toml file  negative-impl = "0.1.4"

use negative_impl::negative_impl;
struct ABC;

#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}

// #[negative_impl]
// impl !Sized for ABC {}

//fn some_fn<T>(t: T) {}
//fn some_fn<T: Sized>(t: T) {}
fn some_fn<T: ?Sized>(t: &T) {}

fn main() {
    let x: i32 = Default::default();
    let y = Clone::clone(&x);
}
