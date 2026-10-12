fn main() {
    let str = String::from("");
    let ref_str_1 = &str;
    let ref_str_2 = ref_str_1;
    println!("{ref_str_1}");

    let mut str_slice_1: &str;
    {
        let str_slice_2 = "";
        str_slice_1 = "";
    }
    println!("{str_slice_1}");

    let str_slice_from_string: &str;
    {
        let string = String::from("");
        //str_slice_from_string = &*string;
    }

    //println!("{str_slice_from_string}");

    let mut x = 45;
    let z = &mut x;
    let y = &*z;
}

/*
----------------------------------------------------------------------------------------------------
Concept / Topic           | Explanation
--------------------------|-------------------------------------------------------------------------
&ref vs &mut ref          | Immutable references are copied when assigned.
                          | Mutable references are moved and not copied.
                          | This is ensures that mutable references have exclusive access.

References to slices      | References to string slices and array slices can have static lifetimes.
                          | Their validity is not tied to the scope of the variable holding the reference.

Slice from Owned Values   | Slices created from owned values do not have static lifetimes.
                          | The owned value is dropped when its scope ends.
                          | Any reference to that owned value becomes invalid outside the scope.

Dereference and Reference | The syntax of &*variable can be used to converts a &mut ref -> &ref.
Pattern                   | It is useful when mutability is needed temporarily but not globally.
--------------------------------------------------------------------------------------------------------
*/
