use std::fs::read_to_string;

pub fn day1_part1() {
    let file_input = read_to_string("input/day1.txt").unwrap();

    let turns = file_input.trim().split("\n").map(|x| x.split_at(1));

    let mut dial = 50;

    let mut count = 0;

    for (c, val) in turns {
        let turn: i64 = val.parse().unwrap();
        match c {
            "L" => dial -= turn,
            _ => dial += turn,
        }
        dial = dial.strict_rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    println!("{:?}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundaries() {
        assert_eq!(click("L50"), 1);
        assert_eq!(
            click(
                "L49
L1"
            ),
            1
        );
        assert_eq!(
            click(
                "L51
"
            ),
            1
        );
        assert_eq!(click("L150"), 2);
        assert_eq!(click("L151"), 2);
        assert_eq!(click("L250"), 3);
        assert_eq!(click("L251"), 3);

        assert_eq!(click("R50"), 1);
        assert_eq!(
            click(
                "R49
R1"
            ),
            1
        );
        assert_eq!(
            click(
                "R51
"
            ),
            1
        );
        assert_eq!(click("R150"), 2);
        assert_eq!(click("R151"), 2);
        assert_eq!(click("R250"), 3);
        assert_eq!(click("R251"), 3);
    }
}

fn click(input: &str) -> i64 {
    let turns = input.trim().split("\n").map(|x| x.split_at(1));

    let mut dial = 50;
    let mut count = 0;

    for (c, val) in turns {
        let turn: i64 = val.parse().unwrap();

        let start_dial = dial;

        match c {
            "L" => dial -= turn,
            _ => dial += turn,
        }

        if start_dial == 0 && dial < 0 {
            count += dial / -100;

            if dial.rem_euclid(100) == 0 {
                count -= 1;
            }
        } else if start_dial != 0 && dial < 0 {
            count += dial / -100 + 1;

            if dial.rem_euclid(100) == 0 {
                count -= 1;
            }
        } else if dial >= 100 {
            count += dial / 100;

            if dial.rem_euclid(100) == 0 {
                count -= 1;
            }
        }

        dial = dial.strict_rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    count
}

pub fn day1_part2() {
    let file_input = read_to_string("input/day1.txt").unwrap();
    let count = click(&file_input);

    println!("{:?}", count);
}
