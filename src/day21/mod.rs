use std::collections::HashMap;

enum Monkey<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Eq(&'a str, &'a str),
    Unknown,
    Value(isize),
}

struct Solution<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
}

impl<'a> Solution<'a> {
    fn solve(&self, root_name: &str) -> isize {
        if let Monkey::Eq(m1, m2) = self.monkeys[root_name] {
            if let Some(v) = self.eval(m1) {
                self.recursive_solve(m2, v)
            } else if let Some(v) = self.eval(m2) {
                self.recursive_solve(m1, v)
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
    fn recursive_solve(&self, name: &str, val: isize) -> isize {
        match self.monkeys[name] {
            Monkey::Value(v) => v,
            Monkey::Unknown => val,
            Monkey::Eq(_, _) => unreachable!(),
            Monkey::Add(m1, m2) => {
                if let Some(n) = self.eval(m1) {
                    self.recursive_solve(m2, val - n)
                } else if let Some(n) = self.eval(m2) {
                    self.recursive_solve(m1, val - n)
                } else {
                    unreachable!()
                }
            }
            Monkey::Sub(m1, m2) => {
                if let Some(n) = self.eval(m1) {
                    self.recursive_solve(m2, n - val)
                } else if let Some(n) = self.eval(m2) {
                    self.recursive_solve(m1, val + n)
                } else {
                    unreachable!()
                }
            }
            Monkey::Mul(m1, m2) => {
                if let Some(n) = self.eval(m1) {
                    self.recursive_solve(m2, val / n)
                } else if let Some(n) = self.eval(m2) {
                    self.recursive_solve(m1, val / n)
                } else {
                    unreachable!()
                }
            }
            Monkey::Div(m1, m2) => {
                if let Some(n) = self.eval(m1) {
                    self.recursive_solve(m2, n / val)
                } else if let Some(n) = self.eval(m2) {
                    self.recursive_solve(m1, val * n)
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl Solution<'_> {
    fn eval_name(&self, key: &str) -> isize {
        match self.monkeys[key] {
            Monkey::Value(v) => v,
            Monkey::Add(m1, m2) => self.eval_name(m1) + self.eval_name(m2),
            Monkey::Sub(m1, m2) => self.eval_name(m1) - self.eval_name(m2),
            Monkey::Mul(m1, m2) => self.eval_name(m1) * self.eval_name(m2),
            Monkey::Div(m1, m2) => self.eval_name(m1) / self.eval_name(m2),
            _ => 0,
        }
    }

    fn can_eval(&self, m: &str) -> bool {
        match self.monkeys[m] {
            Monkey::Value(_) => true,
            Monkey::Unknown | Monkey::Eq(_, _) => false,
            Monkey::Add(m1, m2)
            | Monkey::Sub(m1, m2)
            | Monkey::Mul(m1, m2)
            | Monkey::Div(m1, m2) => self.can_eval(m1) && self.can_eval(m2),
        }
    }

    fn eval(&self, m: &str) -> Option<isize> {
        if self.can_eval(m) {
            Some(self.eval_name(m))
        } else {
            None
        }
    }
}

impl Monkey<'_> {
    fn parse(string: &str) -> Monkey {
        if string.chars().next().unwrap().is_ascii_digit() {
            Monkey::Value(string.parse().unwrap())
        } else {
            let mut parts = string.split_whitespace();

            let m1 = parts.next().unwrap();
            let op = parts.next().unwrap();
            let m2 = parts.next().unwrap();

            match op {
                "+" => Monkey::Add(m1, m2),
                "-" => Monkey::Sub(m1, m2),
                "*" => Monkey::Mul(m1, m2),
                "/" => Monkey::Div(m1, m2),
                _ => unreachable!(),
            }
        }
    }
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    input
        .lines()
        .map(|l| {
            let (id, rest) = l.split_once(": ").unwrap();
            (id, Monkey::parse(rest))
        })
        .collect()
}

pub fn solve(input: &str) -> isize {
    let solution = Solution {
        monkeys: parse(input),
    };

    solution.eval_name("root")
}

pub fn solve_2(input: &str) -> isize {
    let mut monkeys = parse(input);
    monkeys.entry("root").and_modify(|v| {
        *v = match v {
            Monkey::Add(m1, m2)
            | Monkey::Sub(m1, m2)
            | Monkey::Mul(m1, m2)
            | Monkey::Div(m1, m2) => Monkey::Eq(m1, m2),
            _ => unreachable!(),
        }
    });

    monkeys.insert("humn", Monkey::Unknown);

    let solution = Solution { monkeys };

    solution.solve("root")
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let p1 = solve(&simple);

        assert_eq!(p1, 152);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day20/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 286698846151845);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day20/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 13183);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day20/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 6676132372578);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        let res = solve_2(simple);

        assert_eq!(res, 301);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day20/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 3759566892641);
    }
}
