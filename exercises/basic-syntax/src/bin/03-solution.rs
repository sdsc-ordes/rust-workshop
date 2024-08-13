fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max = i32::MIN;
    let mut min = i32::MAX;

    for i in input {
        min = i32::min(min, i);
        max = i32::max(max, i);
    }

    println!("{} is largest and {} is smallest", max, min);
}
