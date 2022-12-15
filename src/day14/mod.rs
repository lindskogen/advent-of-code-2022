use std::collections::HashMap;

enum Material {
    Rock,
    Sand,
    Air,
}

impl Material {
    fn debug(&self) -> char {
        match self {
            Material::Rock => '#',
            Material::Sand => 'o',
            Material::Air => ' ',
        }
    }
}

fn parse(input: &str) -> HashMap<(isize, isize), Material> {
    let mut map = HashMap::new();

    for line in input.lines() {
        for coords in line
            .split(" -> ")
            .map(|coord| {
                let (xs, ys) = coord.split_once(',').unwrap();

                (xs.parse::<isize>().unwrap(), ys.parse::<isize>().unwrap())
            })
            .collect::<Vec<_>>()
            .windows(2)
        {
            let (x1, y1) = coords[0];
            let (x2, y2) = coords[1];

            for y in y1..=y2 {
                map.insert((x1, y), Material::Rock);
            }

            for y in y2..=y1 {
                map.insert((x1, y), Material::Rock);
            }

            for x in x1..=x2 {
                map.insert((x, y1), Material::Rock);
            }

            for x in x2..=x1 {
                map.insert((x, y1), Material::Rock);
            }
        }
    }

    map
}

pub fn solve(input: &str) -> isize {
    let mut map = parse(input);

    let lowest_rock_y = map.iter().max_by_key(|((_, y), _)| y).unwrap().0 .1;

    let origin_pos = (500, 0);

    let mut is_done = false;

    let mut count = 0;

    'outer: while !is_done {
        // Launch next piece of sand!

        let (mut cx, mut cy) = origin_pos;

        while cy < lowest_rock_y {
            if map.get(&(cx, cy + 1)).is_some() {
                if map.get(&(cx - 1, cy + 1)).is_none() {
                    // check left
                    cx -= 1;
                } else if map.get(&(cx + 1, cy + 1)).is_none() {
                    // check right
                    cx += 1;
                } else {
                    count += 1;
                    map.insert((cx, cy), Material::Sand);
                    continue 'outer;
                }
            }

            cy += 1;
        }
        is_done = true;
    }

    count
}

pub fn solve_2(input: &str) -> isize {
    let mut map = parse(input);

    let lowest_rock_y = map.iter().max_by_key(|((_, y), _)| y).unwrap().0 .1;
    let floor_y = lowest_rock_y + 2;

    let origin_pos = (500, 0);

    let mut count = 0;

    'outer: loop {
        // Launch next piece of sand!

        let (mut cx, mut cy) = origin_pos;

        if map.get(&(cx, cy)).is_some() {
            break;
        }

        loop {
            if cy == floor_y {
                map.insert((cx, cy), Material::Sand);
                continue 'outer;
            } else if map.get(&(cx, cy + 1)).is_some() {
                if map.get(&(cx - 1, cy + 1)).is_none() {
                    // check left
                    cx -= 1;
                } else if map.get(&(cx + 1, cy + 1)).is_none() {
                    // check right
                    cx += 1;
                } else {
                    count += 1;
                    map.insert((cx, cy), Material::Sand);
                    continue 'outer;
                }
            }

            cy += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let p1 = solve(&simple);

        assert_eq!(p1, 24);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day14/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 828);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day14/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 6272);
        })
    }
    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day14/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 25500);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let res = solve_2(simple);

        assert_eq!(res, 93);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day14/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 25500);
    }
}
