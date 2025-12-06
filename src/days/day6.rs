use std::fs::read_to_string;

use aoc::AocDay;
use itertools::Itertools;

pub struct Day6;

impl AocDay for Day6 {
    fn part1() {
        let file_input = read_to_string("input/day6.txt").unwrap();
        let data = file_input
            .trim()
            .lines()
            .map(|l| l.trim().split_ascii_whitespace().collect_vec())
            .collect_vec();

        let sum = data
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, operator)| match *operator {
                "+" => data
                    .iter()
                    .map(|n| n[i].parse::<i64>().unwrap_or(0))
                    .sum::<i64>(),
                "*" => data
                    .iter()
                    .map(|n| n[i].parse::<i64>().unwrap_or(1))
                    .product::<i64>(),
                _ => unimplemented!(),
            })
            .sum::<i64>();

        println!("{sum}");
    }

    fn part2() {
        let input = read_to_string("input/day6.txt").unwrap();

        let operators = input.trim().lines().last().unwrap();

        let mut ops = vec![];
        for (i, c) in operators.chars().enumerate() {
            if let '+' | '*' = c {
                ops.push((i, c))
            } else {
                continue;
            }
        }

        let mut data = vec![];
        for line in input.trim().lines() {
            let mut i = 0;
            let mut numbers = vec![];
            for (distance, _) in ops.iter().skip(1) {
                numbers.push(&line[i..(*distance - 1)]);
                i = *distance;
            }
            numbers.push(&line[i..]);
            data.push(numbers);
        }

        let sum = data
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, operator)| match operator.trim() {
                "+" => (0..5)
                    .map(|b| {
                        data.iter()
                            .filter(|n| !n[i].starts_with("+") && !n[i].starts_with("*"))
                            .filter_map(|(n)| n[i].as_bytes().get(b).or(None))
                            .filter(|b| **b as char != ' ')
                            .rev()
                            .enumerate()
                            .map(|(i, n)| (*n as i64 - '0' as i64) * 10_i64.pow(i as u32))
                            .sum::<i64>()
                    })
                    .sum::<i64>(),
                "*" => (0..5)
                    .map(|b| {
                        data.iter()
                            .filter(|n| !n[i].starts_with("+") && !n[i].starts_with("*"))
                            .filter_map(|(n)| n[i].as_bytes().get(b).or(None))
                            .filter(|b| **b as char != ' ')
                            .rev()
                            .enumerate()
                            .map(|(i, n)| (*n as i64 - '0' as i64) * 10_i64.pow(i as u32))
                            .sum::<i64>()
                    })
                    .filter(|n| *n != 0)
                    .product(),
                _ => unimplemented!(),
            })
            .sum::<i64>();

        println!("{sum}");
    }
}
