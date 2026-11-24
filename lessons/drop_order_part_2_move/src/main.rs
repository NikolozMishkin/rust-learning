struct A {
    value: String,
}
struct B;

impl Drop for A {
    fn drop(&mut self) {
        println!("Dropping value of type A with value {}", self.value);
    }
}

impl Drop for B {
    fn drop(&mut self) {
        println!("Dropping value of type B");
    }
}

trait T {}
impl T for A {}
impl T for B {}
fn main() {
    // let x = (
    //     A {
    //         value: "first".to_string(),
    //     },
    //     B {},
    // );
    // let (a, b) = (
    //     A {
    //         value: "Second".to_string(),
    //     },
    //     B {},
    // );
    // let x = (
    //     &A {
    //         value: "thread".to_string(),
    //     },
    //     &B {},
    // );

    // let v: Vec<&dyn T> = vec![&A {}, &B {}];

    let vec = vec![
        A {
            value: String::from("First"),
        },
        A {
            value: String::from("Second"),
        },
        A {
            value: String::from("Third"),
        },
    ];

    // Key Rules Recap:

    // Data Type                        || Drop Order
    // Owned data in tuples/structs     || Drop in declaration order
    // Owned data in vectors            || Drop in insertion order
    // Local variables                  || Drop in reverse declaration order
    // References                       || Don't affect the drop order of the referenced data
}

/*
------------------------------------------------------------------------------------------------------------------
Concept/Topic              | Explanation
---------------------------|--------------------------------------------------------------------------------------
Drop Order Recap           | Nested owned data (struct fields, tuple elements) are dropped in source-code order.
                           | Local variables, however, are dropped in reverse order of their declaration.

Tuple Destructuring Effect | Tuple elements are dropped in order when the tuple owns them.
                           | When a tuple is destructured, the extracted bindings become local variables.
                           | As locals, they follow reverse declaration drop order instead.

Ownership vs References    | Drop order depends on ownership, not on where values are mentioned.
                           | Reference do not affect the drop order of the referenced data.

Vectors and Ownership      | Owned elements inside a vector are dropped in insertion order.
------------------------------------------------------------------------------------------------------------------
 */
