use std::vec;

fn main() {
    let mut vec_1 = vec![4, 5, 6];
    let ref1 = &vec_1;
    let ref2 = &vec_1;
    print!("ref_1{}, ref_2{}", ref1[0], ref2[0]);
    let ref32 = &mut vec_1;

    let vec_2 = {
        let vec_3 = vec![1, 2, 3];
        //&vec_3
    };
}
