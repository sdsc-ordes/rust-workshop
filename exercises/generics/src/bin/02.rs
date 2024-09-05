// This powerful wrapper provides the ability to store a positive integer value.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // A normal `new(value: T)` is also sufficient, but this
    // constructor is more generic taking everything which can be
    // turned into a T. Below that is also `&str` into String.
    fn new(value: impl Into<T>) -> Self {
        Wrapper {
            value: value.into(),
        }
    }
}

fn main() {
    Wrapper::<i32>::new(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::<i32>::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::<String>::new("Foo").value, "Foo");
    }
}
