use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve<'a> {
    flow_rate: usize,
    neighbors: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    opened_valves: BTreeSet<&'a str>,
    position: &'a str,
    time_left: usize,
}

impl<'a> State<'a> {
    fn move_to(&self, pos: &'a str) -> Self {
        State {
            opened_valves: self.opened_valves.clone(),
            position: pos,
            time_left: self.time_left - 1,
        }
    }
    fn open_valve(&self, valve: &'a str) -> Self {
        let mut new_set = self.opened_valves.clone();
        new_set.insert(valve);

        State {
            opened_valves: new_set,
            position: self.position,
            time_left: self.time_left - 1,
        }
    }
}

struct Network<'a> {
    valves: HashMap<String, Valve<'a>>,
    table: RefCell<HashMap<State<'a>, usize>>,
}

impl<'a> Network<'a> {
    fn recur(&self, state: State<'a>) -> usize {
        let State {
            time_left,
            position,
            ..
        } = state;

        if time_left == 0 {
            return 0;
        }

        if let Some(&res) = self.table.borrow().get(&state) {
            return res;
        }

        let node = &self.valves[position];

        let max_open = if node.flow_rate > 0 {
            self.recur(state.open_valve(&position))
        } else {
            0
        };

        let (max_move_node, max_move) = node
            .neighbors
            .iter()
            .map(|n| {
                (
                    n,
                    (time_left - 1) * node.flow_rate + self.recur(state.move_to(n)),
                )
            })
            .max_by_key(|a| a.1)
            .unwrap();

        if max_move > max_open {
            println!("TIME: {time_left}, MOVE TO {max_move_node}");
        } else {
            println!("TIME: {time_left}, OPEN {}", position);
        }

        let best_value = max_open.max(max_move);

        self.table.borrow_mut().insert(state, best_value);

        best_value
    }
}

fn parse(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once("; ").unwrap();
            let mut words = first.split(&[' ', '=']).skip(1);

            let id = words.next().unwrap().to_string();
            let flow_rate: usize = words.skip(3).next().unwrap().parse().unwrap();

            let valves = if let Some((_, valves)) = second.split_once(" valves ") {
                valves.split(", ").collect()
            } else {
                let (_, valve) = second.split_once(" valve ").unwrap();
                vec![valve]
            };

            (
                id,
                Valve {
                    flow_rate,
                    neighbors: valves,
                },
            )
        })
        .collect()
}

pub fn solve(input: &str) -> usize {
    let valves = parse(input);

    let mut n = Network {
        table: Default::default(),
        valves,
    };

    n.recur(State {
        opened_valves: Default::default(),
        time_left: 30,
        position: "AA",
    })
}

pub fn solve_2(input: &str, search_space: isize) -> usize {
    5
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let p1 = solve(&simple);

        assert_eq!(p1, 26);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day15/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 5335787);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day15/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 5335787);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day15/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end(), 4000000);
            assert_eq!(res, 13673971349056);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let res = solve_2(simple, 20);

        assert_eq!(res, 56000011);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day15/input");
        let res = solve_2(&lines.trim_end(), 4000000);

        assert_eq!(res, 13673971349056);
    }
}
