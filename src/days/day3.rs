use aoc::AocDay;
use std::fs::read_to_string;

pub struct Day3;

impl AocDay for Day3 {
    fn part1() {
        let file_input = read_to_string("input/day3.txt").unwrap();
        let joltage = largest_joltage(&file_input, 2);
        println!("{joltage}")
    }

    fn part2() {
        let file_input = read_to_string("input/day3.txt").unwrap();
        let joltage = largest_joltage(&file_input, 12);
        println!("{joltage}")
    }
}

fn largest_joltage(input: &str, batteries: usize) -> i64 {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut largest_joltage = 0;
            let mut index = 0;

            for battery_index in (0..batteries).rev() {
                let mut max_in_sequence = 0;
                for i in index..(l.len() - battery_index) {
                    let value = l.as_bytes()[i] as i64 - '0' as i64;
                    if value > max_in_sequence {
                        max_in_sequence = value;
                        index = i + 1;
                    }
                }

                largest_joltage += max_in_sequence * 10_i64.pow(battery_index as u32);
            }

            largest_joltage
        })
        .sum::<i64>()
}
