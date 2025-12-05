use std::fs::read_to_string;

use itertools::Itertools;

use crate::days::AocDay;

pub struct Day5;

impl AocDay for Day5 {
    fn part1() {
        let file_input = read_to_string("input/day5.txt").unwrap();
        let (fresh, ids) = file_input.trim().split("\n\n").collect_tuple().unwrap();
        let fresh = fresh
            .lines()
            .map(|l| {
                l.split("-")
                    .map(|a| a.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(a, b)| a..=b)
            .collect_vec();
        let ids = ids.lines().map(|l| l.parse::<i64>().unwrap());

        let number_of_fresh_ingredients = ids
            .filter(|id| {
                for range in &fresh {
                    if range.contains(id) {
                        return true;
                    }
                }
                false
            })
            .count();

        println!("{number_of_fresh_ingredients}")
    }

    fn part2() {
        let file_input = read_to_string("input/day5.txt").unwrap();
        let (fresh, _) = file_input.trim().split("\n\n").collect_tuple().unwrap();
        let mut ranges = fresh
            .lines()
            .map(|l| {
                l.split("-")
                    .map(|a| a.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(a, b)| (a, b))
            .collect_vec();

        loop {
            let ranges_clone = ranges.iter().cloned().rev().collect_vec();

            ranges = vec![];

            let mut changed = false;

            for range in &ranges_clone {
                if let Some(range_to_grow) = ranges
                    .iter_mut()
                    .find(|(a, b)| (*a..=*b).contains(&range.0) || (*a..=*b).contains(&range.1))
                {
                    if range.0 < range_to_grow.0 && range.1 > range_to_grow.1 {
                        range_to_grow.0 = range.0;
                        range_to_grow.1 = range.1;
                        changed = true;
                    }

                    if range.0 < range_to_grow.0 {
                        range_to_grow.0 = range.0;
                        changed = true;
                    }

                    if range.1 > range_to_grow.1 {
                        range_to_grow.1 = range.1;
                        changed = true;
                    }
                } else {
                    ranges.push((range.0, range.1));
                }
            }

            if !changed {
                break;
            }
        }

        let sum = ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<i64>();

        println!("{sum}")
    }
}
