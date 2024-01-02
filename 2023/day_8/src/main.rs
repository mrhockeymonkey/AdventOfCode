
use std::collections::HashMap;

fn main() {
    //let input = include_str!("sample1.txt");
    //let input = include_str!("sample2.txt");
    //let input = include_str!("sample3.txt");

    let input = include_str!("input.txt");
    let mut input_iter = input.lines().into_iter();
    // the example suggests EEE = (EEE, EEE) is valid so maybe this isnt strictly a tree structure...

    let mut dirs: RepeatingDirections = input_iter.next().unwrap().into();
    input_iter.next();

    let maps: Vec<_> = input_iter.map(|line| {
        let label = line[0..=2].to_string();
        let left = line[7..=9].to_string();
        let right = line[12..=14].to_string();

        Map { label, left, right, }
    })
    //.inspect(|m| {println!("{} = ({}, {})", m.label, m.left, m.right)})
    .collect();




    // let mut i:u64 = 0;
    // let mut curr = maps.iter().filter(|m| m.label == "AAA").nth(0).unwrap();
    let maps_map: HashMap<String, &Map> = maps.iter().map(|map| (map.label.clone(), map)).collect();

    // while curr.label != "ZZZ" {
    //     let d = dirs.next().unwrap();
    //     let next_label = match d {
    //         Direction::Left => &curr.left,
    //         Direction::Right => &curr.right,
    //     };
    //     i += 1;
    //     curr = maps_map.get(next_label).unwrap();
    // }

    // println!("It took {} steps", i);


    let mut i:u64 = 0;
    let mut curr: Vec<_> = maps.iter().filter(|m| m.label.ends_with('A')).collect();

    let cycles: Vec<_> = curr.clone().into_iter()
        .map(|m| find_cycles(m.clone(), &mut dirs.clone(), &maps_map))
        .inspect(|p| {dbg!(p);})
        .collect();

    dbg!(&cycles);
    // so every start point is in fact the start of a loop with. 
    // so the answer to part two is the lowest common multiple of all values 

    let values: Vec<_> = cycles.iter().map(|cycle| *cycle.last().unwrap()).collect();
    let part_2 = lcm(values.as_slice());

    println!("part 2: {}", part_2);
    println!("{}", 0);

}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Step {
    dir: Direction,
    label: String,
}

fn find_cycles(start: Map, dirs: &mut RepeatingDirections, map: &HashMap<String, &Map>) -> Vec<usize> {
    
    let mut last_ended = 0;
    let mut periods = vec![];
    let mut curr = start;

    for (i, d) in dirs.enumerate() {
        if curr.label.ends_with('Z') {
            periods.push(i - last_ended);
            last_ended = i;
        }

        if periods.len() >= 6 { break } // arbitrary number

        let next_label = match d {
            Direction::Left => curr.left,
            Direction::Right => curr.right,
        };
        let next = (**map.get(&next_label).unwrap()).clone();

        curr = next;
    }

    periods
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone)]
struct RepeatingDirections {
    dirs: Vec<Direction>,
    cursor: usize,
}

impl From<&str> for RepeatingDirections {
    fn from(value: &str) -> Self {
        RepeatingDirections {
            dirs: value.chars().map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            }).collect(),
            cursor: 0,
        }
    }
}

impl Iterator for RepeatingDirections {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.cursor;
        self.cursor = (self.cursor + 1) % self.dirs.len();

        Some(self.dirs[i])
    }
}

#[derive(Debug, Clone)]
struct Map {
    label: String,
    left: String,
    right: String,
}

