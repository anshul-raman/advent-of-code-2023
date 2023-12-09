fn main() {
    let input = include_str!("../inputs/day6.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

mod part1 {

    pub fn process(input: &str) -> String {
        let (time_str, dist_str) = input.split_once('\n').unwrap();

        let (_, time) = time_str.split_once(':').unwrap();
        let time: Vec<u64> = time
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        let (_, dist) = dist_str.split_once(':').unwrap();
        let dist: Vec<u64> = dist
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        time.iter()
            .zip(dist.iter())
            .map(|(t, dis)| {
                let d = t * t - 4 * dis;
                let d = f64::sqrt(d as f64);
                let is_perf_sqr = d.ceil() == d.floor();

                let a = (*t as f64 - d) / 2f64;
                let a = a.ceil() as u64;

                let b = (*t as f64 + d) / 2f64;
                let b = b.floor() as u64;

                let mut ans = b - a + 1;
                if is_perf_sqr {
                    ans -= 2;
                }
                dbg!(t, dis, a, b, ans);
                ans
            })
            .product::<u64>()
            .to_string()
    }
}

mod part2 {

    pub fn process(input: &str) -> String {
        let (time_str, dist_str) = input.split_once('\n').unwrap();
        let (_, time) = time_str.split_once(':').unwrap();
        let (_, dist) = dist_str.split_once(':').unwrap();
        let time = time
            .split_whitespace()
            .fold(String::new(), |mut acc, f| {
                acc.push_str(f);
                acc
            })
            .parse::<u64>()
            .unwrap();

        let dist = dist
            .split_whitespace()
            .fold(String::new(), |mut acc, f| {
                acc.push_str(f);
                acc
            })
            .parse::<u64>()
            .unwrap();

        let d = time * time - 4 * dist;
        let d = f64::sqrt(d as f64);
        let is_perf_sqr = d.ceil() == d.floor();

        let a = (time as f64 - d) / 2f64;
        let a = a.ceil() as u64;

        let b = (time as f64 + d) / 2f64;
        let b = b.floor() as u64;

        let mut ans = b - a + 1;
        if is_perf_sqr {
            ans -= 2;
        }
        ans.to_string()
    }
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

    #[test]
    fn part2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let result = part2::process(input);
        assert_eq!(result, "71503")
    }
}
