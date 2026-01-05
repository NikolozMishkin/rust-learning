fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut temp;

    while !my_vec.is_empty() {
        temp = my_vec.clone(); // Something wrong on this line
        println!("Elements in temporary vector are: {:?}", temp);
        println!("Popped element: {}", my_vec.pop().unwrap()); // pop() is used to remove an element from the vec. The unwrap() is used to return the value inside the Some() variant
    }
}
