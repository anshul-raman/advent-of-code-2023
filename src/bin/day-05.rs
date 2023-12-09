fn main() {
    let input = include_str!("../inputs/day5.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {

    pub fn process(input: &str) -> String {
        let mut seeds: Vec<i64> = vec![];
        let mut lookup: Vec<Vec<(i64, i64, i64)>> = vec![];
        let mut ans = i64::MAX;
        for s in input.split('\n') {
            if s.starts_with("seeds") {
                let (_, seeds_str) = s.split_once(':').unwrap();
                seeds.extend(
                    seeds_str
                        .split_whitespace()
                        .map(|f| f.parse::<i64>().unwrap()),
                );
            } else if s.ends_with("map:") {
                lookup.push(Vec::new());
            } else if !s.is_empty() {
                let range_map: Vec<i64> = s
                    .split_whitespace()
                    .map(|f| f.parse::<i64>().unwrap())
                    .collect();
                let n = lookup.len() - 1;
                lookup[n].push((range_map[0], range_map[1], range_map[2]));
            }
        }

        for s in seeds {
            let mut seed_loc = s;
            for l in &lookup {
                for m in l {
                    if m.1 <= seed_loc && seed_loc <= m.1 + m.2 {
                        seed_loc = seed_loc - m.1 + m.0;
                        break;
                    }
                }
            }

            ans = ans.min(seed_loc)
        }

        ans.to_string()
    }
}

mod part2 {
    // use super::*;

    pub fn process(_input: &str) -> String {
        "todo".to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = part1::process(input);
        assert_eq!(result, "35")
    }

    //     #[test]
    //     fn part2_test() {
    //         let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    //
    //         let result = part2::process(input);
    //         assert_eq!(result, "30")
    // }
}
