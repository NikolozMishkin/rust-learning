struct User {
    name: String,
    id: u32,
}

fn take_and_return<T>(input: T) -> T {
    // this line needs updation
    input
}

fn main() {
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _user2 = take_and_return(user1);

    let str1 = String::from("Hello folks");
    let _str2 = take_and_return(str1); // we want this to compile
}
