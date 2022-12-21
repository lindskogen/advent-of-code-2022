fn parse_state(state: &str) -> Vec<Vec<char>> {
    let vec: Vec<_> = state.lines().collect();

    let mut state = (0..9)
        .map(|_| Vec::with_capacity(50))
        .collect::<Vec<Vec<char>>>();

    for i in 0..9 {
        let index = 1 + (4 * i);
        for r in &vec {
            let x = r.as_bytes()[index];
            if x.is_ascii_uppercase() {
                state[i].insert(0, x as char)
            }
        }
    }

    state
}

pub fn solve(input: &str, is_part_2: bool) -> Option<String> {
    let (state, instrs) = input.split_once("\n\n").expect("No double newline found");

    let mut state = parse_state(state);

    for l in instrs.lines() {
        let mut inst = l.split_whitespace().filter_map(|i| i.parse::<u32>().ok());

        let (num, from, to) = (
            inst.next()? as usize,
            inst.next()? as usize - 1,
            inst.next()? as usize - 1,
        );

        let idx = state[from].len() - num;
        let c = state[from].split_off(idx);
        if is_part_2 {
            state[to].extend(c.iter());
        } else {
            state[to].extend(c.iter().rev());
        }
    }

    Some(state.iter().map(|row| row.last().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_file_to_string;
    use test::{black_box, Bencher};

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day05/input");
        let p1 = solve(&lines, false).unwrap();

        assert_eq!(p1, "PSNRGBTFT");
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day05/input");
        b.iter(|| {
            black_box(solve(&lines, false));
        })
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day05/input");
        b.iter(|| {
            black_box(solve(&lines, false));
        })
    }

    #[test]
    fn it_works_p2() {
        let lines = read_file_to_string("src/day05/input");
        let p1 = solve(&lines, true).unwrap();

        assert_eq!(p1, "BNTZFPMMW");
    }
}
