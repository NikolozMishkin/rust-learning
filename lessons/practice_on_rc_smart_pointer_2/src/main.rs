use std::rc::Rc;

struct File {}

struct User {
    file: Rc<File>,
}

fn main() {
    let txt_file = Rc::new(File {});

    let user_1 = User {
        file: Rc::clone(&txt_file),
    };

    let user_2 = User {
        file: Rc::clone(&txt_file),
    };
}
