#![feature(fn_traits, unboxed_closures)]

struct User {
    name: String,
    age: u8,
    salary: u32,
}

// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

struct ValidateUserSimplee<'a> {
    banned_user: &'a String,
}

impl<'a> ValidateUserSimplee<'a> {
    fn new(banned_user: &'a String) -> Self {
        Self { banned_user }
    }
}

impl<'a> FnOnce<(&str,)> for ValidateUserSimplee<'a> {
    type Output = bool;
    extern "rust-call" fn call_once(self, (name,): (&str,)) -> bool {
        name.len() != 0 && name != self.banned_user
    }
}

impl<'a> FnMut<(&str,)> for ValidateUserSimplee<'a> {
    extern "rust-call" fn call_mut(&mut self, (name,): (&str,)) -> bool {
        name.len() != 0 && name != self.banned_user
    }
}

impl<'a> Fn<(&str,)> for ValidateUserSimplee<'a> {
    extern "rust-call" fn call(&self, (name,): (&str,)) -> bool {
        name.len() != 0 && name != self.banned_user
    }
}

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: FnOnce(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}
fn main() {
    let person_1 = User {
        name: String::from("someone"),
        age: 35,
        salary: 40_000,
    };

    let mut banned_user = String::from("banned user");
    let validate_user_simple = move |name: &str| {
        let banned_user_name = &banned_user;
        name.len() != 0 && name != banned_user_name
    };
    validate_user_simple("someone");
    println!("validate_user_simple createad");
    //println!("{banned_user}");

    let validate_user_simple = ValidateUserSimplee::new(&banned_user);

    validate_user_simple("someone");

    let validate_user_advance = |age: u8| age >= 30;
    println!(
        "User validity {}",
        is_valid_user(
            &person_1.name,
            person_1.age,
            validate_user_simple,
            validate_user_advance
        )
    );
}
