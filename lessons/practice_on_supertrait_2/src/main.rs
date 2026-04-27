trait Size {
    fn compute_size(&self) -> u16;
}

trait Printable {
    fn size_to_str(&self) -> String;
}

trait Comparable: Size + Printable {
    fn print_greater(a: &Self, b: &Self) {
        // Please note that Self on the line above means, that type which will be implementing the trait

        let item1 = a.compute_size();
        let item2 = b.compute_size();
        if item1 > item2 {
            println!("{} is greater than {}", a.size_to_str(), b.size_to_str());
        } else if item2 > item1 {
            println!("{} is greater than {}", b.size_to_str(), a.size_to_str());
        } else {
            println!("Both sizes are {}", a.size_to_str());
        }
    }
}

struct Book {
    page: u16,
}

impl Size for Book {
    fn compute_size(&self) -> u16 {
        self.page
    }
}

impl Printable for Book {
    fn size_to_str(&self) -> String {
        format!("Book having {} pages", self.page)
    }
}

impl Comparable for Book {}

fn main() {
    let book_1 = Book { page: 50 };
    let book_2 = Book { page: 450 };
    Comparable::print_greater(&book_1, &book_2);
}
