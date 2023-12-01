fn main() {
    let input = include_str!("../inputs/day1.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {
    pub fn process(input: &str) -> String {
        input
            .split('\n')
            .map(|line| {
                let mut sm = 0;

                // forward pass
                for c in line.chars() {
                    if let Some(d) = c.to_digit(10) {
                        sm = d;
                        break;
                    }
                }

                // backward pass
                for c in line.chars().rev() {
                    if let Some(d) = c.to_digit(10) {
                        sm = sm * 10 + d;
                        break;
                    }
                }
                sm
            })
            .sum::<u32>()
            .to_string()
    }
}

mod part2 {

    pub fn process(input: &str) -> String {
        input.split('\n').map(process_line).sum::<u32>().to_string()
    }

    fn process_line(line: &str) -> u32 {
        let mut sm = 0;

        // forward pass
        for (idx, c) in line.chars().enumerate() {
            if line[idx..].starts_with("one") {
                sm = 1;
                break;
            } else if line[idx..].starts_with("two") {
                sm = 2;
                break;
            } else if line[idx..].starts_with("three") {
                sm = 3;
                break;
            } else if line[idx..].starts_with("four") {
                sm = 4;
                break;
            } else if line[idx..].starts_with("five") {
                sm = 5;
                break;
            } else if line[idx..].starts_with("six") {
                sm = 6;
                break;
            } else if line[idx..].starts_with("seven") {
                sm = 7;
                break;
            } else if line[idx..].starts_with("eight") {
                sm = 8;
                break;
            } else if line[idx..].starts_with("nine") {
                sm = 9;
                break;
            } else if let Some(d) = c.to_digit(10) {
                sm = d;
                break;
            }
        }

        // backward pass
        for (idx, c) in line.chars().rev().enumerate() {
            let idx = line.len() - idx - 1;
            if line[idx..].starts_with("one") {
                sm = sm * 10 + 1;
                break;
            } else if line[idx..].starts_with("two") {
                sm = sm * 10 + 2;
                break;
            } else if line[idx..].starts_with("three") {
                sm = sm * 10 + 3;
                break;
            } else if line[idx..].starts_with("four") {
                sm = sm * 10 + 4;
                break;
            } else if line[idx..].starts_with("five") {
                sm = sm * 10 + 5;
                break;
            } else if line[idx..].starts_with("six") {
                sm = sm * 10 + 6;
                break;
            } else if line[idx..].starts_with("seven") {
                sm = sm * 10 + 7;
                break;
            } else if line[idx..].starts_with("eight") {
                sm = sm * 10 + 8;
                break;
            } else if line[idx..].starts_with("nine") {
                sm = sm * 10 + 9;
                break;
            } else if let Some(d) = c.to_digit(10) {
                sm = sm * 10 + d;
                break;
            }
        }
        sm
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1::process(input);
        assert_eq!(result, "142")
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = part2::process(input);
        assert_eq!("281", result);
    }
}
