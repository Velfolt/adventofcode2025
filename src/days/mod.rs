pub mod day1;
pub mod day2;

pub trait AocDay {
    fn part1();
    fn part2();

    fn run() {
	Self::part1();
	Self::part2();
    }
}
