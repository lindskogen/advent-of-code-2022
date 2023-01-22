use std::collections::HashMap;

type Pos = (isize, isize);

#[derive(Debug)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn value(&self) -> isize {
        match *self {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }

    fn rotate_right(&mut self) {
        *self = match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        };
    }

    fn rotate_left(&mut self) {
        *self = match self {
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
        };
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Space,
}

#[derive(Debug)]
enum Move {
    Num(usize),
    Right,
    Left,
}

fn parse(input: &str) -> (HashMap<Pos, Tile>, Vec<Move>) {
    let (string_map, string_moves) = input.split_once("\n\n").unwrap();

    let map = string_map
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.char_indices().filter_map(move |(col, c)| {
                if c == '.' {
                    Some(((col as isize + 1, row as isize + 1), Tile::Space))
                } else if c == '#' {
                    Some(((col as isize + 1, row as isize + 1), Tile::Wall))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut moves = Vec::new();

    let mut start_index = None;

    for (idx, ch) in string_moves.char_indices() {
        match ch {
            'L' => {
                if let Some(from) = start_index.take() {
                    moves.push(Move::Num(string_moves[from..idx].parse().unwrap()))
                }
                moves.push(Move::Left)
            }
            'R' => {
                if let Some(from) = start_index.take() {
                    moves.push(Move::Num(string_moves[from..idx].parse().unwrap()))
                }
                moves.push(Move::Right)
            }
            _ => {
                if start_index.is_none() {
                    start_index = Some(idx);
                }
            }
        }
    }

    if let Some(from) = start_index {
        if let Ok(num) = string_moves[from..].parse() {
            moves.push(Move::Num(num))
        }
    }

    (map, moves)
}

pub fn solve(input: &str) -> isize {
    let (map, moves) = parse(input);

    let start_pos: Pos = *map
        .keys()
        .filter(|(x, y)| *y == 1)
        .min_by_key(|(x, y)| x)
        .unwrap();

    let ((col, row), facing) = walk(&map, &moves, start_pos);

    1000 * row + 4 * col + facing.value()
}

fn step_and_wrap(
    old_pos: &Pos,
    dir: &Dir,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
) -> Pos {
    let (dx, dy) = match dir {
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
        Dir::Up => (0, -1),
    };

    let mut pos = ((old_pos.0 + dx), old_pos.1 + dy);

    if pos.0 < min_x {
        println!("Wrap left");
        pos.0 = max_x;
    } else if pos.0 > max_x {
        println!("Wrap right");
        pos.0 = min_x;
    } else if pos.1 < min_y {
        println!("Wrap top");
        pos.1 = max_y;
    } else if pos.1 > max_y {
        println!("Wrap bottom");
        pos.1 = min_y;
    }

    pos
}

fn walk(map: &HashMap<Pos, Tile>, moves: &Vec<Move>, start_pos: Pos) -> (Pos, Dir) {
    let mut dir = Dir::Right;
    let mut pos = start_pos;
    let min_x: isize = 1;
    let min_y: isize = 1;

    let max_x: isize = *map.keys().map(|(x, _)| x).max().unwrap();
    let max_y: isize = *map.keys().map(|(_, y)| y).max().unwrap();

    for m in moves {
        match *m {
            Move::Right => {
                println!("Rotate right at {:?}", pos);
                dir.rotate_right()
            }
            Move::Left => {
                println!("Rotate left {:?}", pos);
                dir.rotate_left()
            }
            Move::Num(n) => {
                println!("Move {} at {:?}", n, pos);
                for _ in 0..n {
                    let mut new_pos = step_and_wrap(&pos, &dir, min_x, min_y, max_x, max_y);

                    let mut has_valid_pos = map.contains_key(&new_pos);

                    while !has_valid_pos {
                        new_pos = step_and_wrap(&new_pos, &dir, min_x, min_y, max_x, max_y);
                        has_valid_pos = map.contains_key(&new_pos);
                    }

                    match map[&new_pos] {
                        Tile::Wall => break,
                        Tile::Space => {
                            pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    (pos, dir)
}

pub fn solve_2(input: &str) -> isize {
    5
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let p1 = solve(&simple);

        assert_eq!(p1, 6032);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day22/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 162186);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day22/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 162186);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day22/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 6676132372578);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        let res = solve_2(simple);

        assert_eq!(res, 301);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day22/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 3759566892641);
    }
}
