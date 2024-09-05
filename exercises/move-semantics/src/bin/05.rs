fn main() {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);
    assert_eq!(x, [42, 13]);
}
