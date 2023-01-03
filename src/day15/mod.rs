type Point = (isize, isize);

struct Sensor {
    pos: Point,
    range: usize,
    f_range: f32,
    beacon_pos: Point,
}

impl Sensor {
    fn in_range(&self, other: &Point) -> bool {
        manhattan_distance(&other, &self.pos) <= self.range
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    f32::hypot(p1.0.abs_diff(p2.0) as f32, p1.1.abs_diff(p2.1) as f32)
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let nbrs: Vec<_> = line.split(&['=', ',', ':']).collect();
            let nbrs = vec![nbrs[1], nbrs[3], nbrs[5], nbrs[7]];

            let nbrs: Vec<_> = nbrs.iter().map(|n| n.parse::<isize>().unwrap()).collect();

            let (sensor, beacon) = ((nbrs[0], nbrs[1]), (nbrs[2], nbrs[3]));

            Sensor {
                pos: sensor,
                range: manhattan_distance(&sensor, &beacon),
                f_range: distance(&sensor, &beacon),
                beacon_pos: beacon,
            }
        })
        .collect()
}

pub fn solve(input: &str, row: isize) -> usize {
    let sensors = parse(input);

    let max_sensor = sensors
        .iter()
        .map(|s| s.pos.0 + s.range as isize)
        .max()
        .unwrap();
    let min_sensor = sensors
        .iter()
        .map(|s| s.pos.0 - s.range as isize)
        .min()
        .unwrap();

    (min_sensor..max_sensor)
        .filter(|x| {
            let pos = (*x, row);
            for s in &sensors {
                let in_range = manhattan_distance(&pos, &s.pos) <= s.range;
                if in_range && pos != s.beacon_pos {
                    return true;
                }
            }
            return false;
        })
        .count()
}

pub fn solve_2(input: &str, search_space: isize) -> u128 {
    let sensors = parse(input);

    for y in 0..search_space {
        let mut x = 0isize;
        while x < search_space {
            let mut found = false;
            for s in &sensors {
                if s.in_range(&(x, y)) {
                    let dy = s.pos.1.abs_diff(y) as isize;

                    found = true;
                    x = s.pos.0 - dy + s.range as isize + 1;
                    break;
                }
            }
            if !found {
                println!("FOUND!: {x}, {y}");
                return (x * 4000000 + y) as u128;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let p1 = solve(&simple, 10);

        assert_eq!(p1, 26);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day15/input");
        let p1 = solve(lines.trim_end(), 2000000);

        assert_eq!(p1, 5335787);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day15/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end(), 2000000);
            assert_eq!(p1, 5335787);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day15/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end(), 4000000);
            assert_eq!(res, 13673971349056);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let res = solve_2(simple, 20);

        assert_eq!(res, 56000011);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day15/input");
        let res = solve_2(&lines.trim_end(), 4000000);

        assert_eq!(res, 13673971349056);
    }
}
