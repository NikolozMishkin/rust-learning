fn main() {
    //commens

    //the current line is a comment line
    //this is another comment line

    /* this is a
    multi-line
    comment
    */

    //more on printing
    print!("This is a print command");
    print!("This is going to be printed on the same line");

    // escape sequences
    // \escape sequence characters

    /*
    \n :newline character
    \t : tab space
    \r : carriage return
    \" : double quote
    \\ : backward slash 
    */

    println!("\nThis is printed after a new line");
    println!("\tThis is printed after a tab space");
    println!("This will be overwritten \r This text will appear on the screen");
    println!("Prints double quote: \", print backward slash: \\");

    //positional arguments
    println!(
        "i am doing {2} from {1} year and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is fun, {language} is awesome",
        language = "Rust"
    );
}
