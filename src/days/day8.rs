use std::fs::read_to_string;

use aoc::{AocDay, utils::Distance};
use disjoint::DisjointSetVec;
use itertools::Itertools;

pub struct Day8;

impl AocDay for Day8 {
    fn part1() {
        let input = read_to_string("input/day8.txt").unwrap();

        let junction_boxes = input
            .trim()
            .lines()
            .filter_map(|l| {
                l.split(",")
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
            })
            .collect_vec();

        let permutations = junction_boxes
            .iter()
            .combinations(2)
            .map(|x| x.iter().cloned().collect_tuple::<(_, _)>().unwrap())
            .map(|(a, b)| ((*a, *b).distance(), *a, *b))
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0));

        let mut circuits = DisjointSetVec::from(junction_boxes);

        for (_, a, b) in permutations.take(1000) {
            let (i1, _) = circuits.clone().iter().find_position(|x| **x == a).unwrap();
            let (i2, _) = circuits.clone().iter().find_position(|x| **x == b).unwrap();

            if i1 == i2 {
                continue;
            } else {
                circuits.join(i1, i2);
            }
        }

        let circuits = circuits.indices().sets();

        let product = circuits
            .iter()
            .map(|x| x.len() as i64)
            .sorted()
            .rev()
            .take(3)
            .product::<i64>();

        println!("{product:?}");
    }

    fn part2() {
        let input = read_to_string("input/day8.txt").unwrap();

        let junction_boxes = input
            .trim()
            .lines()
            .filter_map(|l| {
                l.split(",")
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64, i64)>()
            })
            .collect_vec();

        let permutations = junction_boxes
            .iter()
            .combinations(2)
            .map(|x| x.iter().cloned().collect_tuple::<(_, _)>().unwrap())
            .map(|(a, b)| ((*a, *b).distance(), *a, *b))
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0));

        let mut circuits = DisjointSetVec::from(junction_boxes);

        for (_, a, b) in permutations {
            let (i1, _) = circuits.clone().iter().find_position(|x| **x == a).unwrap();
            let (i2, _) = circuits.clone().iter().find_position(|x| **x == b).unwrap();

            if i1 == i2 {
                continue;
            } else {
                circuits.join(i1, i2);
            }

            if circuits.indices().sets().len() == 1 {
                println!("{}", a.0 * b.0);
                break;
            }
        }
    }
}
