use std::vec;

use std::collections::HashMap;

fn main() {
    //solution 1 :
    let words = vec!["apple", "banana", "apple", "orange", "banana"];
    let counts = vec![5, 2, 15, 5];

    //solution 2 :
    let word_counts = vec![("apple", 5), ("banana", 2), ("orange", 15), ("banana", 5)];
    let target_word = word_counts.contains(&("apple", 5));

    let mut word_counts: HashMap<&str, u8> = HashMap::new();
    word_counts.insert("apple", 5);
    word_counts.insert("banana", 2);
    word_counts.insert("orange", 15);
    word_counts.insert("lime", 5);
    println!("Hashmap: {:?}", word_counts);

    let has_programing_key = word_counts.contains_key("banana");
    let programing_value = word_counts.get("banana");

    let new_entry = word_counts.entry("mango").or_insert(0);
    println!("Hashmap: {:?}", word_counts);

    let new_entry = word_counts.entry("banana").or_insert(4);
    println!("Hashmap: {:?}", word_counts);
}
