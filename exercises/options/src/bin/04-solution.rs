fn main() {
    // Extend a collection
    let grade: Option<&str> = Some("A+");
    let mut grades: Vec<&str> = vec!["B-", "C+", "D"];

    grades.extend(grade);

    println!("{grades:?}")
}
