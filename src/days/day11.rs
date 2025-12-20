use std::{collections::HashMap, fs::read_to_string};

use aoc::AocDay;
use nom::{
    IResult, Parser,
    bytes::tag,
    character::complete::{alpha1, line_ending},
    combinator::opt,
    multi::{fold_many1, separated_list0},
    sequence::separated_pair,
};

pub struct Day11;

impl AocDay for Day11 {
    fn part1() {
        let input = read_to_string("input/day11.txt").unwrap();

        let paths = parse(&input).unwrap().1;

        let n = travel("you", &paths);

        println!("{n:?}")
    }

    fn part2() {
        let input = read_to_string("input/day11.txt").unwrap();

        let paths = parse(&input).unwrap().1;

        let mut cache = HashMap::new();

        let n = travel_with_flag("svr", false, false, &paths, &mut cache);

        println!("{n:?}")
    }
}

fn travel(node: &str, graph: &HashMap<&str, Vec<&str>>) -> usize {
    if node == "out" {
        return 1;
    }

    let paths = graph[node].iter().map(|&path| travel(path, graph)).sum();

    paths
}

fn travel_with_flag<'a>(
    node: &'a str,
    dac: bool,
    fft: bool,
    graph: &'a HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    if let Some(paths) = cache.get(&(node, dac, fft)) {
        return *paths;
    }

    if node == "out" {
        if dac && fft {
            return 1;
        } else {
            return 0;
        }
    }

    let paths = graph[node]
        .iter()
        .map(|&path| {
            travel_with_flag(
                path,
                dac || node == "dac",
                fft || node == "fft",
                graph,
                cache,
            )
        })
        .sum();

    cache.insert((node, dac, fft), paths);

    paths
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    fold_many1(
        (
            separated_pair(alpha1, tag(": "), separated_list0(tag(" "), alpha1)),
            opt(line_ending),
        ),
        || HashMap::new(),
        |mut acc, ((key, item), _)| {
            acc.insert(key, item);

            acc
        },
    )
    .parse_complete(input)
}
