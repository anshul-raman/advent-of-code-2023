#[derive(Default, Debug, PartialEq, Eq)]
struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    game_id: u64,
    bag: Bag,
}

enum Cube {
    RED(u64),
    GREEN(u64),
    BLUE(u64),
}

fn main() {
    let input = include_str!("../inputs/day2.txt");
    dbg!(part1::process(input));
    dbg!(part2::process(input));
}

/*
*
* input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
* output = Game {
*               bag: Bag {
*                   red: 4,
*                   green: 2,
*                   blue: 6
*               },
*               game_id: 1
*           }
*
* */
fn parse_line(input: &str) -> Game {
    let (game_id, game_play) = input.split_once(':').expect("should be present");
    let game_id = parse_game_id(game_id);
    let bag = game_play
        .split(';')
        .map(parse_game_play)
        .fold(Bag::default(), |acc, x| Bag {
            red: acc.red.max(x.red),
            green: acc.green.max(x.green),
            blue: acc.blue.max(x.blue),
        });

    Game { game_id, bag }
}

/*
*
* input = "1 red, 2 green"
* output = Bag {red = 1, green = 2}
*
* */
fn parse_game_play(input: &str) -> Bag {
    input
        .split(',')
        .map(parse_cube)
        .fold(Bag::default(), |mut acc, c| {
            match c {
                Cube::RED(x) => acc.red = x,
                Cube::GREEN(x) => acc.green = x,
                Cube::BLUE(x) => acc.blue = x,
            };
            acc
        })
}

/*
 *
 * input = "1 red"
 * output = Cube::RED(1)
 *
 * */
fn parse_cube(input: &str) -> Cube {
    let (num, ctype) = input.trim().split_once(' ').expect("should be present");
    let num = num.parse::<u64>().expect("should be a number");
    match ctype {
        "blue" => Cube::BLUE(num),
        "red" => Cube::RED(num),
        "green" => Cube::GREEN(num),
        _ => panic!("Invlid input"),
    }
}

/*
 *
 * input = "Game 1"
 * output = 1
 *
 * */
fn parse_game_id(input: &str) -> u64 {
    let (_, id) = input.split_once(' ').expect("should be present");
    id.parse().expect("should be number")
}

mod part1 {
    use super::*;

    pub fn process(input: &str) -> String {
        input
            .trim()
            .split('\n')
            .map(parse_line)
            .filter(|g| g.bag.red <= 12 && g.bag.green <= 13 && g.bag.blue <= 14)
            .map(|g| g.game_id)
            .sum::<u64>()
            .to_string()
    }
}

mod part2 {

    use super::*;

    pub fn process(input: &str) -> String {
        input
            .trim()
            .split('\n')
            .map(parse_line)
            .map(|g| g.bag.red * g.bag.green * g.bag.blue)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{part1, part2};

    #[test]
    fn test_parse_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let res = parse_line(input);
        assert_eq!(
            res,
            Game {
                bag: Bag {
                    red: 4,
                    green: 2,
                    blue: 6
                },
                game_id: 1
            }
        )
    }

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1::process(input);
        assert_eq!(result, "8")
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part2::process(input);
        assert_eq!("2286", result);
    }
}
