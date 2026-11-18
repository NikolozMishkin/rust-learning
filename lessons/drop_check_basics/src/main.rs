struct A<'a> {
    a: &'a i32,
}

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("Dropping A: {}", self.a);
    }
}

// SomeType<T>
// SomeType<T> // Compiler thinks:- dropping Sometype will access T (regardless
// of whether or not it access T)

// #![feature(dropck_eyepatch)]

// struct A<'a> {
//     a: &'a mut i32,
//     //b: String, // add later on
// }

// unsafe impl<#[may_dangle] 'a> Drop for A<'a> {
//     fn drop(&mut self) {
//         *self.a *= 2;
//         println!("Dropping");
//     }
// }

fn main() {
    let mut x = 45;
    let y = &mut x;
    println!("{x}");

    let mut x = 45;
    let b = A { a: &mut x };

    drop(b);
    println!("{}", x);
}

/*
-------------------------------------------------------------------------------------------------------------------
Concept/Topic           | Explanation
------------------------|------------------------------------------------------------------------------------------
Drop Check              | Rust performs a conservative analysis during dropping a value.
                        | If a type contains some T, dropping the type is assumed to access T (aka drop check).
                        | This can extend the perceived lifetime of borrows until the end of scope.

Drop Check and Borrows  | A mutable borrow stored in a struct can block later immutable access.
                        | The compiler believes the mutable borrow may be used during drop.
                        | This leads to an apparent violation of borrowing rules.

Effect of Removing Drop | Removing the Drop implementation eliminates the borrow checker conflict.
                        | Without Drop, the compiler knows no access will occur at scope end.
                        | As a result, borrowing rules are satisfied and the code compiles.

Drop Check Eyepatch     | The #![feature(dropck_eyepatch)] allows opting out of conservative drop assumptions.
(Nightly)               | Using #[may_dangle] asserts that drop does not access inner values.
                        | This requires `unsafe` and is only available on nightly Rust.
-------------------------------------------------------------------------------------------------------------------
 */
