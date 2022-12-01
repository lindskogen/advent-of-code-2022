use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub fn solve() -> io::Result<(u32, u32)> {
    let mut file = File::open("src/day01/input")?;


    let mut result_string = String::new();
    file.read_to_string(&mut result_string)?;

    let mut groups: Vec<u32> = result_string.trim_end().split("\n\n").map(|a| a.split("\n").map(|b| b.parse::<u32>().unwrap()).sum()).collect();

    groups.sort_by(|a, b| b.cmp(a));

    return Ok((groups[0], groups[0] + groups[1] + groups[2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (p1, p2) = solve().unwrap();

        assert_eq!(p1, 72478);
        assert_eq!(p2, 210367);
    }
}
