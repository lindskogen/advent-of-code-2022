use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve<'a> {
    flow_rate: usize,
    neighbors: Vec<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    opened_valves: u64,
    position: &'a str,
    time_left: usize,
    number_others: usize,
}

impl<'a> State<'a> {
    fn contains_valve(&self, valve_index: u8) -> bool {
        (self.opened_valves & 1 << valve_index) > 0
    }

    fn move_to(&self, pos: &'a str) -> Self {
        State {
            opened_valves: self.opened_valves,
            position: pos,
            time_left: self.time_left - 1,
            number_others: self.number_others,
        }
    }

    fn open_valve(&self, valve_index: u8) -> Self {
        let opened_valves = self.opened_valves | 1 << valve_index;

        State {
            opened_valves,
            position: self.position,
            time_left: self.time_left - 1,
            number_others: self.number_others,
        }
    }

    fn next_player(&self) -> Self {
        State {
            opened_valves: self.opened_valves,
            position: "AA",
            time_left: 26,
            number_others: self.number_others - 1,
        }
    }
}

struct Network<'a> {
    valve_indices: HashMap<String, u8>,
    valves: HashMap<String, Valve<'a>>,
    table: RefCell<HashMap<State<'a>, usize>>,
}

impl<'a> Network<'a> {
    fn recur(&self, state: State<'a>) -> usize {
        if state.time_left == 0 {
            return if state.number_others > 0 {
                self.recur(state.next_player())
            } else {
                0
            };
        }

        if let Some(&res) = self.table.borrow().get(&state) {
            return res;
        }

        let node = &self.valves[state.position];

        let max_open =
            if !state.contains_valve(self.valve_indices[state.position]) && node.flow_rate > 0 {
                (state.time_left - 1) * node.flow_rate
                    + self.recur(state.open_valve(self.valve_indices[state.position]))
            } else {
                0
            };

        let max_move = node
            .neighbors
            .iter()
            .map(|n| self.recur(state.move_to(n)))
            .max()
            .unwrap();

        let best_value = max_open.max(max_move);

        self.table.borrow_mut().insert(state, best_value);

        best_value
    }

    fn parse(input: &'a str) -> Self {
        let valves: HashMap<_, _> = input
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
            .collect();

        let valve_indices = valves
            .iter()
            .enumerate()
            .map(|(index, (id, valve))| (id.to_string(), index as u8))
            .collect();

        Network {
            table: Default::default(),
            valves,
            valve_indices,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let n = Network::parse(input);

    n.recur(State {
        opened_valves: 0,
        time_left: 30,
        number_others: 0,
        position: "AA",
    })
}

pub fn solve_2(input: &str) -> usize {
    let n = Network::parse(input);

    n.recur(State {
        opened_valves: 0,
        time_left: 26,
        number_others: 1,
        position: "AA",
    })
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

        assert_eq!(p1, 1651);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day16/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 1991);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day16/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 1991);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day16/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 2705);
        })
    }

    #[test]
    fn it_works_simple_2() {
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

        let res = solve_2(simple);

        assert_eq!(res, 1707);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day16/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 2705);
    }
}
