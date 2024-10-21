fn main() {
    let numbers = vec![1, 2, 5, 9];
    let smaller_than_5 = |x: i32| -> bool { x < 5 };

    let res = filter(&numbers, smaller_than_5);

    print!("Result: {res:?}")
}

fn filter(numbers: &[i32], mut func: impl FnMut(i32) -> bool) -> Vec<i32> {
    let mut res = Vec::<i32>::with_capacity(numbers.len());

    for &x in numbers {
        if func(x) {
            res.push(x)
        }
    }

    res
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
