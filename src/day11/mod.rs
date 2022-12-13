use crate::day11::Op::{AddNum, Square, TimesNum};

#[derive(Debug)]
enum Op {
    Square,
    TimesNum(usize),
    AddNum(usize),
}

#[derive(Debug)]
struct Monkey {
    operation: Op,
    test_divisible_by: usize,
    if_true_id: usize,
    if_false_id: usize,
}

impl Monkey {
    fn perform_op(&self, old: &usize) -> usize {
        match self.operation {
            Square => old * old,
            TimesNum(num) => old * num,
            AddNum(num) => old + num,
        }
    }
}

fn parse(input: &str) -> (Vec<Monkey>, Vec<Vec<usize>>) {
    let mut items = vec![];

    let monkeys = input
        .split("\n\n")
        .map(|group| {
            let mut lines = group.lines();

            let monkey_id: usize = lines
                .next()
                .unwrap()
                .split_once(" ")
                .unwrap()
                .1
                .trim_end_matches(':')
                .parse()
                .unwrap();
            let starting_items: Vec<_> = lines
                .next()
                .unwrap()
                .split_once(":")
                .unwrap()
                .1
                .split(",")
                .map(|num| num.trim().parse::<usize>().unwrap())
                .collect();
            let operation = parse_operation(lines.next().unwrap());
            let test_divisible_by: usize = lines
                .next()
                .unwrap()
                .split_once(" by ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let if_true_id: usize = lines
                .next()
                .unwrap()
                .split_once(" monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let if_false_id: usize = lines
                .next()
                .unwrap()
                .split_once(" monkey ")
                .unwrap()
                .1
                .parse()
                .unwrap();

            items.insert(monkey_id as usize, starting_items);

            Monkey {
                operation,
                test_divisible_by,
                if_true_id,
                if_false_id,
            }
        })
        .collect();

    (monkeys, items)
}

fn parse_operation(line: &str) -> Op {
    let mut words = line
        .split_once("=")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .skip(1);
    let op = words.next().unwrap();
    let operand = words.next().unwrap();

    match (op, operand) {
        ("*", "old") => Square,
        ("+", num) => AddNum(num.parse().unwrap()),
        ("*", num) => TimesNum(num.parse().unwrap()),
        _ => unreachable!(),
    }
}

pub fn solve(input: &str, is_part_2: bool) -> u64 {
    let (monkeys, mut items) = parse(input);
    let mut inspections: Vec<u64> = monkeys.iter().map(|_| 0).collect();

    let rounds = if is_part_2 { 10_000 } else { 20 };

    let modulo: usize = monkeys.iter().map(|m| m.test_divisible_by).product();

    println!("mod === {}", modulo);

    for _ in 0..rounds {
        for (index, m) in monkeys.iter().enumerate() {
            for mut item in items[index].clone() {
                item = m.perform_op(&item);
                inspections[index] += 1;
                if !is_part_2 {
                    item /= 3;
                } else {
                    item %= modulo;
                }

                let next_id = if item % m.test_divisible_by == 0 {
                    m.if_true_id
                } else {
                    m.if_false_id
                };

                items[next_id as usize].push(item);
            }
            items[index].clear();
        }
    }

    inspections.sort();

    inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let p1 = solve(&simple, false);

        assert_eq!(p1, 10605);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day11/input");
        let p1 = solve(lines.trim_end(), false);

        assert_eq!(p1, 99852);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day11/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end(), false);
            assert_eq!(p1, 99852);
        })
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day11/input");
        let res = solve(&lines.trim_end(), true);

        assert_eq!(res, 25935263541);
    }
}
