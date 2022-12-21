use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn solve_1(input: &str) -> usize {
    parse_input(input).values().filter(|&&x| x <= 100_000).sum()
}

pub fn solve_2(input: &str) -> usize {
    let total_space: usize = 70_000_000;
    let need: usize = 30_000_000;

    let sizes = parse_input(input);
    let space_used = *sizes.values().max().unwrap();

    let space_to_delete = need - (total_space - space_used);

    *sizes
        .values()
        .filter(|&&x| x > space_to_delete)
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> HashMap<PathBuf, usize> {
    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut seen_files: HashSet<PathBuf> = HashSet::new();
    let mut cwd = PathBuf::new();

    for command_with_result in input.split('$').skip(1) {
        let (command, result) = command_with_result.split_once('\n').unwrap();

        match command.trim() {
            "ls" => {
                for line in result.lines() {
                    let (size, name) = line.split_once(' ').unwrap();

                    if size != "dir" {
                        let abs_path = cwd.join(name);
                        if !seen_files.contains(&abs_path) {
                            seen_files.insert(abs_path.clone());
                            let size: usize = size.parse().unwrap();

                            for p in cwd.ancestors() {
                                let path = PathBuf::from(p);

                                *dir_sizes.entry(path.clone()).or_insert(0) += size;
                            }
                        }
                    }
                }
            }
            "cd .." => {
                cwd.pop();
            }
            cd_dir => {
                let (_, dir) = cd_dir.split_once(' ').unwrap();
                cwd.push(dir);
            }
        }
    }

    return dir_sizes;
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let p1 = solve_1(&simple);

        assert_eq!(p1, 95437);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day07/input");
        let p1 = solve_1(&lines.trim_end());

        assert_eq!(p1, 1513699);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day07/input");
        let p1 = solve_2(&lines.trim_end());

        assert_eq!(p1, 7991939);
    }
}
