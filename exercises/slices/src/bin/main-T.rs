use std::{fmt::Debug, str::FromStr};

/// Merge two array slices (that have to be sorted) into a vector
/// NOTE: We need `Sized` here since a generic type `T` is always `Sized` which will
///       not fulfill the invocation where `T := str` (which is `!Sized`).
fn merge<'a, T: std::cmp::Ord + ?Sized>(a: &[&'a T], b: &[&'a T]) -> Vec<&'a T> {
    let mut dest = Vec::new();

    let mut a_idx = 0;
    let mut b_idx = 0;

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] <= b[b_idx] {
            dest.push(a[a_idx]);
            a_idx += 1
        } else {
            dest.push(b[b_idx]);
            b_idx += 1
        }
    }

    for elem in &a[a_idx..] {
        dest.push(elem)
    }
    for elem in &b[b_idx..] {
        dest.push(elem)
    }

    dest
}

/// Take an array slice, and sort into a freshly constructed vector using the above function
fn merge_sort<'a, T: std::cmp::Ord + ?Sized>(data: &[&'a T]) -> Vec<&'a T> {
    match data.len() {
        i if i > 2 => {
            let center = data.len() / 2;
            let l = merge_sort(&data[0..center]);
            let r = merge_sort(&data[center..]);

            merge(l.as_slice(), r.as_slice())
        }
        2 => {
            if data[0] <= data[1] {
                return vec![&data[0], &data[1]];
            } else {
                return vec![&data[1], &data[0]];
            }
        }
        _ => {
            vec![&data[0]]
        }
    }
}

/// Read a bunch of numbers from standard input into a Vec<Item>.
fn read_numbers<T>() -> Vec<T>
where
    T: FromStr<Err: Debug>,
{
    use std::io;
    let mut result = Vec::new();

    'outer: for line in io::stdin().lines().flatten() {
        if line == "" {
            break;
        }

        for word in line.split_whitespace() {
            if word == "end" {
                break 'outer;
            }

            // NOTE: Notice here the type inference at play
            //       The type for `word.parse()` will be
            //       inferred from `push`.
            result.push(word.parse().unwrap())
        }
    }

    result
}

fn main() {
    println!("Enter some string.");
    let ins: Vec<String> = read_numbers();
    println!("Data to be sorted:");
    println!("{ins:?}");

    let input: Vec<&str> = ins.iter().map(|s| s.as_ref()).collect();
    let sorted_input = merge_sort(input.as_slice());
    println!("Sorted data:");
    println!("{sorted_input:?}");

    println!("Enter some numbers.");
    // NOTE: See how `read_numbers` gets monomorphized from the
    //       return type from type we specify here.
    let ins: Vec<i32> = read_numbers();
    println!("Data to be sorted:");
    println!("{ins:?}");

    let input: Vec<&i32> = ins.iter().map(|s| s).collect();
    let sorted_input = merge_sort(input.as_slice());
    println!("Sorted data:");
    println!("{sorted_input:?}");
}
