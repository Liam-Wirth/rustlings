// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!


fn main() {

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    for x in 0..100 {
        if x%2 == 0 {
            print!{"DICK!"}
        }else {
            println!{ "AND BALLS!"}
        }
        vec1.push(x);
    }
    vec1.sort();
    vec1.push(88);
    for mut x in vec1.iter() {
     println!("{} is a gay stupid idiot baby dumb idiot!", x);
    }

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument Instead I make it return Vec<i32>
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();

   vec.push(22);
   vec.push(44);
   vec.push(66);

    vec
}
