#[derive(PartialEq, Debug, Clone, Copy)]
enum MapValue {
    Blank,
    Paper,
    Accesible,
}

fn is_accesible(row: usize, col: usize, map: &[Vec<MapValue>]) -> bool {
    let mut count: u8 = 0;
    if col > 0 && map[row][col - 1] == MapValue::Paper {
        count += 1;
    }

    if col < map[0].len() - 1 && map[row][col + 1] == MapValue::Paper {
        count += 1;
    }

    if row > 0 && map[row - 1][col] == MapValue::Paper {
        count += 1;
    }

    if row < map.len() - 1 && map[row + 1][col] == MapValue::Paper {
        count += 1;
    }

    if col > 0 && row > 0 && map[row - 1][col - 1] == MapValue::Paper {
        count += 1;
    }

    if col < map[0].len() - 1 && row < map.len() - 1 && map[row + 1][col + 1] == MapValue::Paper {
        count += 1;
    }

    if row > 0 && col < map[0].len() - 1 && map[row - 1][col + 1] == MapValue::Paper {
        count += 1;
    }

    if row < map.len() - 1 && col > 0 && map[row + 1][col - 1] == MapValue::Paper {
        count += 1;
    }

    count < 4
}

// fn print_map(map: &[Vec<MapValue>]) {
//     for row in map.iter() {
//         for v in row.iter() {
//             match *v {
//                 MapValue::Blank => print!("."),
//                 MapValue::Paper => print!("@"),
//                 MapValue::Accesible => print!("X"),
//             }
//         }
//         println!();
//     }
// }

fn main() {
    let input = shared::get_input(4);
    let mut map: Vec<Vec<MapValue>> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut row: Vec<MapValue> = vec![];
        for v in line.chars() {
            if v == '@' {
                row.push(MapValue::Paper);
            } else {
                row.push(MapValue::Blank);
            }
        }

        map.push(row);
    }

    let mut count = 0;
    let mut iteration = 0;
    let mut removed = -1;
    let mut count2 = 0;
    let mut map2 = map.clone();
    while removed != 0 {
        removed = 0;
        for i in 0..map.len() {
            for j in 0..map.first().unwrap().len() {
                if map[i][j] != MapValue::Paper {
                    continue;
                }
                if is_accesible(i, j, &map) {
                    removed += 1;
                    map2[i][j] = MapValue::Accesible;
                }
            }
        }
        if iteration == 0 {
            count = removed;
        }
        count2 += removed;
        iteration += 1;
        map = map2.clone();
    }

    println!("Part 1: {count}");
    println!("Part 2: {count2}");
}
