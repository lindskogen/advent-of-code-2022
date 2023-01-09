use std::collections::HashSet;
use std::iter::Cycle;
use std::slice::Iter;

enum Piece {
    HorizontalBeam,
    Plus,
    MirrorL,
    VerticalBeam,
    Square,
}

type Position = (usize, isize);

const MIN_X: usize = 1;
const MAX_X: usize = 7;

const PIECES: [Piece; 5] = [
    Piece::HorizontalBeam,
    Piece::Plus,
    Piece::MirrorL,
    Piece::VerticalBeam,
    Piece::Square,
];

impl Piece {
    fn cycle() -> Cycle<Iter<'static, Piece>> {
        PIECES.iter().cycle()
    }

    fn positions(&self, (x, y): Position) -> Vec<Position> {
        match self {
            Piece::HorizontalBeam => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Piece::Plus => vec![(x, y + 1), (x + 1, y), (x + 2, y + 1), (x + 1, y + 2)],
            Piece::MirrorL => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Piece::VerticalBeam => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Piece::Square => vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)],
        }
    }
}

fn print_grid(grid: &HashSet<Position>, highest_y: isize, piece: Option<Vec<Position>>) {
    let max_y = if let Some(ref positions) = &piece {
        positions
            .iter()
            .map(|&p| p.1)
            .max()
            .unwrap_or(0)
            .max(highest_y)
    } else {
        highest_y
    };

    for y in (0..=max_y).rev() {
        for x in MIN_X..=MAX_X {
            if piece.as_ref().map(|p| p.contains(&(x, y))).unwrap_or(false) {
                print!("@")
            } else if grid.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }

    println!();
    println!();
}

fn simulate_rocks(input: &str, steps: usize) -> isize {
    let mut highest_y = -1isize;
    let mut max_ys = [0isize; 7];

    let mut grid: HashSet<Position> = Default::default();
    let mut moves = input.chars().cycle();

    for piece in Piece::cycle().take(steps) {
        // put piece at left=2, bottom=3

        let mut pos = {
            let x = MIN_X + 2;
            let y = highest_y + 4;
            (x, y)
        };

        // print_grid(&grid, highest_y, Some(piece.positions(pos)));

        loop {
            // wind move

            let next_pos = match moves.next().unwrap() {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                _ => unreachable!(),
            };

            if !piece
                .positions(next_pos)
                .iter()
                .any(|p| p.0 < MIN_X || p.0 > MAX_X || grid.contains(p))
            {
                // no collisions, update pos!
                pos = next_pos;

                // print_grid(&grid, highest_y, Some(piece.positions(next_pos)))
            };

            // move down
            let next_pos = (pos.0, pos.1 - 1);

            let next_positions = piece.positions(next_pos);

            if next_positions.iter().any(|p| grid.contains(p)) || next_pos.1 < 0 {
                // collide with piece below -> rest at pos!
                let positions = piece.positions(pos);
                let prev_y = highest_y;

                highest_y = positions
                    .iter()
                    .map(|(_, y)| *y)
                    .max()
                    .unwrap()
                    .max(highest_y);

                for (x, y) in positions {
                    max_ys[x] = max_ys[x].max(y);
                }

                ys.push(highest_y - prev_y);

                grid.extend(positions);
                break;
                //  print_grid(&grid, highest_y, None);
            } else {
                // no collision, update pos!
                pos = next_pos;
                //  print_grid(&grid, highest_y, Some(piece.positions(next_pos)))
            }
        }
    }

    // print_grid(&grid, highest_y, None);

    println!("{:?}", ys);

    highest_y + 1
}

pub fn solve(input: &str) -> isize {
    simulate_rocks(input, 2022)
}

pub fn solve_2(input: &str) -> isize {
    // let steps = 1000000000000;
    simulate_rocks(input, 10000)
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let p1 = solve(&simple);

        assert_eq!(p1, 3068);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day17/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 3177);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day16/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 1991);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day16/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 2705);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let res = solve_2(simple);

        assert_eq!(res, 1514285714288);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day16/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 2705);
    }
}
