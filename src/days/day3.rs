use std::{cmp::max, fs::read_to_string};

use itertools::Itertools;

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, days::AocDay};

pub struct Day3;

impl AocDay for Day3 {
    fn part1() {
        let file_input = read_to_string("input/day3.txt").unwrap();
        let joltage = joltage(&file_input, 2);
        println!("{joltage}")
    }

    fn part2() {
        let file_input = read_to_string("input/day3.txt").unwrap();
        let joltage = joltage(&file_input, 12);
        println!("{joltage}")
    }
}

fn joltage(input: &str, n: usize) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut number = 0;
            let mut temp = 0;
            let mut i1 = 0;

            for jolt_index in (0..n).rev() {
                for i in i1..(l.len() - jolt_index) {
                    if l[i..i + 1].parse::<i64>().unwrap() > temp {
                        temp = l[i..i + 1].parse::<i64>().unwrap();
                        i1 = i + 1;
                    }
                }

                number += temp * 10_i64.pow(jolt_index as u32);
                temp = 0;
            }

            number
        })
        .sum::<i64>()
}
