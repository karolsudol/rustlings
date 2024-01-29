// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut vec0 = vec![22, 44, 66];

    let mut vec1 = vec0.clone();
    fill_vec(&mut vec1);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}
