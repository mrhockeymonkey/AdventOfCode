use std::cmp::{max, min};
use nom::{sequence::{preceded, tuple, delimited}, character::complete::{digit1, char}, bytes::complete::{tag, take_while1}, error::{ErrorKind, ParseError, Error}, IResult, combinator::{map, map_res}, multi::separated_list0};
use nom::branch::alt;
use nom::multi::separated_list1;
use nom::sequence::pair;

fn main() {
    let sample = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let games = input.lines()
        .map(|line| dbg!(Game::from(line)))
        .collect::<Vec<_>>();

    // if only 12 red cubes, 13 green cubes, and 14 blue cubes in bag
    let possible_sum = games.iter()
        .filter(|g| !g.reveals.iter()
            .any(|reveal| reveal.r > 12 || reveal.g > 13 || reveal.b > 14))
        .inspect(|g| println!("game {} is possible", g.id))
        .map(|g| g.id)
        .sum::<u32>();

    println!("sum is {}", possible_sum);

    let power_sum = games.iter()
        .map(|game| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            game.reveals.iter().for_each(|re| {
                r = max(r,re.r);
                g = max(g,re.g);
                b = max(b,re.b);
            });
            println!("r{} g{} b{}", r, g, b);

            // return power
            dbg!(r*g*b)
        })
        .sum::<u32>();

    println!("sum of powers is {}", power_sum);


    ()
}


#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (reveals, id) = parse_game_id(value).unwrap();
        let reveals =  parse_reveals(reveals).unwrap();

        Game { id, reveals: reveals.1 }
    }
}

#[derive(Debug, PartialEq)]
struct Reveal {
    r: u32,
    g: u32,
    b: u32
}

impl Reveal {
    fn new(colors: Vec<ColouredSet>) -> Self {
        let mut r = 0;
        let mut b = 0;
        let mut g = 0;

        colors.iter().for_each(|c| match c {
            ColouredSet::Blue(n) => b += n,
            ColouredSet::Green(n) => g += n,
            ColouredSet::Red(n) => r += n,
        });

        Self{ r, g, b, }
    }
}

#[derive(Debug, PartialEq)]
enum ColouredSet {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    map_res(tuple((
        tag("Game "), 
        digit1::<&str, _>, 
        tag(": "))), |(_, id, _)| id.parse())(input)
}

fn parse_reveals(input: &str) -> IResult<&str, Vec<Reveal>> {
    separated_list1(tag("; "), parse_reveal)(input)
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    map(
        separated_list1(tag(", "), parse_coloured_set),
        |cols| Reveal::new(cols))(input)
}

fn parse_coloured_set(input: &str) -> IResult<&str, ColouredSet> {
    map(pair(
        digit1,
        alt((
            tag(" blue"), tag(" green"),tag(" red"))
        )
    ), |pair: (&str, &str)| match pair {
            (n, " blue") => ColouredSet::Blue(n.parse::<u32>().unwrap()),
            (n, " green") => ColouredSet::Green(n.parse::<u32>().unwrap()),
            (n, " red") => ColouredSet::Red(n.parse::<u32>().unwrap()),
            _ => panic!("should be no other colours!")
    })(input)
}

#[cfg(test)]
mod tests {
    use crate::{ColouredSet, parse_coloured_set, parse_reveal, parse_reveals, Reveal};

    #[test]
    fn can_parse_coloured_set() {
        let r = parse_coloured_set("3 blue").unwrap().1;
        assert_eq!(ColouredSet::Blue(3), dbg!(r));
    }

    #[test]
    fn can_parse_reveal() {
        let r = parse_reveal("3 blue, 4 green, 5 red").unwrap().1;
        assert_eq!(Reveal{r: 5, g: 4, b: 3}, dbg!(r));
    }

    #[test]
    fn can_parse_reveals() {
        let r = parse_reveals("3 blue, 4 green, 5 red; 1 blue, 2 red, 3 green").unwrap().1;
        assert_eq!(vec!(Reveal{r: 5, g: 4, b: 3}, Reveal{r: 2, g: 3, b: 1}) , dbg!(r));
    }
}

