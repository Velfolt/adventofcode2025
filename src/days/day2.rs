use std::fs::read_to_string;

use itertools::Itertools;

use aoc::AocDay;

pub struct Day2;

impl AocDay for Day2 {
    fn part1() {
        let file_input = read_to_string("input/day2.txt").unwrap();
        let invalid_ranges: i64 = file_input
            .trim()
            .split(",")
            .map(|s| s.split("-").collect_tuple().unwrap())
            .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            .flat_map(|(low, high)| {
                (low..=high).filter(|value| {
                    let string_value = value.to_string();
                    string_value[0..string_value.len() / 2]
                        == string_value[string_value.len() / 2..]
                })
            })
            .sum();

        println!("{invalid_ranges:?}");
    }

    fn part2() {
        let file_input = read_to_string("input/day2.txt").unwrap();
        let invalid_ranges = file_input
            .trim()
            .split(",")
            .map(|s| s.split("-").collect_tuple().unwrap())
            .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
            .flat_map(|(low, high)| {
                (low..=high).filter(|value| {
                    let string_value = value.to_string();

                    for len in 1..=string_value.len() / 2 {
                        let mut s = String::new();
                        for _ in 0..string_value.len() / len {
                            s.push_str(&string_value[0..len]);
                        }

                        if s == string_value {
                            return true;
                        }
                    }

                    false
                })
            })
            .sum::<i64>();

        println!("{invalid_ranges}");
    }
}

fn invalid_ranges() -> i64 {
    let file_input = read_to_string("input/day2.txt").unwrap();
    file_input
        .trim()
        .split(",")
        .map(|s| s.split("-").collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .flat_map(|(low, high)| {
            (low..=high).filter(|value| {
                let string_value = value.to_string();
                string_value[0..string_value.len() / 2] == string_value[string_value.len() / 2..]
            })
        })
        .sum()
}

fn invalid_ranges2() -> i64 {
    let file_input = read_to_string("input/day2.txt").unwrap();
    file_input
        .trim()
        .split(",")
        .map(|s| s.split("-").collect_tuple().unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .flat_map(|(low, high)| {
            (low..=high).filter(|value| {
                let string_value = value.to_string();

                for len in 1..=string_value.len() / 2 {
                    let mut s = String::new();
                    for _ in 0..string_value.len() / len {
                        s.push_str(&string_value[0..len]);
                    }

                    if s == string_value {
                        return true;
                    }
                }

                false
            })
        })
        .sum::<i64>()
}

// #[cfg(test)]
// mod tests {
//     use crate::days::day2::{invalid_ranges, invalid_ranges2};

//     #[test]
//     fn test_part1() {
//         assert_eq!(invalid_ranges(), 31839939622)
//     }

//     #[test]
//     fn test_part2() {
//         assert_eq!(invalid_ranges2(), 41662374059)
//     }
// }
