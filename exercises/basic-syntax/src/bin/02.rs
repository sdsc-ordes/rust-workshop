fn main() {
    if (bigger(10, 20)) {
        println!("10 is bigger than 20");
    } else {
        println!("10 still isn't bigger than 20");
    }
}

fn bigger(a: i32, b: i32) -> i32 {
    // TODO
}

// Tests; run with `cargo test --bin 02` or `just run basic-syntax --bin 02`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_biggers() {
        assert!(bigger(20, 10));
        assert!(!bigger(10, 20));
    }
}
