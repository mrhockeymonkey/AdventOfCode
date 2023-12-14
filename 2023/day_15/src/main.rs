use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use nalgebra::{DMatrix, Matrix2x3, OMatrix};
use crate::Direction::{North, East, South, West};
use crate::Element::{RoundRock, Space, SquareRock};

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    // parse input into matrix
    let x = input.lines().next().unwrap().chars().count();
    let y = input.lines().count();

    let iter = input.lines()
        .map(|l| l.chars()
            .map(|c| match c {
            '.' => Space,
            '#' => SquareRock,
            'O' => RoundRock,
            _ => unreachable!(),
        }))
        .flatten();

    let grid = DMatrix::from_row_iterator(y, x, iter);

    // part 1
    // let tilted_north = tilt(grid, Direction::North);
    //
    // let mut total_weight = 0;
    // for (i, row) in tilted_north.row_iter().enumerate() {
    //     let factor = y - i;
    //     let round_rocks = row.iter().filter(|elem| matches!(elem,  RoundRock)).count();
    //     total_weight += round_rocks * factor;
    // }
    // println!("total weight {}", total_weight);

    // part 2
    let mut result = grid;
    let mut states = HashMap::new();
    let loop_tolerance = 500;
    let mut loop_count= 0;
    let mut loop_size = 0;

    // let steps = ((1_000_000_000 - 3) % 7) + 3; // sample

    // Iteration 179 saw this state already in iteration 153 which was 26 iterations ago
    // so the loop starts in 153 and is 26 cycles long
    let steps = ((1_000_000_000 - 153) % 26) + 153;

    for i in 1..=steps {
    //for i in 1..=5000 {
        result = spin_cycle(result);

        if let Some(prev_i) = states.insert(result.clone(), i) {
            let last_seen = i - prev_i;
            println!("Iteration {} saw this state already in iteration {} which was {} iterations ago", i, prev_i, last_seen);

            // if the loop size does not change within the loop tolerance we can break early
            if loop_size == last_seen {loop_count += 1} else {loop_size = last_seen}
            if loop_count > loop_tolerance {
                //
                break
            }
        }
    }

    let mut total_weight = 0;
    for (i, row) in result.row_iter().enumerate() {
        let factor = y - i;
        let round_rocks = row.iter().filter(|elem| matches!(elem,  RoundRock)).count();
        total_weight += round_rocks * factor;
    }
    println!("part 2 total weight {}", total_weight);


}

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
enum Element {
    RoundRock,
    Space,
    SquareRock,
}

enum Direction {
    North,
    East,
    South,
    West
}

fn spin_cycle(grid: DMatrix<Element>) -> DMatrix<Element> {
    let north = tilt(grid, Direction::North);
    let west = tilt(north, Direction::West);
    let south = tilt(west, Direction::South);
    tilt(south, Direction::East)
}

fn tilt(grid: DMatrix<Element>, direction: Direction) -> DMatrix<Element> {
    if let North | South = direction {
        let tilted_cols = grid.column_iter()
            .map(|col| col.iter().cloned().collect())
            .map(|elems| roll_rocks(elems, &direction).into_iter())
            .flatten()
            .collect::<Vec<_>>();

        DMatrix::from_iterator(grid.ncols(), grid.ncols(), tilted_cols)
    }
    else {
        let tilted_rows = grid.row_iter()
            .map(|row| row.iter().cloned().collect())
            .map(|elems| roll_rocks(elems, &direction).into_iter())
            .flatten()
            .collect::<Vec<_>>();

        DMatrix::from_row_iterator(grid.ncols(), grid.ncols(), tilted_rows)
    }
}

fn roll_rocks(source: Vec<Element>, direction: &Direction) -> Vec<Element> {
    let mut reordered = source.split(|elem| matches!(elem, SquareRock))
        .map(|part| {
            let mut sorted = part.to_vec();
            sorted.sort_unstable_by(|a, b| if matches!(direction, North) || matches!(direction, West) { a.cmp(b) } else { b.cmp(a) });
            sorted.into_iter().chain(Some(SquareRock).into_iter())
        })
        .flatten()
        .collect::<Vec<_>>();

    reordered.pop();
    reordered
}