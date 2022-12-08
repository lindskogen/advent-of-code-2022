use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn solve_1(input: &str) -> usize {
    let grid = parse(input);

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

pub fn solve_2(input: &str) -> usize {
    let grid = parse(input);

    let mut max_score = 0;

    for y in 1..grid.len() {
        for x in 1..grid.len() {
            let cand = grid[y][x];

            let mut up = 0;

            for i in (0..y).rev() {
                if grid[i][x] >= cand {
                    up += 1;
                    break;
                } else {
                    up += 1;
                }
            }

            let mut down = 0;
            for i in (y + 1)..grid.len() {
                if grid[i][x] >= cand {
                    down += 1;
                    break;
                } else {
                    down += 1;
                }
            }

            let mut left = 0;

            for i in (0..x).rev() {
                if grid[y][i] >= cand {
                    left += 1;
                    break;
                } else {
                    left += 1;
                }
            }

            let mut right = 0;

            for i in (x + 1)..grid.len() {
                if grid[y][i] >= cand {
                    right += 1;
                    break;
                } else {
                    right += 1;
                }
            }

            let score = up * down * left * right;

            if score > max_score {
                max_score = score;
            }
        }
    }

    return max_score;
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
    fn it_works_simple_2() {
        let simple = r"30373
25512
65332
33549
35390";
        let p1 = solve_2(&simple);

        assert_eq!(p1, 8);
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

        assert_eq!(p1, 230112);
    }
}
