fn main() {
    let input = include_str!("../inputs/day9.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {

    pub fn process(input: &str) -> String {
        input
            .trim()
            .split('\n')
            .map(|inp| {
                let inp: Vec<i64> = inp.trim().split(' ').map(|i| i.parse().unwrap()).collect();
                let mut matrix: Vec<Vec<i64>> = Vec::new();
                matrix.push(inp);

                loop {
                    let mut n_arr: Vec<i64> = vec![];
                    let last_arr = matrix.last().unwrap();

                    let mut is_all_zero = true;
                    for i in 1..last_arr.len() {
                        let elem = last_arr[i] - last_arr[i - 1];
                        if elem != 0 {
                            is_all_zero = false;
                        }
                        n_arr.push(elem);
                    }

                    matrix.push(n_arr);
                    if is_all_zero {
                        break;
                    }
                }

                matrix.reverse();

                let mut ans = *matrix.first().unwrap().last().unwrap();
                for i in 1..matrix.len() {
                    let cur = matrix[i].last().unwrap();
                    ans += cur
                }
                ans
            })
            .sum::<i64>()
            .to_string()
    }
}

mod part2 {

    pub fn process(input: &str) -> String {
       input
            .trim()
            .split('\n')
            .map(|inp| {
                let inp: Vec<i64> = inp.trim().split(' ').map(|i| i.parse().unwrap()).collect();
                let mut matrix: Vec<Vec<i64>> = Vec::new();
                matrix.push(inp);

                loop {
                    let mut n_arr: Vec<i64> = vec![];
                    let last_arr = matrix.last().unwrap();

                    let mut is_all_zero = true;
                    for i in 1..last_arr.len() {
                        let elem = last_arr[i] - last_arr[i - 1];
                        if elem != 0 {
                            is_all_zero = false;
                        }
                        n_arr.push(elem);
                    }

                    matrix.push(n_arr);
                    if is_all_zero {
                        break;
                    }
                }

                matrix.reverse();

                let mut ans = *matrix.first().unwrap().first().unwrap();
                for i in 1..matrix.len() {
                    let cur = matrix[i].first().unwrap();
                    ans = cur - ans;
                }
                ans
            })
            .sum::<i64>()
            .to_string()
    } 
    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part1::process(input);
        assert_eq!(result, "114");

        let input = "0 1 1 0";
        assert_eq!(part1::process(input), "-2");
    }

    #[test]
    fn part2_test() {

        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = part2::process(input);
        assert_eq!(result, "2")
    }
}
