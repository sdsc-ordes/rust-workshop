// TODO: Fix the compiler errors only by reordering the lines.
// Don't add, change or remove any line.

fn main() {
    let mut x = Vec::new();
    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);
    assert_eq!(x, [42, 13]);
}
