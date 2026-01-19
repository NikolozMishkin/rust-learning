fn main() {
    // dereferencing of stack allocated types
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let derf_copy = *ref_1;
    *ref_1 = 13;
    println!("some_data: {}, derf_copy: {}", some_data, derf_copy);

    //owned types: bo & sign at the start of the type
    //borrowed types: starts with & sign at the start of the type

    //dereferencing of heap allocated types
    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &mut heap_data;
    // let deref_copy = *ref_1;
    ref_1.push(8);
    (*ref_1).push(8);
}
