fn main() {
    type Book = (String, String, u32);

    let book1: Book = (
        String::from("Rust Programming Langauge"),
        String::from("RUST Community"),
        2020,
    );
    println!(
        "Book name: {}, Author: {}, Year {}",
        book1.0, book1.1, book1.2
    );

    let book2: Book = (
        String::from("Rust by Example"),
        String::from("Steve Klabnik and Carol Nichols"),
        2010,
    );
    println!(
        "Book name: {}, Authors: {}, Year {}",
        book2.0, book2.1, book2.2
    );
}
