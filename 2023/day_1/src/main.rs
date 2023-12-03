use nom::{IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1, anychar, char, digit1};
use nom::character::is_digit;
use nom::combinator::{map};
use nom::multi::many0;
use regex::Regex;

fn main() {
    let sample = include_str!("sample.txt");
    let sample2 = include_str!("sample2.txt");
    let input = include_str!("input.txt");

    // part_1
    //solve(sample, |entry: CalibrationEntry| entry.calibration_value());
    solve(input, |entry: CalibrationEntry| entry.calibration_value());


    // part_2
    //solve(sample2, |entry: CalibrationEntry| entry.calibration_value_2());
    //solve(input, |entry: CalibrationEntry| entry.calibration_value_2());
    // Wrong - I had assumed that "oneight" parses as [1], not [1, 8] i.e. chars are shared! fml

    // part_2_with_regex
    //solve(sample2, |entry: CalibrationEntry| entry.calibration_value_regex());
    solve(input, |entry: CalibrationEntry| entry.calibration_value_regex());

}

fn solve(input: &str, f: fn(CalibrationEntry) -> u32) {
    let sum = input.lines()
        //.take(1)
        .map(|line| {
            CalibrationEntry(line)
        })
        .map(f)
        .sum::<u32>();

    dbg!(sum);
}

#[derive(Debug)]
struct CalibrationEntry<'a>(&'a str);

impl<'a> CalibrationEntry<'a> {
    fn calibration_value(&self) -> u32 {
        let digits = self.0
            .chars()
            .filter_map(|c| {
                c.to_digit(10)
            })
            .collect::<Vec<_>>();

        let first = digits.first().unwrap().clone();
        let last = digits.last().unwrap().clone();

        first * 10 + last
    }

    fn calibration_value_2(&self) -> u32 {
        dbg!(self.0);
        // loop through all input
        // let (i, parsed) = parse_value(self.0).unwrap();
        // let mut parsed_vec = vec!(parsed);
        // let mut remaining = i;
        //
        // loop {
        //     let (i, parsed) = parse_value(remaining).unwrap();
        //     parsed_vec.push(parsed);
        //     if i.is_empty() { break; };
        //     remaining = i;
        // }
        // dbg!(parsed_vec);

        // Better, use nom::multi::many
        let digits = many0(parse_value)(self.0)
            .unwrap()
            .1
            .iter()
            // because we parse to an option we can use filter_map to unwrap only the Some elements
            .filter_map(|&v| v)
            .collect::<Vec<_>>();
        dbg!(&digits);

        let first = digits.first().unwrap().clone();
        let last = digits.last().unwrap().clone();
        let value = first * 10 + last;

        dbg!(value);
        value
    }


    fn calibration_value_regex(&self) -> u32 {
        dbg!(self.0);
        let searches = vec!(
            r"1",r"2",r"3",r"4",r"5",r"6",r"7",r"8",r"9",
            r"one",r"two",r"three",r"four",r"five",r"six",r"seven",r"eight",r"nine");

        let mut matches = searches.iter()
            .map(|search| {
                let re = Regex::new(search).unwrap();
                re.find_iter(self.0).collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        matches.sort_by(|a,b| a.start().cmp(&b.start()));
        let digits = matches.iter()
            .map(|m| match m.as_str() {
                "1"|"one" => 1,
                "2"|"two" => 2,
                "3"|"three" => 3,
                "4"|"four" => 4,
                "5"|"five" => 5,
                "6"|"six" => 6,
                "7"|"seven" => 7,
                "8"|"eight" => 8,
                "9"|"nine" => 9,
                _ => panic!()
            })
            .collect::<Vec<u32>>();

        let first = digits.first().unwrap().clone();
        let last = digits.last().unwrap().clone();
        let value = first * 10 + last;

        dbg!(value);
        value
    }
}

fn parse_value(input: &str) -> IResult<&str, Option<u32>> {
    alt((
        map(tag("one"), |_| Some(1)),
        map(tag("two"), |_| Some(2)),
        map(tag("three"), |_| Some(3)),
        map(tag("four"), |_| Some(4)),
        map(tag("five"), |_| Some(5)),
        map(tag("six"), |_| Some(6)),
        map(tag("seven"), |_| Some(7)),
        map(tag("eight"), |_| Some(8)),
        map(tag("nine"), |_| Some(9)),
        map(anychar, |c: char| c.to_digit(10)),
    ))(input)
}

