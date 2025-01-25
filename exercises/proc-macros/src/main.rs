use lib_macro::comp;

fn main() {
    let v = vec![1, 2, 3];
    for i in comp![ x*x for x in v ] {
        println!("{i}")
    }
}
