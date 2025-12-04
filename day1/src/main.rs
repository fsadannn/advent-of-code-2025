fn main() {
    let input = shared::get_input(1);

    let mut current = 50;
    let mut count = 0;
    let mut count2 = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut direction = 1;
        if line.starts_with("L") {
            direction = -1;
        }
        let mut counter = line[1..].parse::<i32>().unwrap();
        if counter >= 100 {
            count2 += counter / 100;
            counter %= 100;
        }
        let prev = current;
        current += direction * counter;
        match current {
            v if v < 0 => {
                current += 100;
                if prev != 0 {
                    count2 += 1;
                }
            }
            v if v >= 100 => {
                current -= 100;
                count2 += 1;
                if current == 0 {
                    count += 1;
                }
            }
            0 => {
                count += 1;
                count2 += 1;
            }
            _ => {}
        }
    }
    println!("part 1: {}", count);
    println!("part 2: {}", count2);
}
