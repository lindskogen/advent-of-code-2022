use crate::day13::Package::{List, Num};
use std::cmp::Ordering;
use std::iter;

#[derive(Debug, Eq, PartialEq)]
enum Package {
    List(Vec<Package>),
    Num(u32),
}

impl PartialOrd<Self> for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List(v1), List(v2)) => is_vec_in_right_order(v1, v2),
            (i @ List(_), Num(y)) => i.cmp(&List(vec![Num(*y)])),
            (Num(x), j @ List(_)) => List(vec![Num(*x)]).cmp(j),
            (Num(x), Num(y)) => x.cmp(y),
        }
    }
}

fn parse_line(input: &str) -> (Package, usize) {
    let mut vec = vec![];

    let mut start_num_index = None;

    let str_len = input.len();
    let mut index = 0;
    while index < str_len {
        match &input[index..index + 1] {
            "[" => {
                let (pkg, num_consumed) = parse_line(&input[index + 1..]);
                index += num_consumed;
                vec.push(pkg);
            }
            "]" => {
                if let Some(start_index) = start_num_index.take() {
                    let num = parse_num_from_slice(input, start_index, index);
                    vec.push(Num(num));
                }
                return (List(vec), index + 1);
            }
            "," => {
                if let Some(start_index) = start_num_index.take() {
                    let num = parse_num_from_slice(input, start_index, index);
                    vec.push(Num(num));
                }
            }
            _ => {
                if start_num_index.is_none() {
                    start_num_index = Some(index);
                }
            }
        }
        index += 1;
    }

    let ret = if vec.len() == 1 {
        vec.remove(0)
    } else {
        List(vec)
    };

    (ret, str_len)
}

fn parse_num_from_slice(input: &str, start_index: usize, index: usize) -> u32 {
    input[start_index..index].parse().unwrap()
}

fn parse(input: &str) -> (Package, Package) {
    let (p1, p2) = input.split_once("\n").unwrap();

    let (vec1, _) = parse_line(p1);

    let (vec2, _) = parse_line(p2);

    (vec1, vec2)
}

fn is_vec_in_right_order(v1: &Vec<Package>, v2: &Vec<Package>) -> Ordering {
    let max_len = (v1.len()).max(v2.len());
    let mut last_ordering = Ordering::Equal;
    for i in 0..max_len {
        let ordering = match (v1.get(i), v2.get(i)) {
            (Some(x), Some(y)) => x.cmp(y),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,

            _ => last_ordering,
        };

        if ordering != Ordering::Equal {
            return ordering;
        }

        last_ordering = ordering;
    }

    return last_ordering;
}

pub fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pairs)| {
            let (p1, p2) = parse(pairs);

            if p1 < p2 {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    let mut packages: Vec<_> = input
        .split("\n\n")
        .flat_map(|pairs| {
            let (p1, p2) = parse(pairs);

            iter::once(p1).chain(iter::once(p2))
        })
        .chain(iter::once(List(vec![Num(2)])))
        .chain(iter::once(List(vec![Num(6)])))
        .collect();

    packages.sort();

    packages
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p == &List(vec![Num(2)]) || p == &List(vec![Num(6)]) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_handles_pair_1_example() {
        let res = solve(&"[1,1,3,1,1]\n[1,1,5,1,1]");
        assert_eq!(res, 1)
    }

    #[test]
    fn it_handles_pair_2_example() {
        let res = solve(&"[[1],[2,3,4]]\n[[1],4]");
        assert_eq!(res, 1)
    }
    #[test]
    fn it_handles_pair_3_example() {
        let res = solve(&"[9]\n[[8,7,6]]");
        assert_eq!(res, 0)
    }
    #[test]
    fn it_handles_pair_4_example() {
        let res = solve(&"[[4,4],4,4]\n[[4,4],4,4,4]");
        assert_eq!(res, 1)
    }
    #[test]
    fn it_handles_pair_5_example() {
        let res = solve(&"[7,7,7,7]\n[7,7,7]");
        assert_eq!(res, 0)
    }

    #[test]
    fn it_handles_pair_6_example() {
        let res = solve(&"[]\n[3]");
        assert_eq!(res, 1)
    }

    #[test]
    fn it_handles_pair_7_example() {
        let res = solve(&"[[[]]]\n[[]]");
        assert_eq!(res, 0)
    }

    #[test]
    fn it_handles_pair_8_example() {
        let res = solve(&"[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(res, 0)
    }

    #[test]
    fn it_works_simple() {
        let simple = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let p1 = solve(&simple);

        assert_eq!(p1, 13);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day13/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 6272);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day13/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 6272);
        })
    }
    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day13/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 22288);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let res = solve_2(simple);

        assert_eq!(res, 140);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day13/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 22288);
    }
}
