use std::collections::HashSet;

fn merge_intervals(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut ranges_merge: Vec<(u64, u64)> = vec![];
    let mut current_range = *ranges.first().unwrap();
    for range in ranges.iter().skip(1) {
        if range.0 > current_range.1 {
            ranges_merge.push(current_range);
            current_range = *range;
            continue;
        }
        current_range = (current_range.0, range.1.max(current_range.1));
    }

    if ranges_merge.last().unwrap().1 < current_range.0 {
        ranges_merge.push(current_range);
    } else {
        ranges_merge.last_mut().unwrap().1 = current_range.1;
    }

    ranges_merge
}

fn main() {
    let input = shared::get_input(5);
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut skip = 0;
    for (n, line) in input.lines().enumerate() {
        if line.is_empty() {
            skip = n;
            break;
        }
        if let Some((r1, r2)) = line.split_once('-') {
            let left_bound = r1.parse::<u64>().unwrap();
            let right_bound = r2.parse::<u64>().unwrap();

            ranges.push((left_bound, right_bound));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ranges_merge: Vec<(u64, u64)> = merge_intervals(&ranges);
    let mut _ranges_merge: Vec<(u64, u64)> = merge_intervals(&ranges_merge);
    while ranges_merge.len() != _ranges_merge.len() {
        ranges_merge = _ranges_merge;
        _ranges_merge = merge_intervals(&ranges_merge);
    }

    let mut values = HashSet::<u64>::new();
    let mut count: u32 = 0;
    for line in input.lines().skip(skip + 1) {
        if line.is_empty() {
            continue;
        }
        let id = line.parse::<u64>().unwrap();
        for (l, r) in ranges.iter() {
            if id >= *l && id <= *r {
                count += 1;
                values.insert(id);
                break;
            }
        }
    }

    let mut count2: u64 = 0;
    for (l, r) in ranges_merge.iter() {
        count2 += *r - *l + 1;
    }

    println!("part 1: {count}");
    println!("part 2: {count2}");
}
