use std::fs::File;
use std::io::{BufRead, BufReader};

fn map_lines_to_strings(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum PlayResult {
    Win,
    Lose,
    Draw,
}

fn map_str_1(str: &str) -> Option<Move> {
    match str {
        "A" => Some(Move::Rock),
        "B" => Some(Move::Paper),
        "C" => Some(Move::Scissors),

        "X" => Some(Move::Rock),
        "Y" => Some(Move::Paper),
        "Z" => Some(Move::Scissors),
        _ => None
    }
}

fn map_str2(str: &str) -> Option<PlayResult> {
    match str {
        "X" => Some(PlayResult::Lose),
        "Y" => Some(PlayResult::Draw),
        "Z" => Some(PlayResult::Win),
        _ => None
    }
}

fn calculate_move_score(m: &Move) -> u32 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    }
}

fn calculate_win_score(r: &PlayResult) -> u32 {
    match r {
        PlayResult::Lose => 0,
        PlayResult::Draw => 3,
        PlayResult::Win => 6
    }
}

fn calculate_round_win_score(m1: &Move, m2: &Move) -> u32 {
    let result = match (m1, m2) {
        (Move::Rock, Move::Scissors) => PlayResult::Win,
        (Move::Rock, Move::Paper) => PlayResult::Lose,
        (Move::Scissors, Move::Paper) => PlayResult::Win,
        (Move::Scissors, Move::Rock) => PlayResult::Lose,
        (Move::Paper, Move::Rock) => PlayResult::Win,
        (Move::Paper, Move::Scissors) => PlayResult::Lose,
        _ => PlayResult::Draw
    };

    calculate_move_score(m1) + calculate_win_score(&result)
}

pub fn solve(lines: &Vec<String>) -> u32 {
    let mut sum = 0u32;

    for x in lines {
        let vec: Vec<_> = x.split(" ").collect();

        let m2 = map_str_1(vec[0]).unwrap();
        let m1 = map_str_1(vec[1]).unwrap();

        sum += calculate_round_win_score(&m1, &m2);
    }

    sum
}

fn get_move_from_result(r: &PlayResult, opponent_move: &Move) -> Move {
    match (r, opponent_move) {
        (PlayResult::Win, Move::Scissors) => Move::Rock,
        (PlayResult::Lose, Move::Paper) => Move::Rock,
        (PlayResult::Win, Move::Paper) => Move::Scissors,
        (PlayResult::Lose, Move::Rock) => Move::Scissors,
        (PlayResult::Win, Move::Rock) => Move::Paper,
        (PlayResult::Lose, Move::Scissors) => Move::Paper,
        (PlayResult::Draw, &a) => a
    }
}

pub fn solve2(lines: &Vec<String>) -> u32 {
    let mut sum = 0u32;

    for x in lines {
        let vec: Vec<_> = x.split(" ").collect();

        let m1 = map_str_1(vec[0]).unwrap();
        let result = map_str2(vec[1]).unwrap();
        let m2 = get_move_from_result(&result, &m1);

        let round_score = calculate_move_score(&m2) + calculate_win_score(&result);

        sum += round_score;
    }

    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_simple() {
        let sum = solve(&vec![String::from("A Y"), String::from("B X"), String::from("C Z")]);

        assert_eq!(sum, 15);
    }

    #[test]
    fn it_works() {
        let lines = map_lines_to_strings("src/day02/input");
        let sum = solve(&lines);

        assert_eq!(sum, 13484);
    }

    #[test]
    fn it_works_part_2() {
        let lines = map_lines_to_strings("src/day02/input");
        let sum = solve2(&lines);

        assert_eq!(sum, 13433);
    }
}
