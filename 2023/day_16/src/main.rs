use std::collections::{HashMap, HashSet, VecDeque};
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr, s};
use crate::Direction::{D, L, R, U};

// This time we are going to try ndarray crate to see if its easier to work with matrices than nalgebra.
fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let elements: Vec<char> = input.lines()
        .map(|line| line.chars())
        .flatten()
        .collect();

    let grid = Array2::<char>::from_shape_vec((height, width),  elements).unwrap();

    // kicking the tyres
    // dbg!(&grid.column(0));
    // dbg!(&grid.row(0));
    // dbg!(&grid[(0, 5)]); // y, x not x, y
    // let i = Ix2(0, 5);
    // dbg!(&grid.get(i));
    // let s = &grid.slice(s![0..;-1, 0]);
    // dbg!(s);
    //dbg!(i + i);

    // depth first search
    let mut beams: HashSet<Beam> = HashSet::new();
    let mut queue = VecDeque::new();

    // needed to cheat this because I didnt want to change the entire trace logic for an edge case on first element
    let start = trace_beam(&grid, Ix2(0, 0), D).unwrap();
    queue.push_back(start);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        dbg!(&curr.points(&grid));

        if beams.contains(&curr) {
            // if we have processed this exact beam before its the start of a loop
            continue;
        }

        let (left, right) = get_next_beams(&grid, &curr);
        beams.insert(curr);

        if let Some(l) = left {
            queue.push_back(l);
        }

        if let Some(r) = right {
            queue.push_back(r);
        }
    }

    //part 1
    let all: HashSet<_> = beams.iter()
        .map(|beam| beam.points(&grid))
        .flatten()
        .collect();
    println!("sum is {}", &all.len());
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    U,
    D,
    L,
    R
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Beam {
    start: Ix2,
    end: Ix2,
    direction: Direction,
    // left: Option<Box<Beam>>, // this worked but made it hard to account for loops
    // right: Option<Box<Beam>>,
}

impl Beam {
    fn points(&self, grid: &ArrayBase<OwnedRepr<char>, Ix2>) -> Vec<(usize, usize)> {
        let points = match self.direction {
            U => (self.end[0]..=self.start[0]).collect::<Vec<_>>().iter().map(|&y| (y, self.start[1])).collect(),
            D => (self.start[0]..=self.end[0]).collect::<Vec<_>>().iter().map(|&y| (y, self.start[1])).collect(),
            L => (self.end[1]..=self.start[1]).collect::<Vec<_>>().iter().map(|&x| (self.start[0], x)).collect(),
            R => (self.start[1]..=self.end[1]).collect::<Vec<_>>().iter().map(|&x| (self.start[0], x)).collect(),
        };

        println!("start {},{} -- end {},{}", self.start[0], self.start[1], self.end[0], self.end[1]);
        dbg!(&points);

        points
    }
}

fn trace_beam(grid: &ArrayBase<OwnedRepr<char>, Ix2>, start: Ix2, direction: Direction) -> Option<Beam> {
    // include the element we are on so that we dont have to deal with bounds checking
    // i.e. starting where we ended is guaranteed to exist, adjacent is not
    let line = match direction {
        U => grid.slice(s![..=start[0];-1, start[1]]),
        D => grid.slice(s![start[0].., start[1]]),
        L => grid.slice(s![start[0], ..=start[1];-1]),
        R => grid.slice(s![start[0], start[1]..]),
    };

    dbg!(&direction);
    dbg!(&line);
    let mut dist = 0;
    let mut end_char = 'x';
    for (i, &c) in line.iter().enumerate().skip(1) {
        dist = i;
        end_char = c;

        match (&direction, c) {
            (_ , '/' | '\\') => break,
            (U | D, '-') => break,
            (L | R, '|') => break,
            _ => continue,
        }
    }

    let end = match direction {
        U => start - Ix2(dist, 0),
        D => start + Ix2(dist, 0),
        L => start - Ix2(0, dist),
        R => start + Ix2(0, dist),
    };

    if start == end {
        None
    }
    else {
        Some(Beam{ start, end, direction})
    }
}

fn get_next_beams(grid: &ArrayBase<OwnedRepr<char>, Ix2>, beam: &Beam) -> (Option<Beam>, Option<Beam>) {

    let end_char = grid.get(beam.end).unwrap();

    match end_char {
        '/' => match beam.direction {
            U => (None, trace_beam(grid, beam.end, R)),
            D => (trace_beam(grid, beam.end, L), None),
            L => (None, trace_beam(grid, beam.end, D)),
            R => (trace_beam(grid, beam.end, U), None),
        },
        '\\' => match beam.direction {
            U => (trace_beam(grid, beam.end, L), None),
            D => (None, trace_beam(grid, beam.end, R)),
            L => (trace_beam(grid, beam.end, U), None),
            R => (None, trace_beam(grid, beam.end, D)),
        },
        '|' => (trace_beam(grid, beam.end, U), trace_beam(grid, beam.end, D)),
        '-' => (trace_beam(grid, beam.end, L), trace_beam(grid, beam.end, R)),
        '.' => (None, None),
        _ => unreachable!()
    }
}
