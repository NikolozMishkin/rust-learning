struct Wrapper {
    data: String,
}

fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
    wrapper.data = String::from("Modified");
    wrapper
}

fn main() {
    let original_wrapper = Box::new(Wrapper {
        data: String::from("Original"),
    });
    let modified_wrapper = modify_data(original_wrapper);
}
