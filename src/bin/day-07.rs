use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
}
fn main() {
    let input = include_str!("../inputs/day7.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {
    use super::*;

    pub fn process(input: &str) -> String {
        let card_map = HashMap::from([
            ('2', 0),
            ('3', 1),
            ('4', 2),
            ('5', 3),
            ('6', 4),
            ('7', 5),
            ('8', 6),
            ('9', 7),
            ('T', 8),
            ('J', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]);

        let mut input = input
            .trim()
            .split('\n')
            .map(|f| {
                let (cards, bid) = f.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                let cards = cards.to_string();
                Hand { cards, bid }
            })
            .collect::<Vec<Hand>>();


        input.sort_by(|a, b| {
            let mut mp_a = HashMap::new();
            a.cards.chars().for_each(|c| {
                mp_a.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });

            let mut mp_b = HashMap::new();
            b.cards.chars().for_each(|c| {
                mp_b.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });


            if mp_a.len() < mp_b.len() {
                return Ordering::Greater;
            }

            if mp_a.len() > mp_b.len() {
                return Ordering::Less;
            }

            let mx_len_a = mp_a
                .iter()
                .map(|(_, l)| l)
                .reduce(|acc, l| acc.max(l))
                .unwrap();
            let mx_len_b = mp_b
                .iter()
                .map(|(_, l)| l)
                .reduce(|acc, l| acc.max(l))
                .unwrap();

            if mx_len_a < mx_len_b {
                return Ordering::Less;
            }

            if mx_len_a > mx_len_b {
                return Ordering::Greater;
            }

            for (x, y) in a.cards.chars().zip(b.cards.chars()) {
                dbg!(x, y);
                let rank_a = card_map.get(&x).unwrap();
                let rank_b = card_map.get(&y).unwrap();

                if rank_a < rank_b {
                    return Ordering::Less;
                }
                if rank_a > rank_b {
                    return Ordering::Greater;
                }
            }
            unreachable!();

        });

        input
            .iter()
            .enumerate()
            .map(|(idx, h)| (idx + 1) * h.bid as usize)
            .sum::<usize>()
            .to_string()
    }
}

mod part2 {

    use super::*;

    pub fn process(input: &str) -> String {
        let card_map = HashMap::from([
            ('J', 0),
            ('2', 1),
            ('3', 2),
            ('4', 3),
            ('5', 4),
            ('6', 5),
            ('7', 6),
            ('8', 7),
            ('9', 8),
            ('T', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]);

        let mut input = input
            .trim()
            .split('\n')
            .map(|f| {
                let (cards, bid) = f.split_once(' ').unwrap();
                let bid = bid.parse::<u64>().unwrap();
                let cards = cards.to_string();
                Hand { cards, bid }
            })
            .collect::<Vec<Hand>>();


        input.sort_by(|a, b| {
            let mut mp_a = HashMap::new();
            let mut j_count_a = 0;
            a.cards.chars().for_each(|c| {
                if c == 'J' {
                    j_count_a += 1;
                } else {
                    mp_a.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
            });
            if j_count_a == 5 {
                mp_a.insert('A', 5);
            } else {
                let mut mp_a_arr = mp_a.clone().into_iter().collect::<Vec<(char, i32)>>();
                mp_a_arr.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                let (c, _) = mp_a_arr[0];
                mp_a.entry(c).and_modify(|e| *e += j_count_a);
            }



            let mut mp_b = HashMap::new();
            let mut j_count_b = 0;
            b.cards.chars().for_each(|c| {
                if c == 'J' {
                    j_count_b += 1;
                } else {
                    mp_b.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
            });
            if j_count_b == 5 {
                mp_b.insert('A', 5); 
            }else {
                let mut mp_b_arr = mp_b.clone().into_iter().collect::<Vec<(char, i32)>>();
                mp_b_arr.sort_by(|a,b| a.1.cmp(&b.1).reverse()); 
                let (c, _) = mp_b_arr[0]; 
                mp_b.entry(c).and_modify(|e| *e += j_count_b);
            }



            if mp_a.len() < mp_b.len() {
                return Ordering::Greater;
            }

            if mp_a.len() > mp_b.len() {
                return Ordering::Less;
            }

            let mx_len_a = mp_a
                .iter()
                .map(|(_, l)| l)
                .reduce(|acc, l| acc.max(l))
                .unwrap();
            let mx_len_b = mp_b
                .iter()
                .map(|(_, l)| l)
                .reduce(|acc, l| acc.max(l))
                .unwrap();

            if mx_len_a < mx_len_b {
                return Ordering::Less;
            }

            if mx_len_a > mx_len_b {
                return Ordering::Greater;
            }

            for (x, y) in a.cards.chars().zip(b.cards.chars()) {
                let rank_a = card_map.get(&x).unwrap();
                let rank_b = card_map.get(&y).unwrap();

                if rank_a < rank_b {
                    return Ordering::Less;
                }
                if rank_a > rank_b {
                    return Ordering::Greater;
                }
            }
            unreachable!();

        });

        input
            .iter()
            .enumerate()
            .map(|(idx, h)| (idx + 1) * h.bid as usize)
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part1::process(input);
        assert_eq!(result, "6440");

        let input = "
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

        assert_eq!(part1::process(input), "6592");

        let input = "627Q8 1
A26Q7 1
2K637 1";
        assert_eq!(part1::process(input), "6");

        let input = "AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43";
        assert_eq!(part1::process(input), "1343");

        let input = "23456 22
56789 19
KJJKK 2
AAAAJ 3
JJ243 7
QJ256 6
QQ562 5
Q8Q24 4
AAAAT 3
TJJJJ 2
6789T 18
789TJ 17
22345 13
34567 21
45678 20
32245 12
33245 11
89TJQ 16
9TJQK 15
TJQKA 14
3J245 10
J3425 9
J5432 8
JJJJJ 1";
        assert_eq!(part1::process(input), "2237");

        let input = "23456 1
AAAKK 3
AAAAA 1";
        assert_eq!(part1::process(input), "10");

        let input = "KJ3A2 17
JQ472 72
QT3J2 19
23456 23";
        assert_eq!(part1::process(input), "292");
    }

    #[test]
    fn part2_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part2::process(input);
        assert_eq!(result, "5905")
    }
}
