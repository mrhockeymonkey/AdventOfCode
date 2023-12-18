use std::collections::HashMap;
use ndarray::{Array2, Ix2, s};
use crate::Part::{Gear, Other};

// coming back and doing this after day_16 so I will keep using ndarray now I'm a little more familiar
fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let y_dim = input.lines().count();
    let x_dim = input.find(|c| c == '\n').unwrap();
    let elements: Vec<char> = input.lines()
        .map(|line| line.chars())
        .flatten()
        .collect();

    let grid = Array2::from_shape_vec((y_dim,x_dim), elements).unwrap();

    let part_numbers: Vec<PartNumber> = grid.indexed_iter()
        .fold((Vec::new(), Vec::new()), |(mut acc, mut tmp), (ix, &c)| {
            if c.is_digit(10) {
                tmp.push((ix, c));
            }
            else {
                if tmp.len() > 0 {
                    let num: String = tmp.iter().map(|(_, c)| c).collect();
                    acc.push(PartNumber {
                        value: u32::from_str_radix(&num,10).unwrap(),
                        start: tmp.first().unwrap().0,
                        end: tmp.last().unwrap().0,
                    });
                    tmp.clear();
                }
            }
            (acc, tmp)
        }).0;

    // part 1 find all part numbers that have an adjacent symbol and sum them up
    let sum: u32 = part_numbers.iter()
        .filter_map(|pn| pn.get_adjacent_symbol(&grid).and_then(|_| Some(pn)))
        .map(|pn| pn.value)
        .sum();

    println!("sum of parts is {}", sum);

    // part 2, we need to identify the symbols so that we can detect where a gear has two numbers adjacent
    let gears_map: HashMap<Part,  Vec<_>> = part_numbers.iter()
        .filter_map(|pn| pn.get_adjacent_symbol(&grid).and_then(|p| Some((p, pn))))
        .filter(|(p, pn)| matches!(p, Gear((_,_))))
        .fold(HashMap::new(), |mut acc, (p, pn)| {
            if acc.contains_key(&p) {
                acc.get_mut(&p).unwrap().push(pn);
            }
            else {
                acc.insert(p, vec![pn]);
            }
            acc
        });

    let gear_ratios_sum: u32 = gears_map.values()
        .filter(|&part_numbers| part_numbers.len() == 2)
        .map(|part_numbers| part_numbers.iter().fold(1, |mut acc, pn| acc * pn.value))
        .sum();

    println!("sum of gear ratios is {}", gear_ratios_sum);
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Part {
    Gear((usize, usize)),
    Other,
}

impl PartNumber {
    fn get_adjacent_symbol(&self, grid: &Array2<char>) -> Option<Part> {
        let max_row_index = grid.nrows() -1;
        let max_col_index = grid.ncols() -1;

        let y_range_start = if self.start.0 < 1 {0} else {self.start.0 - 1};
        let y_range_end = if self.end.0 >= max_row_index { max_row_index } else {self.end.0 + 1};
        let x_range_start = if self.start.1 < 1 {0} else { self.start.1 -1 };
        let x_range_end = if self.end.1 >= max_col_index { max_col_index } else {self.end.1 + 1};

        // println!("start {},{} end {},{}", self.start.0,  self.start.1, self.end.0, self.end.1);
        // println!("slice {},{} to {},{}", y_range_start, x_range_start, y_range_end, x_range_end);
        let slice =  grid.slice(s![y_range_start..=y_range_end, x_range_start..=x_range_end]);

        let symbols: Vec<_> = slice.indexed_iter()
            .filter(|(ix, &c)| !c.is_digit(10) && c != '.')
            .map(|((y_ix, x_ix), &c)| match c {
                '*' => Gear((y_range_start + y_ix, x_range_start + x_ix)), // adjust for the slice offset
                _ => Other
            })
            .collect();

        match &symbols[..] {
            [] => None,
            [first] => Some(first.clone()),
            [_, ..] => panic!("should not be more than one symbol adjacent!")
        }
    }
}