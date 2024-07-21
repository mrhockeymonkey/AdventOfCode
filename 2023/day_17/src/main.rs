use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::marker::PhantomData;
use ndarray::{Array2, Dimension, Ix2};
use crate::Direction::{North, East, South, West};

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let elements: Vec<_> = input.lines()
        .map(|line| line.chars())
        .flatten()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let grid = Array2::<u32>::from_shape_vec((height, width),  elements).unwrap();
    let end_pos = Ix2(grid.nrows() - 1, grid.ncols() - 1);
    //dbg!(&grid);

    let starts = vec![
        Node{ position: Ix2(0,0), direction: East, grid: &grid, fwd_count: 0},
        Node{ position: Ix2(0,0), direction: South, grid: &grid, fwd_count: 0},
    ];
    let distances = Dijkstra::cost(starts, &end_pos);
    dbg!(distances);

    // let ends = distances.0.iter()
    //     .filter(|(k, v)| k.position == end_pos)
    //     .reduce(|acc, d| {
    //         if d.1 < acc.1 {
    //             d
    //         } else {
    //             acc
    //         }
    //     }).unwrap();
    //
    // dbg!(ends);
    // let r = distances.1.get(&ends.0);
    // dbg!(r);

    // let end = distances.0.get(&end_pos).unwrap();
    // let endr = distances.1.get(&end_pos).unwrap();
    //
    // // dbg!(distances.get(&Ix2(1, 1)).unwrap());
    // // dbg!(distances.get(&Ix2(2, 2)).unwrap());
    // dbg!(&end);
    // dbg!(endr);
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Node<'a> {
    position: Ix2,
    direction: Direction,
    grid: &'a Array2<u32>,
    fwd_count: u32,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position[0].cmp(&other.position[0])
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Node<'a> {
    fn neighbours(&self) -> Vec<Node<'a>> {
        let y = self.position[0] as isize;
        let x = self.position[1] as isize;

        let lrf = match self.direction {
            North => (
                ((y, x - 1), West),
                ((y, x + 1), East),
                ((y - 1, x), North),
            ),
            East => (
                ((y - 1, x), North),
                ((y + 1, x), South),
                ((y, x + 1), East),
            ),
            South => (
                ((y, x + 1), East),
                ((y, x - 1), West),
                ((y + 1, x), South),
            ),
            West => (
                ((y + 1, x), South),
                ((y - 1, x), North),
                ((y, x - 1), West),
            ),
        };

        let mut nodes = vec![];
        if let Some(l) = Self::new(lrf.0.0, lrf.0.1, 1, self.grid) {
            nodes.push(l);
        }
        if let Some(r) = Self::new(lrf.1.0, lrf.1.1, 1, self.grid) {
            nodes.push(r);
        }
        if let Some(f) = Self::new(lrf.2.0, lrf.2.1, self.fwd_count + 1, self.grid) {
            if (self.fwd_count < 3) {
                nodes.push(f);
            }
        }

        nodes
    }

    fn new(pos: (isize, isize), dir: Direction, fwd: u32, grid: &'a Array2<u32>) -> Option<Node<'a>> {
        if pos.0 < 0 || pos.0 >= grid.nrows() as isize {
            return None
        }

        if pos.1 < 0 || pos.1 >= grid.ncols() as isize {
            return None
        }

        Some(Self {
            position: Ix2(pos.0 as usize, pos.1 as usize),
            direction: dir,
            grid: grid,
            fwd_count: fwd
        })
    }

    fn cost(&self) -> u32 {
        *self.grid.get(self.position).unwrap()
    }
}

#[derive(PartialEq, Eq)]
struct State<T> {
    node: T,
    cost: u32,
}

impl<T> State<T> {
    fn new(node: T, cost: u32) -> Self {
        Self { node, cost }
    }
}

impl<T> PartialOrd for State<T>
    where T: PartialEq + Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for State<T>
    where T: Eq + Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            //.then_with(|| self.node.cmp(&other.node))
    }
}

struct Dijkstra{
}

impl Dijkstra {
    fn cost(starts: Vec<Node>,  end: &Ix2) -> Option<u32> {
        let mut distances: HashMap<Node, u32> = HashMap::new();
        let mut routes: HashMap<Node, Vec<String>> = HashMap::new();
        //let mut distances: HashMap<Node, u32> = HashMap::new();
        //let mut q: VecDeque<(Node, (u32, Vec<String>))> = VecDeque::new();
        let mut q: BinaryHeap<State<Node>> = BinaryHeap::new();

        // initial nodes have a distance of zero
        for s in starts {
            distances.insert(s, 0);
            routes.insert(s, vec![]);
            //q.push_back((s, (0, vec![])));
            q.push(State::new(s, 0))
        }

        // measure distance to each neighbour
        while let Some(State{node, cost}) = q.pop() {
            //dbg!(node);
            if node.position[0] == end[0] && node.position[1] == end[1]  {
                return Some(cost)
            }

            //dbg!(&this_route);
            // check to see if we already have a shorter path
            let lowest_cost = *distances.get(&node).unwrap_or(&u32::MAX);
            if cost > lowest_cost {
                continue
            }

            for neighbour in node.neighbours() {
                //print!(" ({},{}) ", neighbour.position[0], neighbour.position[1]);
                let this_neighbour_cost = cost + neighbour.cost();

                // the current distance from start to this node so far
                let lowest_neighbour_cost = *distances.get(&neighbour).unwrap_or(&u32::MAX);

                // if (neighbour.position == Ix2(12,12)){
                //     //dbg!(node);
                //     println!("{:?} {} -- this {}, lowest {}", neighbour.direction, neighbour.fwd_count, this_neighbour_cost, lowest_neighbour_cost);
                //     dbg!(&this_route);
                // }

                if this_neighbour_cost < lowest_neighbour_cost {
                    *distances.entry(neighbour).or_insert(u32::MAX) = this_neighbour_cost;

                    //let mut rt = this_route.clone();
                    //rt.push(format!("{:?} + {} = {}", neighbour.position, neighbour.cost(), this_neighbour_cost));
                    //routes.insert(neighbour, rt.clone());
                    //q.push_back((neighbour, (this_neighbour_cost, rt)));
                    q.push(State::new(neighbour, this_neighbour_cost))

                    // dbg!(neighbour);
                    //println!("{},{} costs {}", neighbour.position[0], neighbour.position[1], this_neighbour_cost);
                    // println!("___");
                }
            }
            //println!();
        }

        None
    }
}
