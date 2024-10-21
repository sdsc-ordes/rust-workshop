fn main() {
    let numbers = vec![1, 2, 5, 9];
    let smaller_than_5 = |x: i32| -> bool { x < 5 };

    let res = filter(&numbers, smaller_than_5);

    print!("Result: {res:?}")
}

fn filter(numbers: &[i32], func: impl Fn(i32) -> bool) -> Vec<i32> {
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
        let f = |x: i32| -> bool { 3 < x && x < 9 };

        let res = filter(&numbers, f);
        assert_eq!(res, [4, 4, 5, 5, 5]);
    }
}
