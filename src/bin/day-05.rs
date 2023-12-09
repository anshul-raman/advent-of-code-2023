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

    use std::collections::HashMap;

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

        for l in lookup.iter_mut() {
            l.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        }

        for s_range in seeds.chunks_exact(2) {
            for s in s_range[0]..s_range[0] + s_range[1] {
                let mut seed_loc = s;
                // dbg!(seed_loc);
                for  l in lookup.iter() {
                    // check memo
                    // compute if not found in memo
                    let mut mapped_val = seed_loc;
                    // dbg!("mapping from:", idx, &l);

                    // for m in l {
                    //     if m.1 <= seed_loc && seed_loc <= m.1 + m.2 {
                    //         mapped_val = seed_loc - m.1 + m.0;
                    //         break;
                    //     }
                    // }
                    //
                    //

                    // using binary search simmilar to loop above

                    let mut lo: i64 = 0;
                    let mut hi: i64 = l.len() as i64 - 1;
                    while lo <= hi {
                        let mid = (lo + hi) / 2;
                        let rng = l[mid as usize];
                        // dbg!("checking at", mid, seed_loc, rng);

                        if rng.1 <= seed_loc && seed_loc <= rng.1 + rng.2 - 1 {
                            mapped_val = seed_loc - rng.1 + rng.0;
                            break;
                        }

                        if seed_loc < rng.1 {
                            hi = mid - 1;
                        } else {
                            lo = mid + 1;
                        }
                        // dbg!(found, mapped_val, seed_loc, idx);
                    }
                    seed_loc = mapped_val;
                }

                ans = ans.min(seed_loc);
            }
        }

        ans.to_string()
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
