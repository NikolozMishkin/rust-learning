//! #Online Bussiness
//! This is a rust library for online store
#[derive(PartialEq, Debug)]
/// Struct for storing product related information.
pub struct Product {
    id: u64,
    pub name: String,
    price: f64,
    category: Category,
}

mod category;
pub use category::Category;

impl Product {
    /// # Example
    /// ```
    /// use visualising_and_organizing_modules::Category;
    /// use visualising_and_organizing_modules::Product;
    /// let some_product = Product::new(1, String::from("Laptop"),799.99,Category::Elecronics);
    /// assert_eq!(some_product.name,String::from("Laptop"));
    /// ```
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            id,
            name,
            price,
            category,
        }
    }
}

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
