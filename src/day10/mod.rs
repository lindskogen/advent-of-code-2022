use crate::day10::Instruction::{Addx, Noop};

pub enum Instruction {
    Noop,
    Addx(i32),
}

const NOOP_CYCLES: usize = 1;
const ADDX_CYCLES: usize = 2;
const MAX_COUNT: usize = 240;

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            if l.len() > 4 {
                let (_, arg) = l.split_once(" ").unwrap();

                Addx(arg.trim().parse().unwrap())
            } else {
                Noop
            }
        })
        .collect()
}

pub fn run_program(instrs: Vec<Instruction>) -> [isize; MAX_COUNT] {
    let mut array = [0; MAX_COUNT];

    let mut x: isize = 1;

    let mut next_value = 0;

    let mut skip_cycles: usize = 0;

    let mut moves = instrs.iter();

    for cycle_count in 0..MAX_COUNT {
        if skip_cycles > 0 {
            skip_cycles -= 1;
        }

        if skip_cycles == 0 {
            x += next_value;
            next_value = 0;
        }

        array[cycle_count] = x;

        if skip_cycles == 0 {
            if let Some(instr) = moves.next() {
                match instr {
                    Addx(value) => {
                        next_value = *value as isize;
                        skip_cycles = ADDX_CYCLES;
                    }
                    Noop => {
                        skip_cycles = NOOP_CYCLES;
                    }
                }
            }
        }
    }

    return array;
}

pub fn solve_1(input: &str) -> isize {
    let moves = parse(input);

    let array = run_program(moves);

    array[19] * 20
        + array[59] * 60
        + array[99] * 100
        + array[139] * 140
        + array[179] * 180
        + array[219] * 220
}

pub fn solve_2(input: &str) {
    let moves = parse(input);

    let states = run_program(moves);

    for (cycle, &x) in states.iter().enumerate() {
        let line_pos = (cycle % 40) as isize;
        if line_pos == 0 {
            println!()
        }
        if line_pos - 1 <= x && x <= line_pos + 1 {
            print!("█");
        } else {
            print!(" ");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"noop
addx 3
addx -5";
        let p1 = solve_1(&simple);

        assert_eq!(p1, 13);
    }

    #[test]
    fn it_works_simple_with_large_input() {
        let simple = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let p2 = solve_2(simple);

        // assert_eq!(p2, 13140);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day10/input");
        let p1 = solve_1(lines.trim_end());

        assert_eq!(p1, 13440);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day10/input");
        b.iter(|| {
            let p1 = solve_1(lines.trim_end());
            assert_eq!(p1, 13440);
        })
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day10/input");
        solve_2(&lines.trim_end());

        // assert_eq!(p2, 2557);
    }
}
