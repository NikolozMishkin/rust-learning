use std::num::ParseIntError;

fn read_number(input: &str) -> Result<i32, ParseIntError> {
    // match input.trim().parse::<i32>() {
    //     Ok(num) => Ok(num),
    //     Err(e) => Err(e),
    // }

    /*
    let num = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    */
    let num = input.trim().parse::<i32>()?;
    Ok(num)
}

fn extract_username(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    let username = email.get(0..at_pos)?;
    Some(username)
}
fn main() {}
