fn main() {
    let input = include_str!("../inputs/day8.txt");
    dbg!(part1::process(input));
    // dbg!(part2::process(input));
}

mod part1 {

    use std::collections::HashMap;

    pub fn process(input: &str) -> String {
        let (steps, graph_str) = input.trim().split_once("\n\n").unwrap();
        let steps: Vec<char> = steps.chars().collect();

        let mut graph = HashMap::<String, (String, String)>::new();

        graph_str.split('\n').into_iter().for_each(|f| {
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
        let mut ans = 1;
        let mut cur = "AAA";
        loop {
            let edges = graph.get(cur).unwrap();

            match steps[idx] {
                'R' => cur = &edges.1,
                'L' => cur = &edges.0,
                _ => {}
            }
            if cur.eq("ZZZ") {
                return ans.to_string();
            }
            idx = (idx + 1) % steps.len();
            ans += 1;
        }
    }
}

mod part2 {
    use super::*;

    // pub fn process(input: &str) -> String {
    // }
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

    // #[test]
    // fn part2_test() {
    //     let input = "";
    //
    //     let result = part2::process(input);
    //     assert_eq!(result, "30")
    // }
}
