use std::collections::HashSet;

pub fn solve_1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut set = HashSet::new();

    for (i, _) in grid.iter().enumerate() {
        set.insert((i, 0));
        set.insert((0, i));
        set.insert((i, grid[i].len() - 1));
        set.insert((grid[i].len() - 1, i));

        let mut max_value = grid[i].first().unwrap();

        for (j, _v) in grid[i][1..grid[i].len()]
            .iter()
            .enumerate()
            .filter(|&(_j, x)| {
                if x > max_value {
                    max_value = x;
                    true
                } else {
                    false
                }
            })
        {
            set.insert((i, 1 + j));
        }

        let mut max_value = grid[i].last().unwrap();

        for (j, _v) in grid[i][1..grid[i].len()]
            .iter()
            .enumerate()
            .rev()
            .filter(|&(_j, x)| {
                if x > max_value {
                    max_value = x;
                    true
                } else {
                    false
                }
            })
        {
            set.insert((i, 1 + j));
        }

        let mut max_value = grid.first().unwrap()[i];

        for (j, _v) in grid[1..grid.len()]
            .iter()
            .map(|a| &a[i])
            .enumerate()
            .filter(|&(_j, x)| {
                if x > &max_value {
                    max_value = *x;
                    true
                } else {
                    false
                }
            })
        {
            set.insert((1 + j, i));
        }

        let mut max_value = grid.last().unwrap()[i];

        for (j, _v) in grid[1..grid.len()]
            .iter()
            .map(|a| a[i])
            .enumerate()
            .rev()
            .filter(|(_j, x)| {
                if x > &max_value {
                    max_value = *x;
                    true
                } else {
                    false
                }
            })
        {
            set.insert((1 + j, i));
        }
    }

    set.len()
}

pub fn solve_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"30373
25512
65332
33549
35390";
        let p1 = solve_1(&simple);

        assert_eq!(p1, 21);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day08/input");
        let p1 = solve_1(&lines.trim_end());

        assert_eq!(p1, 1845);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day08/input");
        let p1 = solve_2(&lines.trim_end());

        assert_eq!(p1, 7991939);
    }
}
