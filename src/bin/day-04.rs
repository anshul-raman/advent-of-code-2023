fn main() {
    let input = include_str!("../inputs/day4.txt");
    dbg!(part1::process(input));
    // dbg!(part2::process(input));
}

mod part1 {
    use std::collections::HashSet;

    pub fn process(input: &str) -> String {
        input
            .trim()
            .split('\n')
            .map(process_line)
            .sum::<u64>()
            .to_string()
    }

    // input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    // outpu: "8"
    fn process_line(input: &str) -> u64 {
        let (_, rest) = input.split_once(':').unwrap();
        let (winnning, cur_nums) = rest.split_once('|').unwrap();

        let cur_nums: HashSet<u64> = cur_nums
            .split_whitespace()
            .map(|f| f.parse::<u64>().unwrap())
            .collect();

        let winning_nums = winnning
            .split_whitespace()
            .map(|f| f.parse::<u64>().unwrap())
            .filter(|f| cur_nums.contains(f))
            .count();
        match winning_nums {
            0 => 0,
            _ => u64::pow(2, winning_nums as u32 - 1),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1::process(input);
        assert_eq!(result, "13")
    }

    //     #[test]
    //     fn part2_test() {
    //         let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    //
    //         let result = part2::process(input);
    //         assert_eq!("2286", result);
    // }
}
