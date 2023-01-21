use std::collections::VecDeque;
use std::usize;

type MineralCount = [usize; 4];

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: [MineralCount; 4],
}

#[derive(Debug)]
struct State {
    time_elapsed: usize,
    ores: MineralCount,
    robots: MineralCount,
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(&[' ', ':']);

            let id = words.nth(1).unwrap().parse().unwrap();
            let ore_robot_cost = words.nth(5).unwrap().parse().unwrap();
            let clay_robot_cost = words.nth(5).unwrap().parse().unwrap();
            let obsidian_robot_ore_cost = words.nth(5).unwrap().parse().unwrap();
            let obsidian_robot_clay_cost = words.nth(2).unwrap().parse().unwrap();
            let geode_robot_ore_cost = words.nth(5).unwrap().parse().unwrap();
            let geode_robot_obsidian_cost = words.nth(2).unwrap().parse().unwrap();

            Blueprint {
                id,
                costs: [
                    [ore_robot_cost, 0, 0, 0],
                    [clay_robot_cost, 0, 0, 0],
                    [obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0, 0],
                    [geode_robot_ore_cost, 0, geode_robot_obsidian_cost, 0],
                ],
            }
        })
        .collect()
}

pub fn solve(input: &str) -> usize {
    let blueprints = parse(input);

    blueprints
        .iter()
        .map(|b| b.id * evaluate_blueprint(b, 24))
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    let blueprints = parse(input);

    blueprints
        .iter()
        .take(3)
        .map(|b| evaluate_blueprint(b, 32))
        .product()
}

fn evaluate_blueprint(b: &Blueprint, max_time: usize) -> usize {
    let mut max_geodes = 0;

    let mut queue = VecDeque::new();

    let mut max_robots = [u32::MAX as usize; 4];
    for i in 0..3 {
        max_robots[i] = b.costs.iter().map(|cost| cost[i]).max().unwrap();
    }

    queue.push_back(State {
        ores: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
        time_elapsed: 0,
    });

    while let Some(State {
        ores,
        time_elapsed,
        robots,
    }) = queue.pop_front()
    {
        for i in 0..b.costs.len() {
            let costs = &b.costs[i];

            if robots[i] == max_robots[i] {
                continue;
            }

            let wait_time = (0..3)
                .map(|idx| match costs[idx] {
                    cost if cost <= ores[idx] => 0,
                    _ if robots[idx] == 0 => max_time + 1,
                    cost => (cost - ores[idx] + robots[idx] - 1) / robots[idx],
                })
                .max()
                .unwrap();

            let new_elapsed = time_elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            let mut new_ores = [0; 4];
            for idx in 0..robots.len() {
                new_ores[idx] = ores[idx] + robots[idx] * (wait_time + 1) - costs[idx];
            }

            let mut new_robots = robots.clone();
            new_robots[i] += 1;

            let remaining_time = max_time - new_elapsed;
            if ((remaining_time - 1) * remaining_time) / 2
                + new_ores[3]
                + remaining_time * new_robots[3]
                < max_geodes
            {
                continue;
            }

            queue.push_back(State {
                ores: new_ores,
                robots: new_robots,
                time_elapsed: new_elapsed,
            })
        }

        let geodes = ores[3] + robots[3] * (max_time - time_elapsed);

        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::common::read_file_to_string;

    use super::*;

    #[test]
    fn it_works_simple() {
        let simple = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let p1 = solve(&simple);

        assert_eq!(p1, 33);
    }

    #[test]
    fn it_works() {
        let lines = read_file_to_string("src/day19/input");
        let p1 = solve(lines.trim_end());

        assert_eq!(p1, 1681);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let lines = read_file_to_string("src/day19/input");
        b.iter(|| {
            let p1 = solve(lines.trim_end());
            assert_eq!(p1, 1681);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let lines = read_file_to_string("src/day19/input");

        b.iter(|| {
            let res = solve_2(&lines.trim_end());
            assert_eq!(res, 5394);
        })
    }

    #[test]
    fn it_works_simple_2() {
        let simple = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

        let res = solve_2(simple);

        assert_eq!(res, 3472);
    }

    #[test]
    fn it_works_2() {
        let lines = read_file_to_string("src/day19/input");
        let res = solve_2(&lines.trim_end());

        assert_eq!(res, 5394);
    }
}
