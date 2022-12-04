use std::collections::HashSet;

pub fn solve(lines: &Vec<String>) -> u32 {
    let mut score = 0u32;
    for line in lines {
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();

        for (i, char) in line.char_indices() {
            if i < line.len() / 2 {
                s1.insert(char);
            } else {
                s2.insert(char);
            }
        }

        let set = &s1 & &s2;
        let i: Vec<_> = set.iter().collect();
        assert_eq!(i.len(), 1);
        let c = *i[0];

        if c.is_ascii_lowercase() {
            score += 1 + u32::from(c) - u32::from('a');
        } else if c.is_ascii_uppercase() {
            score += 27 + u32::from(c) - u32::from('A');
        } else {
            println!("c={}", c);
            panic!("Not ascii?");
        }
    }

    return score;
}

pub fn solve_2(lines: &Vec<String>) -> u32 {
    let mut score = 0u32;

    for line in lines.chunks(3) {
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        let mut s3 = HashSet::new();

        for char in line[0].chars() {
            s1.insert(char);
        }
        for char in line[1].chars() {
            s2.insert(char);
        }
        for char in line[2].chars() {
            s3.insert(char);
        }

        let set = &(&s1 & &s2) & &s3;

        let i: Vec<_> = set.iter().collect();

        assert_eq!(i.len(), 1);
        let c = *i[0];

        if c.is_ascii_lowercase() {
            score += 1 + u32::from(c) - u32::from('a');
        } else if c.is_ascii_uppercase() {
            score += 27 + u32::from(c) - u32::from('A');
        } else {
            println!("c={}", c);
            panic!("Not ascii?");
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::map_lines_to_strings;

    #[test]
    fn it_works_simple() {
        let vec = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let sum1 = solve(&vec);

        assert_eq!(sum1, 157);
    }

    #[test]
    fn it_works_simple_p2() {
        let vec = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let sum1 = solve_2(&vec);

        assert_eq!(sum1, 70);
    }

    #[test]
    fn it_works() {
        let lines = map_lines_to_strings("src/day03/input");
        let p1 = solve(&lines);

        assert_eq!(p1, 7850);
    }

    #[test]
    fn it_works_p2() {
        let lines = map_lines_to_strings("src/day03/input");
        let p1 = solve_2(&lines);

        assert_eq!(p1, 2581);
    }
}
