// This exercise practices blanket implementations.
//
//  NOTE: Follow the todo numbers in sequence!

use std::sync::Arc;

// Define the simple trait.
trait Washable<'a> {
    fn name(&self) -> &str;
    fn wash(&self);
}

// Define our simple banana.
#[derive(Clone, Debug, PartialEq, Hash, Default)]
struct Banana<'a> {
    name: &'a str,
}

// Implement `Washable` for `Banana`.
impl<'a> Washable<'a> for Banana<'a> {
    fn name(&self) -> &str {
        return "Banana";
    }

    fn wash(&self) {
        println!("Swoosh...");
    }
}

// `wash_properly` is considered a library function which
// washes an element 10 times.
fn wash_properly<'a>(element: impl Washable<'a>) {
    for i in 0..10 {
        println!("- Wash cycle '{}' for element '{}'", i, element.name());
        element.wash();
    }
}

fn main() {
    let b = Banana::default();

    // TODO: 1. Call here `wash_properly` on `b`.
    // wash_properly(b);

    // TODO: 2. Call here `wash_properly` on `&b`
    // and explore the error the compiler spits out.
    let b = Banana::default();

    // TODO: 4. Call here `wash_properly` on `boxed` and `arc`.
    //          For that to work you have to do the same as in 3.
    //          but `for Box<T>` and `for Arc<T>`.
    let boxed = Box::new(Banana::default());
    let arc = Arc::new(Banana::default());

    // TODO: 5. Call here `wash` on `boxed` and `arc`.
    //          Do you know why that work?
    let boxed = Box::new(b);
    let arc = Arc::new(Banana::default());
}

// TODO: 3. Implement a blanket implementation `Washable` for `&T`.
//          It's of the form `impl<T> Washable for &'a T ...`.
