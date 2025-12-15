use itertools::Itertools;
use std::fs::read_to_string;

use aoc::AocDay;

pub struct Day9;
impl AocDay for Day9 {
    fn part1() {
        let input = read_to_string("input/day9.txt").unwrap();

        let a = input
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap()
            })
            .combinations(2)
            .map(|x| {
                let (a, b) = x.iter().collect_tuple().unwrap();
                let x = (a.0 - b.0).abs() + 1;
                let y = (a.1 - b.1).abs() + 1;

                x * y
            })
            .max()
            .unwrap();

        println!("{a:?}");
    }

    fn part2() {
        let input = read_to_string("input/day9.txt").unwrap();

        let points = input
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap()
            })
            .collect_vec();

        let doublepoints = points
            .iter()
            .cycle()
            .take(points.len() + 1)
            .cloned()
            .collect_vec();

        let edges = doublepoints
            .iter()
            .zip(doublepoints.iter().skip(1))
            .collect_vec();

        let a = points
            .iter()
            .combinations(2)
            .filter_map(|x| {
                let (p1, p2) = x.iter().collect_tuple().unwrap();

                let (xmin, xmax) = if p1.0 <= p2.0 {
                    (p1.0, p2.0)
                } else {
                    (p2.0, p1.0)
                };
                let (ymin, ymax) = if p1.1 <= p2.1 {
                    (p1.1, p2.1)
                } else {
                    (p2.1, p1.1)
                };

                // check that the entire box is inside all edges
                if edges.iter().all(|(a, b)| {
                    // two cases: horizontal and vertical
                    if a.1 == b.1 {
                        a.1 >= ymax
                            || a.1 <= ymin
                            || (a.0 >= xmax && b.0 >= xmax)
                            || (a.0 <= xmin && b.0 <= xmin)
                    } else {
                        a.0 >= xmax
                            || a.0 <= xmin
                            || (a.1 >= ymax && b.1 >= ymax)
                            || (a.1 <= ymin && b.1 <= ymin)
                    }
                }) {
                    let x = (p1.0 - p2.0).abs() + 1;
                    let y = (p1.1 - p2.1).abs() + 1;

                    Some(x * y)
                } else {
                    None
                }
            })
            .max()
            .unwrap();

        println!("{a:?}");
    }
}
