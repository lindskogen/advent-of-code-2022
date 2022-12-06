const fn find_first_non_uniq(slice: &[u8], from: usize, to: usize) -> Option<usize> {
    let mut i = from;

    while i < to {
        let mut j = i + 1;
        while j < to {
            if slice[i] == slice[j] {
                return Some(i);
            }
            j += 1;
        }
        i += 1;
    }

    return None;
}

pub const fn solve(input: &str, is_part_2: bool) -> Option<usize> {
    let length = if is_part_2 { 14usize } else { 4usize };

    let mut i = 0usize;

    while i < input.len() {
        if let Some(idx) = find_first_non_uniq(&input.as_bytes(), i, i + length) {
            i = idx + 1;
        } else {
            return Some(i + length);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::read_file_to_string;
    use test::{black_box, Bencher};

    #[test]
    fn uniq_works() {
        assert_eq!(find_first_non_uniq("mjqj".as_bytes(), 0, 4), Some(1));
    }

    #[test]
    fn uniq_works_2() {
        assert_eq!(find_first_non_uniq("abcd".as_bytes(), 0, 4), None);
    }

    #[test]
    fn it_works_simple() {
        let lines = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let p1 = solve(&lines, true).unwrap();

        assert_eq!(p1, 19);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day06/input");
        let p1 = solve(&lines, false).unwrap();

        assert_eq!(p1, 1707);
    }

    #[test]
    fn it_works_p2() {
        let lines = read_file_to_string("src/day06/input");
        let p1 = solve(&lines, true).unwrap();

        assert_eq!(p1, 3697);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day06/input");
        b.iter(|| {
            black_box(solve(&lines, false));
        })
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day06/input");
        b.iter(|| {
            black_box(solve(&lines, false));
        })
    }

    #[bench]
    fn bench_part1_const(b: &mut Bencher) {
        const LINES: &str = include_str!("./input");
        b.iter(|| {
            const RES: Option<usize> = solve(&LINES, false);
            assert_eq!(RES, Some(1707));
        })
    }

    #[bench]
    fn bench_part2_const(b: &mut Bencher) {
        const LINES: &str = include_str!("./input");
        b.iter(|| {
            const RES: Option<usize> = solve(&LINES, true);
            assert_eq!(RES, Some(3697));
        })
    }
}
