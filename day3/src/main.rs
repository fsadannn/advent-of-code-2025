use std::collections::HashSet;

#[derive(Default, Debug)]
struct MaxVal {
    index: usize,
    value: u64,
}

fn find_max(line: &str, exclude: Option<&HashSet<usize>>, skip: usize) -> MaxVal {
    let mut d1: u64 = 0;
    let mut index: usize = 0;
    let excluded_indexes = match exclude {
        Some(v) => v,
        None => &HashSet::<usize>::new(),
    };
    for (n, &d_byte) in line.as_bytes().iter().skip(skip).enumerate() {
        if excluded_indexes.contains(&n) {
            continue;
        }
        let digit = (d_byte - 48) as u64;
        if digit > d1 {
            d1 = digit;
            index = n;
        }
    }

    MaxVal { index, value: d1 }
}

fn main() {
    let input = shared::get_input(3);
    // part 1
    let mut sum: u64 = 0;
    let mut set = HashSet::<usize>::with_capacity(1);
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let d1 = find_max(line, None, 0);
        set.clear();
        set.insert(d1.index);
        let d2 = find_max(line, Some(&set), 0);

        if d1.index < d2.index {
            sum += d1.value * 10 + d2.value;
            continue;
        }
        if d1.index == line.len() - 1 {
            sum += d2.value * 10 + d1.value;
            continue;
        }
        let d3 = find_max(line, None, d1.index + 1);
        sum += d1.value * 10 + d3.value;
    }

    println!("part 1: {sum}");
}
