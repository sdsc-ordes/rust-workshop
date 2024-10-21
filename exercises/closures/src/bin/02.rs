// TODO: Implement this function `filter` which takes a closure `func` and filters
//       all numbers in `numbers` for which the filter returns `false`.
//
//       Make the type `func` as generic as possible such that also
//       the second test works.

fn main() {
    let numbers = vec![1, 2, 5, 9];
    let smaller_than_5 = |x: i32| -> bool { x < 5 };

    let res = filter(&numbers, smaller_than_5);

    print!("Result: {res:?}")
}

// TODO: Remove `IncorrectType` and replace the signature of `filter` with the correct types .
type IncorrectType = ();

fn filter(numbers: IncorrectType, func: IncorrectType) -> Vec<i32> {
    todo!("Implement this function.");

    vec![]
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

    #[test]
    fn check_mutable() {
        let numbers = vec![1, 1, 3, 3, 3, 4, 4, 5, 5, 5, 9];

        let mut sum: i32 = 0;
        let sum_lt_11 = |x: i32| -> bool {
            sum += x;
            sum <= 11
        };

        let res = filter(&numbers, sum_lt_11);
        assert_eq!(res, [1, 1, 3, 3, 3]);
    }
}
