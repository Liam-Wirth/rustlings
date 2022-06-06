// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)


fn main() {
    //a vector is an array
    let vec0 = Vec::new();
//not sure why this has to be mutable?
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// this is why it has to be mutable I guess, cause I'm pushing the value 88 to the vector
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
