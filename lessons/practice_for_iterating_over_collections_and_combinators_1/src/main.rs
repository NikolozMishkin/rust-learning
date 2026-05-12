use std::ops::Deref;

fn main() {
    let mut vec_1 = vec![4, 5, 6, 9, 8];
    for i in vec_1.iter_mut() {
        // fix this line by making a call to relevant function
        *i = *i * 2;
        // *i = i.deref() * 2;
    }
    println!("{:?}", vec_1);
}
