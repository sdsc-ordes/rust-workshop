// TODO: Make both vectors `vec0` and `vec1` accessible in `main`
// at the same time to

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0);

    assert_eq!(vec0, [22, 44, 66]);
    assert_eq!(vec1, [22, 44, 66, 88]);
}
