struct DataHolder<'a> {
    data: Vec<&'a str>,
}

impl<'a> DataHolder<'a> {
    fn initialize() -> Self {
        DataHolder { data: Vec::new() }
    }

    fn add_data(&mut self, item: &'a str) {
        self.data.push(item);
    }

    fn extract_data_containing_substring(&mut self, sub: &str) -> &str {
        for i in 0..self.data.len() {
            if self.data[i].contains(sub) {
                return self.data.remove(i);
            }
        }
        panic!("Data containing substring not found");
    }
}

fn main() {
    let mut my_data = DataHolder::initialize();
    my_data.add_data("Apple");
    my_data.add_data("Banana");
    my_data.add_data("Cherry");
    my_data.add_data("Date");
    let extracted = my_data.extract_data_containing_substring("na");
    println!("Extracted: {}", extracted);
    assert_eq!(my_data.data.len(), 3);
}
