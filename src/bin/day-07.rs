fn main() {
    let input = include_str!("../inputs/day7.txt");
    dbg!(part1::process(input));
    // dbg!(part2::process(input));
}

mod part1 {

    struct Hand {
        cards: String,
        bid: u64,
    }

    pub fn process(input: &str) -> String {
        input
            .split('\n')
            .map(|f| {
                let (cards, bid) = f.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                let cards = cards.to_string();
                Hand { cards, bid }
            })
            .collect::<Vec<Hand>>()
            .sort_by(|a, b| todo!());

        todo!()
    }
}

mod part2 {

    // pub fn process(input: &str) -> String {
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = part1::process(input);
        assert_eq!(result, "288")
    }

    //     #[test]
    //     fn part2_test() {
    //         let input = "Time:      7  15   30
    // Distance:  9  40  200";
    //
    //         let result = part2::process(input);
    //         assert_eq!(result, "71503")
    //     }
}
