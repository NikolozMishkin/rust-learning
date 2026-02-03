fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}

fn main() {
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}
