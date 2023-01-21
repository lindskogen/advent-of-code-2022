use std::fmt::{Debug, Formatter};

struct ListNumber {
    original_index: usize,
    num: isize,
}

impl Debug for ListNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

fn parse(input: &str) -> Vec<ListNumber> {
    input
        .lines()
        .enumerate()
        .map(|(original_index, num)| ListNumber {
            num: num.parse().unwrap(),
            original_index,
        })
        .collect()
}

pub fn solve(input: &str) -> isize {
    let mut numbers = parse(input);

    let max = numbers.len() - 1;

    mix_numbers(&mut numbers, max);

    let zero_index = numbers.iter().position(|n| n.num == 0).unwrap();

    let idx1 = (zero_index + 1000) % (max + 1);
    let idx2 = (zero_index + 2000) % (max + 1);
    let idx3 = (zero_index + 3000) % (max + 1);

    numbers[idx1].num + numbers[idx2].num + numbers[idx3].num
}

fn mix_numbers(numbers: &mut Vec<ListNumber>, max: usize) {
    for original_index in 0..numbers.len() {
        let (index, &ListNumber { num, .. }) = numbers
            .iter()
            .enumerate()
            .find(|(_, n)| n.original_index == original_index)
            .unwrap();

        if num != 0 {
            let new_index = add_index(index, num, max);

            let temp = numbers.remove(index);

            numbers.insert(new_index, temp);
        }
    }
}

fn add_index(prev: usize, diff: isize, max: usize) -> usize {
    (prev as isize + diff).rem_euclid(max as isize) as usize
}

pub fn solve_2(input: &str) -> isize {
    let decryption_key: isize = 811_589_153;
    let mut numbers = parse(input);

    for n in numbers.iter_mut() {
        n.num *= decryption_key;
    }

    let max = numbers.len() - 1;

    for _ in 0..10 {
        mix_numbers(&mut numbers, max);
    }

    let zero_index = numbers.iter().position(|n| n.num == 0).unwrap();

    let idx1 = (zero_index + 1000) % (max + 1);
    let idx2 = (zero_index + 2000) % (max + 1);
    let idx3 = (zero_index + 3000) % (max + 1);

    numbers[idx1].num + numbers[idx2].num + numbers[idx3].num
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_adds_wrapping() {
        assert_eq!(add_index(6, 5, 9), 2);
    }
    #[test]
    fn it_adds_wrapping_2() {
        assert_eq!(add_index(6, 4, 9), 1);
    }
    #[test]
    fn it_adds_wrapping_3() {
        assert_eq!(add_index(6, 3, 9), 0);
    }
    #[test]
    fn it_adds_wrapping_4() {
        assert_eq!(add_index(5, -6, 9), 8);
    }
    #[test]
    fn it_adds_wrapping_5() {
        assert_eq!(add_index(4, -6, 9), 7);
    }
    #[test]
    fn it_adds_wrapping_6() {
        assert_eq!(add_index(3, -6, 9), 6);
    }
    #[test]
    fn it_adds_wrapping_7() {
        assert_eq!(add_index(2, -6, 4), 0);
    }
    #[test]
    fn it_adds_wrapping_8() {
        assert_eq!(add_index(2, 6, 4), 0);
    }
    #[test]
    fn it_adds_wrapping_9() {
        assert_eq!(add_index(2, 0, 4), 2);
    }

    #[test]
    fn it_works_simple() {
        let simple = r"1
2
-3
3
-2
0
4";
        let p1 = solve(&simple);

        assert_eq!(p1, 3);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day20/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 13183);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day20/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 13183);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day20/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 6676132372578);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"1
2
-3
3
-2
0
4";

        let res = solve_2(simple);

        assert_eq!(res, 1623178306);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day20/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 6676132372578);
    }
}
