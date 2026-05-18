fn identity(a: &i32) -> &i32 {
    a
}

fn main() {
    let mut x_ref: Option<&i32> = None;
    {
        let x = 7;
        x_ref = Some(identity(&x));
        assert_eq!(*x_ref.unwrap(), 7); // Issue at this line
    }
}
