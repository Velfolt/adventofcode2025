use aoc::utils::Directions;
use aoc::utils::IndexToPos;
use aoc::utils::PosToIndex;
use aoc::utils::PosWithinBounds;
use itertools::Itertools;
use std::fs::read_to_string;

use aoc::AocDay;

pub struct Day4;
impl AocDay for Day4 {
    fn part1() {
        let file_input = read_to_string("input/day4.txt").unwrap();

        let width = file_input.find("\n").unwrap();

        let grid = file_input
            .trim()
            .replace("\n", "")
            .chars()
            .enumerate()
            .collect_vec();

        let rolls = rolls_to_remove(&width, &grid).count();

        println!("{rolls}")
    }

    fn part2() {
        let file_input = read_to_string("input/day4.txt").unwrap();

        let width = file_input.find("\n").unwrap();

        let mut grid = file_input
            .trim()
            .replace("\n", "")
            .chars()
            .enumerate()
            .collect_vec();

        let mut removed_rolls = 0;

        loop {
            let rolls = rolls_to_remove(&width, &grid).collect_vec();

            if rolls.len() == 0 {
                break;
            }

            removed_rolls += rolls.len();

            rolls.iter().for_each(|(i, _)| grid[*i].1 = 'x');
        }

        println!("{removed_rolls}");
    }
}

fn rolls_to_remove<'a>(
    width: &'a usize,
    grid: &'a Vec<(usize, char)>,
) -> impl Iterator<Item = (usize, char)> {
    grid.iter()
        .cloned()
        .filter(|(_, c)| *c == '@')
        .filter(|(i, c)| {
            i.to_pos(*width)
                .all_directions()
                .iter()
                .filter(|pos| {
                    pos.to_index(*width) < grid.len()
                        && pos.within_bounds(*width)
                        && grid[pos.to_index(*width)].1 == '@'
                })
                .count()
                < 4
        })
}
