struct A;
struct B;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping value of type A");
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("Dropping value of type B");
    }
}

struct Container {
    a: A,
    b: B,
}
fn some_fn(a: A, b: B) {}
fn main() {
    let a = A;
    let b = B;
    // some_fn(a, b);

    // let c = Container { b: B, a: A };

    // let _x = (A {}, B {});
}

/*
-----------------------------------------------------------------------------------------------
Concept/Topic            | Explanation
-------------------------|---------------------------------------------------------------------
Reverse Drop Order       | Local variables are dropped in the reverse order of their creation.
                         | A value created later is dropped before values created earlier.
                         | This applies to variables and function arguments.

Reason for Reverse Order | Later variables may hold references to earlier values.
                         | Dropping later values first prevents dangling references.
                         | This ordering is in conformity with Rust’s safety guarantees.

Nested Values Exception  | Nested values are dropped in source-code order, not reverse order.
                         | Fields are dropped in the order they are defined.
                         | This behavior is independent of the fields instantiation order.
-----------------------------------------------------------------------------------------------
 */
