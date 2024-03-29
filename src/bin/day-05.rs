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

    pub fn process(input: &str) -> String {
        let mut seeds: Vec<i64> = vec![];
        let mut lookup: Vec<Vec<(i64, i64, i64)>> = vec![];
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
        lookup.reverse();
        let mut loc = 1;
        loop {
            let mut ans = loc;
            for  l in &lookup {
                for m in l {
                    if m.0 <= ans && ans <= m.0 + m.2 {
                        ans = ans - m.0 + m.1;
                        break;
                    }
                }
            }

            for s in seeds.chunks_exact(2) {
                if s[0] <= ans && ans <= s[0] +  s[1] {
                    return loc.to_string(); 
                }
            }

            loc += 1;
        }

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

    #[test]
    fn part2_test() {
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
        let result = part2::process(input);
        assert_eq!(result, "46")
    }
}
