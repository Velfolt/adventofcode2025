use std::fs::read_to_string;

use aoc::AocDay;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

pub struct Day12;

impl AocDay for Day12 {
    fn part1() {
        let input = read_to_string("input/day12.txt").unwrap();

        let a = parse(&input).unwrap().1;

        let num = a
            .iter()
            .filter(|((x, y), count)| *x * *y >= 9 * count.iter().sum::<usize>() as i64)
            .count();

        println!("{num}")
    }

    fn part2() {}
}

fn parse(input: &str) -> IResult<&str, Vec<((i64, i64), Vec<usize>)>> {
    let (input, _) =
        many1(separated_pair(digit1, tag(":\n"), many1(one_of("#.\n")))).parse(input)?;

    separated_list1(
        tag("\n"),
        separated_pair(
            separated_pair(
                map_res(digit1, str::parse),
                tag("x"),
                map_res(digit1, str::parse),
            ),
            tag(": "),
            separated_list1(tag(" "), map_res(digit1, str::parse)),
        ),
    )
    .parse(input)
}
