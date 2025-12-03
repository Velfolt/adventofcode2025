use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;

pub trait AocDay {
    fn part1();
    fn part2();

    fn run() {
	let start = Instant::now();
	Self::part1();
	println!("Part1: {:?}\n", Instant::now().duration_since(start));

	let start = Instant::now();
	Self::part2();
	println!("Part2: {:?}", Instant::now().duration_since(start));
    }
}
