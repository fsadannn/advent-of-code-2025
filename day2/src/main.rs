use std::collections::HashSet;

#[allow(clippy::mut_range_bound)]
fn main() {
    let input = shared::get_input(2);
    let mut sum = 0_u64;
    let mut sum2 = 0_u64;
    for r in input.split(',') {
        let id = r.trim();

        if let Some((r1, r2)) = id.split_once('-') {
            let mut left_bound = r1.parse::<u64>().unwrap();
            let mut right_bound = r2.parse::<u64>().unwrap();
            if left_bound < 11 {
                left_bound = 11;
            }
            if right_bound < 11 {
                right_bound = 11;
            }

            for i in left_bound..=right_bound {
                let n_digits = ((i as f64).log10().floor() as usize) + 1;
                let i_as_str = i.to_string();
                // part 1
                if n_digits & 1 == 0
                    && i_as_str[..i_as_str.len() / 2] == i_as_str[i_as_str.len() / 2..]
                {
                    sum += i;
                    sum2 += i;
                    continue;
                }

                // part 2
                let mut char_set: HashSet<char> = HashSet::new();
                for c in i_as_str.chars() {
                    char_set.insert(c);
                }
                if char_set.len() == 1 {
                    sum2 += i;
                    continue;
                }

                let max_divisor_bound = n_digits / 2;
                if n_digits <= max_divisor_bound || n_digits <= 5 {
                    continue;
                }

                if n_digits & 1 == 0 {
                    let repeat = i_as_str.len() / 2;
                    let factor = 2;
                    let ref_chunk = &i_as_str[..factor];
                    let mut is_invalid = true;
                    for idx in 1..repeat {
                        let chunk = &i_as_str[(idx * factor)..((idx + 1) * factor)];
                        if chunk != ref_chunk {
                            is_invalid = false;
                            break;
                        }
                    }

                    if is_invalid {
                        sum2 += i;
                        continue;
                    }
                }

                let mut repeat: usize = 3;
                'bound: loop {
                    if i_as_str.len() % repeat != 0 {
                        repeat += 2;
                        if repeat > max_divisor_bound {
                            break;
                        }
                        continue;
                    }
                    let factor = i_as_str.len() / repeat;
                    let ref_chunk = &i_as_str[..factor];
                    for idx in 1..repeat {
                        if &i_as_str[(idx * factor)..((idx + 1) * factor)] != ref_chunk {
                            repeat += 2;
                            if repeat > max_divisor_bound {
                                break 'bound;
                            }
                            continue 'bound;
                        }
                    }
                    sum2 += i;

                    repeat += 2;
                    if repeat > max_divisor_bound {
                        break;
                    }
                }
            }
        }
    }

    println!("part 1: {sum}");
    println!("part 2: {sum2}");
}
