fn main() {
    let input = include_str!("../inputs/day8.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {

    use std::collections::HashMap;

    pub fn process(input: &str) -> String {
        let (steps, graph_str) = input.trim().split_once("\n\n").unwrap();
        let steps: Vec<char> = steps.chars().collect();

        let mut graph = HashMap::<String, (String, String)>::new();

        graph_str.split('\n').for_each(|f| {
            let (node, edges) = f.split_once('=').unwrap();
            let (a, b) = edges
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();

            graph.insert(
                node.trim().to_string(),
                (a.to_string(), b.trim().to_string()),
            );
        });
        let mut idx = 0;
        let mut ans = 0;
        let mut cur = "AAA";
        loop {
            let edges = graph.get(cur).unwrap();
            if cur.eq("ZZZ") {
                return ans.to_string();
            }

            match steps[idx] {
                'R' => cur = &edges.1,
                'L' => cur = &edges.0,
                _ => {}
            }

            idx = (idx + 1) % steps.len();
            ans += 1;
        }
    }
}

mod part2 {
    use std::collections::HashMap;

    pub fn process(input: &str) -> String {
        let (steps, graph_str) = input.trim().split_once("\n\n").unwrap();
        let steps: Vec<char> = steps.chars().collect();

        let mut graph = HashMap::<String, (String, String)>::new();

        graph_str.split('\n').for_each(|f| {
            let (node, edges) = f.split_once('=').unwrap();
            let (a, b) = edges
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split_once(',')
                .unwrap();

            graph.insert(
                node.trim().to_string(),
                (a.to_string(), b.trim().to_string()),
            );
        });

        graph
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|s| solve(s, &steps, &graph))
            .fold(1u64, |acc, f| (acc * f) / gcd(acc, f))
            .to_string()
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if a == 0 {
            return b;
        }
        gcd(b % a, a)
    }

    fn solve(start: &str, steps: &Vec<char>, graph: &HashMap<String, (String, String)>) -> u64 {
        let mut idx = 0;
        let mut ans = 0;
        let mut cur = start;
        loop {
            let edges = graph.get(cur).unwrap();
            if cur.ends_with('Z') {
                return ans;
            }
            match steps[idx] {
                'R' => cur = &edges.1,
                'L' => cur = &edges.0,
                _ => {}
            }

            idx = (idx + 1) % steps.len();
            ans += 1;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = part1::process(input);
        assert_eq!(result, "2");

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1::process(input), "6");
    }

    #[test]
    fn part2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = part2::process(input);
        assert_eq!(result, "6")
    }
}
