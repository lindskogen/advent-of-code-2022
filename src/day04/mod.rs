use std::ops::Range;

trait ContainsOverlapsRange<Idx> {
    fn contains_range(&self, item: &Range<Idx>) -> bool;
    fn overlaps_range(&self, item: &Range<Idx>) -> bool;
}

impl<Idx> ContainsOverlapsRange<Idx> for Range<Idx>
where
    Idx: Ord,
{
    fn contains_range(&self, item: &Range<Idx>) -> bool {
        self.start <= item.start && self.end >= item.end
            || item.start <= self.start && item.end >= self.end
    }

    fn overlaps_range(&self, item: &Range<Idx>) -> bool {
        self.start <= item.start && self.end >= item.start
            || self.start <= item.end && self.end >= item.end
            || item.start <= self.start && item.end >= self.start
            || item.start <= self.end && item.end >= self.end
    }
}

fn solve(lines: &Vec<String>) -> (u32, u32) {
    let mut part_1_score = 0u32;
    let mut part_2_score = 0u32;
    for line in lines {
        let pairs: Vec<_> = line
            .split(',')
            .map(|range| {
                let vec: Vec<_> = range
                    .split('-')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                vec[0]..vec[1]
            })
            .collect();

        let range1 = &pairs[0];
        let range2 = &pairs[1];

        if range1.contains_range(range2) {
            part_1_score += 1
        }

        if range1.overlaps_range(range2) {
            part_2_score += 1
        }
    }

    return (part_1_score, part_2_score);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::map_lines_to_strings;

    #[test]
    fn it_works_simple() {
        let vec = vec![
            String::from("2-4,6-8"),
            String::from("2-3,4-5"),
            String::from("5-7,7-9"),
            String::from("2-8,3-7"),
            String::from("6-6,4-6"),
            String::from("2-6,4-8"),
        ];

        let res = solve(&vec);

        assert_eq!(res, (2, 4));
    }

    #[test]
    fn it_works() {
        let lines = map_lines_to_strings("src/day04/input");
        let res = solve(&lines);

        assert_eq!(res, (466, 865));
    }
}
