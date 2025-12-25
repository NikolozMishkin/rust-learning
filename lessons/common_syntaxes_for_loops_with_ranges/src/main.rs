use std::collections::btree_map::VacantEntry;

fn main() {
    //iterating over a range of values in an ascending order
    // for i in 0..5 {
    //     println!("i: {i}");
    // }

    //iterating over a range of values in a descending order
    //start..end or start..=end is only computed when start < end
    // range is emty -> start >= end

    // for i in (0..=5).rev() {
    //     println!("i: {i}");
    // }

    //iterating with a step size
    for i in (1..=10).step_by(2) {
        println!("i: {i}");
    }

    //for i in 0.0..10.0 {} // not valid
    //0.0 0.2 0.4 0.6 0.8 1.0
    for i in 0..=5 {
        let value = i as f32 * 0.2; //  value = start + i * step_size
    }

    let r = 0..5;
    //count(), contains() etc can be used on ranges

    let pairs = vec![(1, "one"), (2, "two")];

    for (num, word) in pairs {
        println!("{} is written as {}", num, word);
    }
}
