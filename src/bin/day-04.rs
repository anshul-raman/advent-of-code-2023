use std::{collections::HashSet, usize};

fn main() {
    let input = include_str!("../inputs/day4.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

// input: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
// output: "4"
fn process_line(input: &str) -> usize {
    let (_, rest) = input.split_once(':').unwrap();
    let (winnning, cur_nums) = rest.split_once('|').unwrap();

    let cur_nums: HashSet<u64> = cur_nums
        .split_whitespace()
        .map(|f| f.parse::<u64>().unwrap())
        .collect();

    winnning
        .split_whitespace()
        .map(|f| f.parse::<u64>().unwrap())
        .filter(|f| cur_nums.contains(f))
        .count()
}

mod part1 {

    use super::*;

    pub fn process(input: &str) -> String {
        input
            .trim()
            .split('\n')
            .map(process_line)
            .map(|c| match c {
                0 => 0,
                _ => u64::pow(2, c as u32 - 1),
            })
            .sum::<u64>()
            .to_string()
    }
}

mod part2 {
    use super::*;

    pub fn process(input: &str) -> String {
        let num_cards = input.trim().split('\n').count();
        let mut cards: Vec<i64> = vec![1; num_cards];
        input
            .trim()
            .split('\n')
            .map(process_line)
            .enumerate()
            .for_each(|(idx, count)| {
                let cur = cards[idx];
                for i in cards.iter_mut().take(idx + count + 1).skip(idx + 1) {
                    *i += cur;
                }
            });

        cards.iter().sum::<i64>().to_string()
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

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part2::process(input);
        assert_eq!(result, "30")
    }
}
