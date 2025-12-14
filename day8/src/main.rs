use std::{collections::HashSet, ops::Sub};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn norm(&self) -> u64 {
        ((self.x as i64) * (self.x as i64)
            + (self.y as i64) * (self.y as i64)
            + (self.z as i64) * (self.z as i64))
            .try_into()
            .unwrap()
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Default, Eq, Clone, Copy)]
struct Distance {
    pub d: u64,
    pub v1: usize,
    pub v2: usize,
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.d.cmp(&other.d)
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = shared::get_input(8);
    let mut points: Vec<Point> = vec![];
    let mut distances: Vec<Distance> = vec![];
    for line in input.lines() {
        let mut _split = line.splitn(3, ',');
        let x = _split.next().unwrap().parse::<i32>().unwrap();
        let y = _split.next().unwrap().parse::<i32>().unwrap();
        let z = _split.next().unwrap().parse::<i32>().unwrap();
        let point = Point { x, y, z };
        for (n, p) in points.iter().enumerate() {
            let d = (point - *p).norm();
            distances.push(Distance {
                d,
                v1: n,
                v2: points.len(),
            });
        }
        points.push(point);
    }

    distances.sort();

    let mut seen: HashSet<usize> = HashSet::with_capacity(points.len());
    let mut circuits: Vec<HashSet<usize>> = vec![HashSet::<usize>::new()];
    circuits[0].insert(distances[0].v1);
    seen.insert(distances[0].v1);
    circuits[0].insert(distances[0].v2);
    seen.insert(distances[0].v2);
    for d in distances.iter().skip(1).take(999) {
        let mut c1: Option<usize> = None;
        let mut c2: Option<usize> = None;
        for (n, c) in circuits.iter().enumerate() {
            if c.contains(&d.v1) {
                c1 = Some(n);
            }

            if c.contains(&d.v2) {
                c2 = Some(n);
            }
        }

        if c1.is_none() && c2.is_none() {
            circuits.push(HashSet::<usize>::new());
            circuits.last_mut().unwrap().insert(d.v1);
            seen.insert(d.v1);
            circuits.last_mut().unwrap().insert(d.v2);
            seen.insert(d.v2);
        }

        if let Some(c) = c1
            && c2.is_none()
        {
            circuits.get_mut(c).unwrap().insert(d.v1);
            circuits.get_mut(c).unwrap().insert(d.v2);
            seen.insert(d.v1);
            seen.insert(d.v2);
        }

        if let Some(c) = c2
            && c1.is_none()
        {
            circuits.get_mut(c).unwrap().insert(d.v1);
            circuits.get_mut(c).unwrap().insert(d.v2);
            seen.insert(d.v1);
            seen.insert(d.v2);
        }

        if let Some(cc1) = c1
            && let Some(cc2) = c2
            && c1 != c2
        {
            let idx1 = cc1.min(cc2);
            let idx2 = cc1.max(cc2);
            let circuit = circuits.remove(idx2);
            let circuit_to_update = circuits.get_mut(idx1).unwrap();
            for c in circuit {
                circuit_to_update.insert(c);
            }
        }

        if seen.len() == points.len() {
            break;
        }
    }
    circuits.sort_by_key(|a| a.len());
    println!(
        "{}",
        circuits[circuits.len() - 3..]
            .iter()
            .fold(1, |acc, x| acc * x.len())
    );
}
