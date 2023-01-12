use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "([{}, {}, {}])", self.x, self.y, self.z)
    }
}

const DELTAS: [(isize, isize, isize); 6] = [
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
];

struct CoordIterator<'a> {
    base: &'a Coord,
    idx: usize,
}

impl<'a> Iterator for CoordIterator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < DELTAS.len() {
            let i = self.idx;
            self.idx += 1;
            let (dx, dy, dz) = DELTAS[i];
            Some(Coord {
                x: self.base.x + dx,
                y: self.base.y + dy,
                z: self.base.z + dz,
            })
        } else {
            return None;
        }
    }
}

impl Coord {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    fn neighbors(&self) -> CoordIterator {
        CoordIterator { base: self, idx: 0 }
    }
}

fn parse(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(',');

            Coord {
                x: nums.next().unwrap().parse().unwrap(),
                y: nums.next().unwrap().parse().unwrap(),
                z: nums.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

pub fn solve(input: &str) -> usize {
    let coords = parse(input);
    let mut count = 0;

    for c in &coords {
        for n in c.neighbors() {
            if !coords.contains(&n) {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_2(input: &str) -> usize {
    let coords = parse(input);
    let (lower, upper) = find_bounds(&coords);
    let mut empty_spaces = HashSet::new();
    let mut queue = Vec::new();

    queue.push(lower);

    while let Some(coord) = queue.pop() {
        for n in coord.neighbors() {
            if !coords.contains(&n)
                && !empty_spaces.contains(&n)
                && is_within_bounds(&n, &lower, &upper)
            {
                queue.push(n);
            }
        }

        empty_spaces.insert(coord);
    }

    let faces: usize = empty_spaces
        .iter()
        .flat_map(|c| c.neighbors().filter(|n| coords.contains(&n)))
        .count();

    faces
}

fn is_within_bounds(c: &Coord, lower: &Coord, upper: &Coord) -> bool {
    c.x >= lower.x
        && c.y >= lower.y
        && c.z >= lower.z
        && c.x <= upper.x
        && c.y <= upper.y
        && c.z <= upper.z
}

fn find_bounds(coords: &HashSet<Coord>) -> (Coord, Coord) {
    let mut lower = Coord::new(100, 100, 100);
    let mut upper = Coord::new(0, 0, 0);

    for Coord { x, y, z } in coords {
        lower.x = lower.x.min(*x);
        lower.y = lower.y.min(*y);
        lower.z = lower.z.min(*z);

        upper.x = upper.x.max(*x);
        upper.y = upper.y.max(*y);
        upper.z = upper.z.max(*z);
    }

    lower.x -= 1;
    lower.y -= 1;
    lower.z -= 1;

    upper.x += 1;
    upper.y += 1;
    upper.z += 1;

    (lower, upper)
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_checks_bounds() {
        let lower = Coord::new(0, 0, 0);
        let upper = Coord::new(1, 2, 3);

        assert!(is_within_bounds(&Coord::new(1, 1, 1), &lower, &upper));
        assert!(is_within_bounds(&Coord::new(1, 2, 3), &lower, &upper));
        assert!(is_within_bounds(&Coord::new(0, 0, 0), &lower, &upper));
        assert!(!is_within_bounds(&Coord::new(2, 2, 3), &lower, &upper));
    }

    #[test]
    fn it_works_simple() {
        let simple = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        let p1 = solve(&simple);

        assert_eq!(p1, 64);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day18/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 4580);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day18/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 4580);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day18/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 2610);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

        let res = solve_2(simple);

        assert_eq!(res, 58);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day18/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 2610);
    }
}
