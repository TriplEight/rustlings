// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // solution 3
    // let vec0 = Vec::new();
    let mut vec0 = Vec::new();

    // solution 1
    // let mut vec1 = fill_vec(vec0.clone());

    // solution 2
    // let mut vec1 = fill_vec(&vec0);

    // solution 3
    // rm the initial line
    // let mut vec1 = fill_vec(&mut vec0);
    fill_vec(&mut vec0);


    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // solution 3
    // rm the initial line
    // vec1.push(88);

    // solution 3
    // rm the initial line
    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// solution 3
// rm the initial line
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

// solution 2
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();

// solution 3
fn fill_vec(vec: &mut Vec<i32>) {

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
