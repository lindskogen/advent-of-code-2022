use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Piece {
    HorizontalBeam,
    Plus,
    MirrorL,
    VerticalBeam,
    Square,
}

type Coord = (usize, usize);

const WIDTH: usize = 7;

const PIECES: [Piece; 5] = [
    Piece::HorizontalBeam,
    Piece::Plus,
    Piece::MirrorL,
    Piece::VerticalBeam,
    Piece::Square,
];

impl Piece {
    fn positions(&self, (x, y): Coord) -> Vec<Coord> {
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

fn is_valid(pos: &Coord, piece: &Piece, grid: &HashSet<(usize, usize)>) -> bool {
    piece
        .positions(*pos)
        .iter()
        .all(|p @ (x, _)| *x < WIDTH && !grid.contains(p))
}

fn simulate_rocks(input: &str, steps: usize) -> usize {
    let mut cache = HashMap::new();
    let mut y_height = 0usize;
    let mut offset = 0usize;

    let mut grid = HashSet::new();
    let moves: Vec<_> = input.chars().collect();

    let mut piece_count = 0usize;
    let mut wind_count = 0usize;

    while piece_count < steps {
        let piece = PIECES[piece_count % PIECES.len()];
        // put piece at left=2, bottom=3

        let mut pos: Coord = (2, y_height + 3);

        loop {
            // wind move
            let wind_move = moves[wind_count % moves.len()];

            let next_pos = match wind_move {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0.saturating_sub(1), pos.1),
                _ => unreachable!(),
            };

            if is_valid(&next_pos, &piece, &grid) {
                // no collisions, update pos!
                pos = next_pos;
            }

            wind_count += 1;

            // move down
            let next_pos = (pos.0, pos.1.saturating_sub(1));

            if pos.1 == 0 || !is_valid(&next_pos, &piece, &grid) {
                // collide with piece below -> rest at pos!
                break;
            }
            // no collision, update pos!
            pos = next_pos;
        }

        let positions = piece.positions(pos);
        for (_, y) in &positions {
            y_height = y_height.max(y + 1);
        }
        grid.extend(positions);

        if offset == 0 {
            let key = (piece_count % PIECES.len(), wind_count % moves.len());

            if let Some((2, old_count, old_height)) = cache.get(&key) {
                let delta_top = y_height - old_height;
                let delta_count = piece_count - old_count;
                let repeats = (steps - piece_count) / delta_count;
                offset += repeats * delta_top;
                piece_count += repeats * delta_count;
            }

            cache
                .entry(key)
                .and_modify(|(n, old_count, old_height)| {
                    *n += 1;
                    *old_count = piece_count;
                    *old_height = y_height;
                })
                .or_insert((1usize, piece_count, y_height));
        }

        piece_count += 1;
    }

    y_height + offset
}

pub fn solve(input: &str) -> usize {
    simulate_rocks(input, 2022)
}

pub fn solve_2(input: &str) -> usize {
    // let steps = 1000000000000;
    simulate_rocks(input, 1000000000000)
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
        let lines = read_file_to_string("src/day17/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 3177);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day17/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 1565517241382);
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
        let lines = read_file_to_string("src/day17/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 1565517241382);
    }
}
