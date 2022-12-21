use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

const POS_DELTAS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_neighs(adj_list: &[Vec<usize>], (x, y): Position) -> impl Iterator<Item = Position> + '_ {
    let my_height = adj_list[y][x];

    POS_DELTAS.iter().filter_map(move |(dx, dy)| {
        let x = (x as isize + dx) as usize;
        let y = (y as isize + dy) as usize;

        adj_list.get(y).and_then(|row| row.get(x)).and_then(|n| {
            if &my_height - 1 <= *n {
                Some((x, y))
            } else {
                None
            }
        })
    })
}

fn shortest_path<F>(adj_list: &[Vec<usize>], start: Position, is_goal: F) -> Option<usize>
where
    F: Fn(&Position) -> bool,
{
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Position, usize> = adj_list
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, _)| ((x, y), usize::MAX))
        })
        .collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State {
        position: start,
        cost: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { position, cost }) = heap.pop() {
        if is_goal(&position) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for pos in get_neighs(adj_list, position) {
            let next_cost = cost + 1;

            // If so, add it to the frontier and continue
            if next_cost < dist[&pos] {
                heap.push(State {
                    position: pos,
                    cost: next_cost,
                });
                // Relaxation, we have now found a better way
                dist.insert(pos, next_cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Position, Position) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    (
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (if c == 'S' {
                            start = (x, y);
                            'a'
                        } else if c == 'E' {
                            end = (x, y);
                            'z'
                        } else {
                            c
                        }) as usize
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    )
}

pub fn solve(input: &str) -> usize {
    let (grid, goal, start) = parse(input);

    shortest_path(&grid, start, |pos| pos == &goal).unwrap()
}

pub fn solve_2(input: &str) -> usize {
    let (grid, _start, goal) = parse(input);

    shortest_path(&grid, goal, |&(x, y)| grid[y][x] == 'a' as usize).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file_to_string;
    use test::Bencher;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let p1 = solve(&simple);

        assert_eq!(p1, 31);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day12/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 330);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day12/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 330);
        })
    }
    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day12/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 321);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let res = solve_2(simple);

        assert_eq!(res, 29);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day12/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 321);
    }
}
