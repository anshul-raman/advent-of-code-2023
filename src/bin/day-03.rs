fn main() {
    let input = include_str!("../inputs/day3.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

#[derive(Debug)]
struct Number {
    value: u64,
    line_idx: usize,
    start_idx: usize,
    end_idx: usize,
}

fn parse_numbers(line_idx: usize, input: &str) -> Vec<Number> {
    let mut nums = Vec::new();
    let mut cur_num = String::new();
    for (idx, c) in input.chars().enumerate() {
        match c {
            '0'..='9' => {
                cur_num.push(c);
                if idx + 1 == input.len() {
                    nums.push(Number {
                        line_idx,
                        value: cur_num.parse().unwrap(),
                        start_idx: idx - cur_num.len() + 1,
                        end_idx: idx,
                    });
                    cur_num.clear();
                }
            }
            _ => {
                // append to nums if string is non empty;
                if !cur_num.is_empty() {
                    nums.push(Number {
                        line_idx,
                        value: cur_num.parse().unwrap(),
                        start_idx: idx - cur_num.len(),
                        end_idx: idx - 1,
                    });
                    cur_num.clear();
                }
            }
        }
    }

    nums
}

mod part1 {
    use super::*;

    pub fn process(input: &str) -> String {
        let lookup: Vec<Vec<usize>> = input.split('\n').map(parse_symbols).collect();

        input
            .split('\n')
            .enumerate()
            .flat_map(|(idx, inp)| parse_numbers(idx, inp))
            .filter(|n| {
                let mut is_valid = false;

                // line_idx - 1 row
                if n.line_idx > 0 {
                    for &s in lookup[n.line_idx - 1].iter() {
                        if n.start_idx <= s + 1 && s <= n.end_idx + 1 {
                            is_valid = true
                        }
                    }
                }

                // line_idx row
                for &s in lookup[n.line_idx].iter() {
                    if s + 1 == n.start_idx || s == n.end_idx + 1 {
                        is_valid = true
                    }
                }

                // line_idx + 1 row
                if n.line_idx + 1 < lookup.len() {
                    for &s in lookup[n.line_idx + 1].iter() {
                        if n.start_idx <= s + 1 && s <= n.end_idx + 1 {
                            is_valid = true
                        }
                    }
                }

                is_valid
            })
            .map(|f| f.value)
            .sum::<u64>()
            .to_string()
    }

    fn parse_symbols(input: &str) -> Vec<usize> {
        input
            .chars()
            .enumerate()
            .filter(|(_, c)| !matches!(c, '.' | '0'..='9'))
            .map(|(idx, _)| idx)
            .collect()
    }
}

mod part2 {
    use super::*;

    pub fn process(input: &str) -> String {
        let numbers: Vec<Vec<Number>> = input
            .split('\n')
            .enumerate()
            .map(|(idx, inp)| parse_numbers(idx, inp))
            .collect();

        input
            .split('\n')
            .map(parse_gears)
            .enumerate()
            .flat_map(|(line_idx, f)| {
                f.into_iter()
                    .map(|g| {
                        // select only gears
                        let mut num_count = 0;
                        let mut gr = 1;

                        if line_idx > 0 {
                            for n in numbers[line_idx - 1].iter() {
                                if g - 1 == n.end_idx
                                    || g + 1 == n.start_idx
                                    || (n.start_idx <= g && g <= n.end_idx)
                                {
                                    num_count += 1;
                                    gr *= n.value
                                }
                            }
                        }

                        // line_idx row
                        for n in numbers[line_idx].iter() {
                            if g - 1 == n.end_idx || g + 1 == n.start_idx {
                                num_count += 1;
                                gr *= n.value
                            }
                        }

                        // line_dx + 1 row
                        if line_idx + 1 < numbers.len() {
                            for n in numbers[line_idx + 1].iter() {
                                if g - 1 == n.end_idx
                                    || g + 1 == n.start_idx
                                    || (n.start_idx <= g && g <= n.end_idx)
                                {
                                    num_count += 1;
                                    gr *= n.value
                                }
                            }
                        }

                        if num_count == 2 {
                            gr
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<u64>>()
            })
            .sum::<u64>()
            .to_string()
    }

    fn parse_gears(input: &str) -> Vec<usize> {
        input
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '*')
            .map(|(idx, _)| idx)
            .collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1::process(input), "4361");
        assert_eq!(part1::process(""), "0");
        assert_eq!(part1::process("..4.."), "0");

        let inp2 = "........
.........
220*.440
.......*.
.........";
        assert_eq!(part1::process(inp2), "660");
    }

    #[test]
    fn part2_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part2::process(input), "467835");
    }
}
