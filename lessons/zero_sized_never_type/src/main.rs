// to change to nightly use the command of rustup override set nightly
#![feature(never_type)]

fn unrecoverable_state() -> ! {
    panic!("This function will never return normally with something valid");
}

// fn function() -> Result<i32, String> {}
// fn function_1() -> Result<i32, !> {}
// fn function_2() -> Result<!, String> {}

// fn function() -> Result<NeverType, String> {}
// fn function_1() -> Result<i32, NeverType> {}
enum NeverType {}

fn main() {
    // unrecoverable_state();
    //let x = !;
    //let x = unrecoverable_state();
    let x: !;

    let x = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let x: String = return;
    let counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    };

    //let x = NeverType;
}
