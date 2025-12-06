use std::vec;

fn sum(v: &[u64]) -> u64 {
    v.iter().sum()
}

fn mul(v: &[u64]) -> u64 {
    v.iter().fold(1, |acc, v| acc * v)
}

type Op = fn(&[u64]) -> u64;

#[derive(Default, Debug, Clone)]
struct PadVal {
    pub val: u64,
    pub pad: u8,
    pub size: u8,
}

fn number_from_digits(arr: &[u8]) -> u64 {
    arr.iter().fold(0, |acc, &v| acc * 10 + v as u64)
}

fn parse_line(line: &str, sizes: &[u8]) -> Vec<PadVal> {
    let mut buffer = [0u8; 20];
    let mut idx = 0_usize;
    let line_bytes = line.as_bytes();
    let mut result: Vec<PadVal> = vec![];
    for &size in sizes.iter() {
        let mut pad: u8 = 0;
        let mut see_number = false;
        let mut v_size: u8 = 0;
        for &v in &line_bytes[idx..(idx + size as usize)] {
            if v == b' ' && see_number {
                pad += 1;
                continue;
            }
            if v != b' ' {
                see_number = true;
                buffer[v_size as usize] = v - 48;
                v_size += 1;
                continue;
            }
        }
        result.push(PadVal {
            val: number_from_digits(&buffer[..v_size as usize]),
            pad,
            size: v_size,
        });
        idx += size as usize + 1;
    }

    result
}

fn main() {
    let input = shared::get_input(6);
    let _l1 = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| v.trim())
        .collect::<Vec<&str>>();
    let mut problems: Vec<Vec<u64>> = Vec::with_capacity(_l1.len());
    let mut ops: Vec<Op> = Vec::with_capacity(_l1.len());

    for &val in _l1.iter() {
        problems.push(vec![val.parse::<u64>().unwrap()])
    }

    for line in input.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let _l = line
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim())
            .collect::<Vec<&str>>();
        if *_l.first().unwrap() == "*" || *_l.first().unwrap() == "+" {
            for &val in _l.iter() {
                ops.push(match val {
                    "*" => mul,
                    "+" => sum,
                    _ => panic!("{}", format!("{} is not a valid op, line '{line}'", val)),
                });
            }
            continue;
        }

        for (n, &val) in _l.iter().enumerate() {
            if let Some(v) = problems.get_mut(n) {
                v.push(val.parse::<u64>().unwrap());
            }
        }
    }

    let result = problems
        .iter()
        .zip(ops.iter())
        .fold(0, |acc, (v, op)| acc + op(v));
    println!("part 1: {result}");

    // part 2 ( need problems vec from part 1)
    let mut result2 = 0;
    let sizes: Vec<u8> = problems
        .iter()
        .map(|p| (*p.iter().max().unwrap_or(&0) as f64).log10().floor() as u8 + 1)
        .collect::<Vec<u8>>();
    let mut pad_problems: Vec<Vec<PadVal>> = Vec::with_capacity(problems.len());
    for v in parse_line(input.lines().next().unwrap(), &sizes) {
        pad_problems.push(vec![v]);
    }
    for line in input.lines().skip(1) {
        if line.is_empty() || line.starts_with('*') || line.starts_with('+') {
            continue;
        }
        for (n, val) in parse_line(line, &sizes).iter().enumerate() {
            if let Some(p) = pad_problems.get_mut(n) {
                p.push(val.clone());
            }
        }
    }
    let mut acc: Vec<u64> = Vec::with_capacity(pad_problems.first().unwrap().len());
    for (idx, p) in pad_problems.iter().enumerate() {
        // println!("{p:?}");
        let mut v = 1;
        let mut digit = 1u32;
        while v != 0 {
            v = 0;
            // println!("digit {digit}");
            let mut exp_count: u32 = 0;
            for pv in p.iter().rev() {
                let n_digits = pv.size as u32;
                if n_digits + (pv.pad as u32) < digit || (pv.pad as u32) >= digit {
                    continue;
                }
                // println!(
                //     "pv: {pv:?}, n_digits: {n_digits}, val: {}",
                //     (pv.val / 10u64.pow(digit - (pv.pad as u32) - 1)) % 10
                // );
                v +=
                    ((pv.val / 10u64.pow(digit - (pv.pad as u32) - 1)) % 10) * 10u64.pow(exp_count);
                exp_count += 1;
            }
            // println!("v: {v}");
            if v != 0 {
                acc.push(v);
            }
            digit += 1;
        }
        // println!("acc = {acc:?}");
        result2 += ops.get(idx).unwrap()(&acc);
        acc.clear();
    }

    println!("part 2: {result2}");
}
