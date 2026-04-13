use array_tool::vec::*;

use visualising_and_organizing_modules::{Category, Customer, Order, Product};
fn main() {
    let product = Product::new(1, String::from("Laptop"), 799.99, Category::Elecronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
    let order = Order::new(1, product, customer, 2);

    let product1 = Product::new(1, String::from("Laptop"), 799.99, Category::Elecronics);
    let product2 = Product::new(2, String::from("T-Shirt"), 20.0, Category::Clothing);
    let product3 = Product::new(3, String::from("Book"), 10.0, Category::Books);

    let set1: Vec<&Product> = vec![&product1, &product2];
    let set2: Vec<&Product> = vec![&product2, &product3];
    let intersection = set1.intersect(set2);
    println!("the intersection is {:?}", intersection);
}
