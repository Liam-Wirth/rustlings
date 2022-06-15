// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

//!important, I guess the lesson to learn here is the fact that the reference is being made, and then the value is being updated
//! and then that initial reference can be disposed, as we use another reference?
fn main() {
    let mut x = 100;
    //so this V is the reference.
    let y = &mut x;
    *y += 100;
    println!{"{} is Y, which is a reference to X ", y }
    println!{"{} is x, which is the variable being referenced ", x }
    let z = &mut x;
    println!{"{} now, the first reference, y, has been disposed of, and replaced with another reference z. Why? I dunno ", z }
    *z += 1000;
    assert_eq!(x, 1200);
}
