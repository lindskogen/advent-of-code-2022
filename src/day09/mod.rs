use std::collections::HashSet;

fn max_distance(p1: &(i32, i32), p2: &(i32, i32)) -> u32 {
    let x_diff = p1.0.abs_diff(p2.0);

    let y_diff = p1.1.abs_diff(p2.1);

    x_diff.max(y_diff)
}

fn parse(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(' ').unwrap();
            (dir, steps.parse().unwrap())
        })
        .collect()
}

pub fn solve_1(input: &str) -> usize {
    let moves = parse(input);

    let mut set = HashSet::new();

    let mut h = (0, 0);
    let mut t = (0, 0);

    set.insert(t);

    for (dir, steps) in moves {
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            let ph = h;
            h.0 += dx;
            h.1 += dy;
            if max_distance(&h, &t) > 1 {
                t.0 = ph.0;
                t.1 = ph.1;

                set.insert(t);
            }
        }
    }

    for y in -100..100 {
        for x in -100..100 {
            if set.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }

    set.len()
}

pub fn solve_2(input: &str) -> usize {
    let moves = parse(input);

    let mut set = HashSet::new();

    let mut h = (0, 0);
    let mut tails = [(0, 0); 9];

    set.insert(tails[8]);

    for (dir, steps) in moves {
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            h.0 += dx;
            h.1 += dy;

            let mut p = h;

            for t in tails.iter_mut() {
                if max_distance(&p, t) > 1 {
                    t.0 += (p.0 - t.0).signum();
                    t.1 += (p.1 - t.1).signum();
                }
                p = *t;
            }

            set.insert(tails[8]);
        }
    }

    for y in -15..6 {
        for x in -11..15 {
            if (x, y) == h {
                print!("H")
            } else if let Some((index, _)) = tails.iter().enumerate().find(|&(_, &p)| (x, y) == p) {
                print!("{}", index + 1)
            } else {
                print!(".")
            }
        }
        println!()
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let p1 = solve_1(&simple);

        assert_eq!(p1, 13);
    }

    #[test]
    fn it_works_simple_2_with_first_input() {
        let simple = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let p2 = solve_2(&simple);

        assert_eq!(p2, 1);
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let p2 = solve_2(&simple);

        assert_eq!(p2, 36);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day09/input");
        let p1 = solve_1(&lines.trim_end());

        assert_eq!(p1, 6269);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day09/input");
        let p1 = solve_2(&lines.trim_end());

        assert_eq!(p1, 2557);
    }
}
