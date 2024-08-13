// A panic is Rust's way to signal that something went so wrong that the
// program can't continue executing, it's an unrecoverable error
// Division by zero classifies as such an error.
// Look into panic macro in https://doc.rust-lang.org/std/macro.panic.html
// to complete this task.

fn main() {
    println!("Speed is: {}", speed(0, 100, 10))
}

// This function computes the average speed for a walk which took
// `time_elapsed` from `start` to `end`.
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end.abs_diff(start);
    if time_elapsed == 0 {
        panic!("The journey took no time at all, that's impossible!")
    }

    distance / time_elapsed
}

// Tests; run with `cargo test --bin 06` or `just run basic-syntax --bin 06`
#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    //    under test to panic. We can also check the panic message by using `expected`.
    //    This is all part of Rust's built-in test framework!
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
