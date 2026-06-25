// struct ABC;
// use negative_impl::negative_impl;

// #[negative_impl]
// impl !Send for ABC {}

// #[negative_impl]
// impl !Sync for ABC {}

use std::{marker::PhantomData, mem::size_of, rc::Rc};
struct ABC {
    ensuring_no_send_sync: PhantomData<Rc<()>>,
}
fn main() {
    println!("{}", size_of::<ABC>());
}
