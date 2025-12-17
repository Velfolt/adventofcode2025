use std::{
    collections::HashMap,
    fs::read_to_string,
};

use aoc::AocDay;
use itertools::{Itertools, repeat_n};
use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::delimited,
};

pub struct Day10;

impl AocDay for Day10 {
    fn part1() {
        let input = read_to_string("input/day10.txt").unwrap();

        let a = input
            .lines()
            .map(|l| line(l).unwrap().1)
            .map(|(goal, buttons, _)| {
                combinations(&buttons, &goal)
                    .iter()
                    .min_by(|a, b| a.1.len().cmp(&b.1.len()))
                    .unwrap()
                    .1
                    .len()
            })
            .sum::<usize>();

        println!("{a}");
    }

    fn part2() {
        let input = read_to_string("input/day10.txt").unwrap();

        let mut cache = HashMap::new();

        let a = input
            .lines()
            .map(|l| line(l).unwrap().1)
            .map(|(_, buttons, goal)| find_min_joltage(&buttons, &goal, &mut cache))
            .sum::<usize>();

        println!("{a}");
    }
}

fn find_min_joltage(
    buttons: &Vec<Vec<usize>>,
    goal: &Vec<i64>,
    cache: &mut HashMap<(Vec<Vec<usize>>, Vec<i64>), usize>,
) -> usize {
    if let Some(len) = cache.get(&(buttons.clone(), goal.clone())) {
        return *len;
    }

    if goal.iter().all(|x| *x == 0) {
        return 0;
    }

    let odd = goal.iter().map(|x| x % 2 == 1).collect_vec();
    let combinations = combinations(buttons, &odd)
        .iter()
        .map(|(_, c)| {
            c.iter().fold((goal.clone(), c), |(acc, c), button| {
                let mut s = acc;
                for &i in button {
                    s[i] -= 1;
                }
                (s, c)
            })
        })
        .filter(|(s, _)| s.iter().all(|x| *x >= 0))
        .map(|(s, c)| (s.iter().map(|c| c / 2).collect_vec(), c))
        .map(|(s, c)| 2 * find_min_joltage(buttons, &s, cache) + c.len())
        .min();

    if let Some(min) = combinations {
        cache.insert((buttons.clone(), goal.clone()), min);

        return min;
    }

    cache.insert((buttons.clone(), goal.clone()), 10000000);

    return 10000000;
}

fn combinations(buttons: &Vec<Vec<usize>>, goal: &Vec<bool>) -> Vec<(Vec<bool>, Vec<Vec<usize>>)> {
    repeat_n([0_usize, 1usize], buttons.len())
        .multi_cartesian_product()
        .map(|x| {
            x.iter()
                .enumerate()
                .filter_map(|(i, &bi)| {
                    if bi > 0 {
                        Some(buttons[i].clone())
                    } else {
                        None
                    }
                })
                .fold(
                    (vec![false; goal.len()], vec![]),
                    |(acc, mut buttons), button| {
                        let mut solution = acc;

                        for &i in &button {
                            solution[i] = !solution[i];
                        }

                        buttons.push(button);

                        (solution, buttons)
                    },
                )
        })
        .filter(|(solution, _)| solution == goal)
        .collect_vec()
}

fn line(input: &str) -> IResult<&str, (Vec<bool>, Vec<Vec<usize>>, Vec<i64>)> {
    let (input, lights) = delimited(
        tag("["),
        many1(map_res(one_of(".#"), |s| -> Result<bool, bool> {
            Ok(match s {
                '#' => true,
                _ => false,
            })
        })),
        tag("]"),
    )
    .parse(input)?;

    let (input, _) = tag(" ").parse(input)?;

    let (input, buttons) = separated_list1(
        tag(" "),
        delimited(
            tag("("),
            separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<usize>())),
            tag(")"),
        ),
    )
    .parse(input)?;

    let (input, _) = tag(" ").parse(input)?;

    let (input, joltages) = delimited(
        tag("{"),
        separated_list1(tag(","), map_res(digit1, |s: &str| s.parse::<i64>())),
        tag("}"),
    )
    .parse(input)?;

    Ok((input, (lights, buttons, joltages)))
}
