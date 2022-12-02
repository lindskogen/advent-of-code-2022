
pub fn solve(lines: &Vec<String>) -> (u32, u32) {

    let score1 = lines.iter().fold(0, |acc, line| acc + match line.as_str()
    {
        "C X" => 6 + 1,
        "A Y" => 6 + 2,
        "B Z" => 6 + 3,
        "A X" => 3 + 1,
        "B Y" => 3 + 2,
        "C Z" => 3 + 3,
        "B X" => 0 + 1,
        "C Y" => 0 + 2,
        "A Z" => 0 + 3,
        _ => panic!()
    });

    let score2 = lines.iter().fold(0, |acc, line| acc + match line.as_str()
    {
        "C X" => 0 + 2,
        "A Y" => 3 + 1,
        "B Z" => 6 + 3,
        "A X" => 0 + 3,
        "B Y" => 3 + 2,
        "C Z" => 6 + 1,
        "B X" => 0 + 1,
        "C Y" => 3 + 3,
        "A Z" => 6 + 2,
        _ => panic!()
    });


    (score1, score2)
}

#[cfg(test)]
mod tests {
    use crate::common::map_lines_to_strings;
    use super::*;

    #[test]
    fn it_works_simple() {
        let (sum1, _) = solve(&vec![String::from("A Y"), String::from("B X"), String::from("C Z")]);

        assert_eq!(sum1, 15);
    }

    #[test]
    fn it_works() {
        let lines = map_lines_to_strings("src/day02/input");
        let (p1, p2) = solve(&lines);

        assert_eq!(p1, 13484);
        assert_eq!(p2, 13433);
    }
}
