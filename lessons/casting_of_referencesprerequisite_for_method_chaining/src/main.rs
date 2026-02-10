use std::io::StderrLock;

fn main() {
    //castiong between references
    let x = 5;
    let y: f32 = x as f32;

    //casting immutable reference -> mutable reference(not allowed)
    // let data = 42;
    // let immutable_ref = &data;
    // let mytable_ref = immutable_ref as &mut i32; // not allowed
    // &T -> &mut T (not allowed)

    //casting mutable referece -> immutable reference (allowed)
    let mut data = 42;
    let mutable_ref = &mut data;
    let immutable_ref = mutable_ref as &i32; // allowed
    // *mutable_ref = 43;
    println!("{:?} {:?}", mutable_ref, immutable_ref);

    //another name : reborrowing

    //assignment of references
    let mut str = String::from("");
    let ref_str_1 = &mut str;
    let ref_str_2 = ref_str_1;
    // println!("{ref_str_1}");
}
