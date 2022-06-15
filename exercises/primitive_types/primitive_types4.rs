// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

//! the ending index of a slice is indexoflastpoint +1

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
