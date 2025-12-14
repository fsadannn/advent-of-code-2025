fn main() {
    let input = shared::get_input(9);
    let mut points: Vec<(u64, u64)> = vec![];

    let mut max_size = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (x_s, y_s) = line.split_once(",").unwrap();
        let x = x_s.parse::<u64>().unwrap();
        let y = y_s.parse::<u64>().unwrap();

        for (px, py) in points.iter() {
            let v = (x.abs_diff(*px) + 1) * (y.abs_diff(*py) + 1);
            if v > max_size {
                max_size = v;
            }
        }

        points.push((x, y));
    }

    println!("part 1: {max_size}");
}
