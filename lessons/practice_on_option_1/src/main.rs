fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
}
