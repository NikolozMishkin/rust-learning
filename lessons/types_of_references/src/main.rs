fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let mut vec_2 = vec![4, 5, 6];
    let mut reference = &vec_1;
    reference = &vec_2;

    let mut reference = &mut vec_1;
    reference.push(10);
    reference = &mut vec_2;

    let reference = &mut &vec_1;
    *reference = &mut &vec_2;

    let mut reference = &mut &vec_1;
    reference = &mut &vec_2;
}

/*
Summary
----------------------------------------------------------------------------------------------
Concept/Topic           | Explanation
------------------------|---------------------------------------------------------------------
ref: &SomeType          | Immutable binding of an immutable reference.
mut ref: &SomeType      | Mutable binding of an immutable reference.
ref: &mut SomeType      | Immutable binding of a mutable reference.
mut ref: &mut SomeType  | Mutable binding of a mutable reference.
ref: &mut &SomeType     | Immutable binding of a mutable reference to an immutable reference.
mut ref: &mut &SomeType | Mutable binding of a mutable reference to an immutable reference.
----------------------------------------------------------------------------------------------
*/
