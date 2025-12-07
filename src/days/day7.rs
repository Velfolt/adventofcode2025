use std::{collections::{HashMap, VecDeque}, fs::read_to_string};

use aoc::{
    AocDay,
    utils::{IndexToPos, Point, PosToIndex, PosWithinBounds, PrintGrid},
};
use itertools::Itertools;

pub struct Day7;

static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

impl AocDay for Day7 {
    fn part1() {
        let input = read_to_string("input/day7.txt").unwrap();

        let width = input.trim().lines().nth(0).unwrap().len();
        let gridstr = input.trim().lines().join("");
        let start = gridstr.find(|c| c == 'S').unwrap();

        let mut beams = vec![];

        let mut active_pos = VecDeque::from([start.to_pos(width)]);

        let mut tachyon_beams_split = vec![];

        loop {
            if let Some(pos) = active_pos.pop_front() {
                if !pos.within_bounds(width) || pos.to_index(width) >= gridstr.len() {
                    continue;
                }

                let next_pos = Point(pos) + Point((0, 1));

                match gridstr.as_bytes()[next_pos.to_index(width)] as char {
                    '^' => {
                        tachyon_beams_split.push(next_pos);
                        let pos_left = Point(next_pos) + Point((-1, 0));
                        let pos_right = Point(next_pos) + Point((1, 0));
                        if !beams.contains(&pos_left) {
                            beams.push(pos_left);
                            active_pos.push_back(pos_left);
                        }

                        if !beams.contains(&pos_right) {
                            beams.push(pos_right);
                            active_pos.push_back(pos_right);
                        }
                    }
                    _ => {
                        beams.push(next_pos);
                        active_pos.push_back(next_pos)
                    }
                }
            } else {
                break;
            }
        }

        tachyon_beams_split.sort();
        let sum = tachyon_beams_split.iter().unique().count();

        println!("tachyon beams split {sum}");
    }

    fn part2() {
        let input = read_to_string("input/day7.txt").unwrap();

        let width = input.trim().lines().nth(0).unwrap().len();
        let gridstr = input.trim().lines().join("");
        let start = gridstr.find(|c| c == 'S').unwrap();

	let mut cache = HashMap::new();

        let total = particle_step(start.to_pos(width), 1, width, &gridstr, &mut cache);

	println!("{total}");
    }
}

fn particle_step(pos: (i64, i64), timelines: i64, width: usize, grid: &str, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(timeline) = cache.get(&pos) {
	return *timeline;
    }

    if !pos.within_bounds(width) {
        return 1;
    }

    let next_pos = Point(pos) + Point((0, 1));

    match grid.as_bytes()[next_pos.to_index(width)] as char {
        '^' => {
            let pos_left = Point(next_pos) + Point((-1, 0));
            let pos_right = Point(next_pos) + Point((1, 0));

	    let l = particle_step(pos_left, timelines, width, grid, cache);
	    let r = particle_step(pos_right, timelines, width, grid, cache);

	    cache.insert(pos_left, l);
	    cache.insert(pos_right, r);

	    return l + r;
        }
        _ => {
	    return particle_step(next_pos, timelines, width, grid, cache);
	}
    }
}
