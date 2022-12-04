use crate::common::read_file_to_string;

pub fn solve() -> (u32, u32) {
    let result_string = read_file_to_string("src/day01/input");

    let mut groups: Vec<u32> = result_string
        .trim_end()
        .split("\n\n")
        .map(|a| a.split("\n").map(|b| b.parse::<u32>().unwrap()).sum())
        .collect();

    groups.sort_by(|a, b| b.cmp(a));

    return (groups[0], groups[0] + groups[1] + groups[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (p1, p2) = solve();

        assert_eq!(p1, 72478);
        assert_eq!(p2, 210367);
    }
}
