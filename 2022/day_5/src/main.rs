use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::{tag, take}, combinator::{all_consuming, map, opt}, sequence::{delimited, preceded}, Finish, IResult, Parser};
use nom::bytes::complete::take_while1;
use nom::character::is_digit;
use nom::combinator::map_res;
use nom::Err::Error;
use nom::sequence::tuple;

fn main() -> color_eyre::Result<()> {

    // let x = 1;
    // let y = 1_usize;
    // let z: u64 = 1;

    let mut lines = include_str!("input.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();
    let mut piles = Piles(transpose_rev(crate_lines));
    println!("{piles:?}");

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    // let instructions: Vec<_> = lines
    //     .map(|line| all_consuming(parse_instruction)(line)
    //         .finish()
    //         .unwrap().1)
    //     .collect();
    // for ins in &instructions {
    //     println!("{ins:?}");
    // }

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply(ins);
        println!("{piles:?}");
    }

    let tops = piles.0.iter().map(|pile| pile.last().unwrap()).join("");
    println!("answer = {tops}");
    
    // let b = Vec<&Crate>::join("");
    // let a = ['a','b','c'].join("");
    // let a = ["a","b","c"].join("");
    // println!(
    //     "answer = {}",
    //     tops.join("")
    // );
    
    Ok(())
}


struct Crate(char);

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",  self.0)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // `drop` takes a value and returns nothing, which is
    // perfect for our case
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    }).parse(i)
    
    // map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
    //     s.parse::<usize>()
    // })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len).map(|_| {
        iters.iter_mut()
            .rev()
            .filter_map(|n| n.next().unwrap())
            .collect()
    }).collect()
}

struct Piles(Vec<Vec<Crate>>);

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let c = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(c);
        }
    }
}

impl Debug for Piles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for (i, pile) in self.0.iter().enumerate() {
                writeln!(f, "Pile {}: {:?}", i, pile)?;
            }
            Ok(())
    }
}