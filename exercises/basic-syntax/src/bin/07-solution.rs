// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// For example:
// - `5!` is `5 * 4 * 3 * 2 * 1`, which is `120`.
// - `0!` is defined to be `1`.
//
// Implement it with recursion by doing:
// `fatorial(n) := n * factorial(n-1)`

fn main() {
    println!("Factorial of 10 is {}", factorial(10));
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Tests; run with `cargo test --bin 07` or `just run basic-syntax --bin 07`
#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
