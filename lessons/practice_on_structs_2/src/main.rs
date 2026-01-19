struct Book {
    title: String,
}

fn main() {
    let some_book = Book {
        title: String::from("The Rust Programming Language"),
    };
    print_book(some_book.title.clone()); // Fix this line
    println!("Book name: {}", some_book.title);
}

fn print_book(book_name: String) {
    println!("Book: {}", book_name);
}
