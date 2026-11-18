struct A<'a> {
    a: &'a mut i32,
    s: String, // add later on
}

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let mut x = 45;
    let b = A {
        a: &mut x,
        s: String::from("some str"),
    };

    // let c = b.s;
}

/*
----------------------------------------------------------------------------------------------------------------------
Concept/Topic              | Explanation
---------------------------|------------------------------------------------------------------------------------------
Drop and Inner Access      | The compiler assumes its inner contents of a type may be accessed during drop.

Drop and Field Moves       | Types that implement Drop do not allow moving out individual fields (aka Partial move).
                           | The compiler rejects partial moves.

Reason for the Restriction | The drop method requires mutable access to the entire self.
                           | Dropping a partially moved value could lead to invalid cleanup or undefined behavior.
                           | Removing the Drop implementation allows field moves again.
-----------------------------------------------------------------------------------------------------------------------
*/
