// TODO: Implement this function `filter` which takes a closure `func` and filters
//       all numbers in `numbers` for which the filter returns `false`.
//       Use `Fn(i32)` for the `func` type in this exercise.
//       Make the tests pass.

fn main() {
    let numbers = vec![1, 2, 5, 9];
    let smaller_than_5 = |x: i32| -> bool { x < 5 };

    let res = filter(&numbers, smaller_than_5);

    print!("Result: {res:?}")
}

// TODO: Remove `IncorrectType` and replace the signature of `filter` with the correct types .
type IncorrectType = ();

fn filter(numbers: IncorrectType, func: IncorrectType) -> Vec<i32> {
    todo!("Implement this function and correct the types.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let numbers = vec![1, 1, 3, 3, 3, 4, 4, 5, 5, 5, 9];
        let between_3_and_9 = |x: i32| -> bool { 3 < x && x < 9 };

        let res = filter(&numbers, between_3_and_9);
        assert_eq!(res, [4, 4, 5, 5, 5]);
    }
}
