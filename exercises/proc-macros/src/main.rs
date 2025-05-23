// This exercise is an excercise following the video:
// https://www.youtube.com/watch?v=SMCRQj9Hbx8
use lib_macro::comp;

fn main() {
    println!("Comprehension: ");
    let v = vec![1, 2, 3];
    for i in comp![ x*x for x in v ] {
        println!("{i}")
    }

    println!("Comprehension with if: ");
    let v = 1..=10;
    for i in comp![ x*x for x in v if x >= 3 if x <= 7 ] {
        println!("{i}")
    }

    // That should not compile.
    // println!("Comprehension with if (should not parse): ");
    // let v = 1..=10;
    // for i in comp![ x*x for x in v if x >= 3 && if x <= 7 ] {
    //     println!("{i}")
    // }
}
