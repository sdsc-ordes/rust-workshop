/// Merge two array slices (that have to be sorted) into a vector
fn merge<'a>(a: &[&'a String], b: &[&'a String]) -> Vec<&'a String> {
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
fn merge_sort<'a>(data: &[&'a String]) -> Vec<&'a String> {
    match data.len() {
        i if i > 2 => {
            let center = data.len() / 2;
            let l = merge_sort(&data[0..center]);
            let r = merge_sort(&data[center..]);

            merge(&l, &r)
        }
        2 => {
            if data[0] < data[1] {
                return data.to_vec();
            } else {
                return vec![data[1], data[0]];
            }
        }
        _ => data.to_vec(),
    }
}

/// Read a bunch of numbers from standard input into a Vec<String>.
fn read_numbers() -> Vec<String> {
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
    let ins = read_numbers();
    println!("Data to be sorted:");
    println!("{ins:?}");

    let input: Vec<&String> = ins.iter().collect();
    let sorted_input = merge_sort(&input);
    println!("Sorted data:");
    println!("{sorted_input:?}");
}
