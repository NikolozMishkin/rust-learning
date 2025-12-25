fn main() {
    //if else
    let num = 40;
    if num < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is 50 or greater");
    }

    //if else if ladder
    let marks = 95;
    //let mut grade = 'N';

    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else
    // last else is optional
    {
        'F'
    };

    //match
    let marks = 95;
    //let mut grde = 'N';

    let grade = match marks {
        90..=100 => {
            //example of how to inclose a block in a match arm
            let x = 10;
            'A'
        }
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    };
}
