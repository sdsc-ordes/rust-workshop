// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}

fn main() {
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}
