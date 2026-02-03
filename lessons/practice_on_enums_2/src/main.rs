struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

impl Item {
    fn display_item_info(&self) {
        println!(
            "ID: {}, Tittle: {}, Year: {}, Type: {:?}",
            self.id, self.title, self.year, self.type_
        )
    }
}

fn main() {
    Item {
        id: 1,
        title: String::from("Rust Book"),
        year: 2024,
        type_: ItemType::Book,
    }
    .display_item_info();
    Item {
        id: 2,
        title: String::from("SuperMan Magazine"),
        year: 2003,
        type_: ItemType::Magazine,
    }
    .display_item_info();
}
