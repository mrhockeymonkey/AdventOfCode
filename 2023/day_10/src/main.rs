use std::cmp::Ordering::Greater;
use std::collections::HashMap;

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let tiles = parse_tiles(input);
    let start = tiles.iter().filter(|tile| tile.value == 'S').next().unwrap();
    dbg!(start);

    let grid = Grid::new(&tiles);
    assert_eq!(grid.coords[&start.x][&start.y].value, start.value);

    let mut pipe_loop: Vec<&Tile> = vec![start];
    let mut curr = start;
    let mut prev = start;
    loop {
        // get the next pipe that is not the previous
        let next = grid.adjacent_and_legal(curr)
            .iter()
            .filter_map(|&x| x)
            .filter(|&t| {
                t.x != prev.x || t.y != prev.y
            })
            .next();

        if next.is_none() {
            let finished = grid.adjacent(curr).iter().any(|&t| t.is_some_and(|t| t.value == 'S'));
            assert!(finished);
            break
        }

        prev = curr;
        curr = next.unwrap();
        pipe_loop.push(next.unwrap());
    }

    //dbg!(pipe_loop);

    let dist = if pipe_loop.len() % 2 == 0 {
        pipe_loop.len() / 2
    } else {
        ((pipe_loop.len() - 1) / 2) + 1
    };

    println!("distance is {}", dist)
}

#[derive(Clone)]
enum Direction {
    Up,
    Down, 
    Left,
    Right,
}

#[derive(Debug)]
struct Grid<'a> {
    coords: HashMap<isize, HashMap<isize, &'a Tile>>,
    start: &'a Tile,
    curr: Option<&'a Tile>, // TODO would be better to use internal immutability
    prev: Option<&'a Tile>, // we also need to keep track of which way around we iterate
}

impl<'a> Grid<'a> {
    fn new(tiles: &'a Vec<Tile>) -> Self {
        let mut coords: HashMap<isize, HashMap<isize, &Tile>> = HashMap::new();
        let mut start = None;


        tiles.iter().for_each(|tile| {
            if tile.value == 'S' {
                start = Some(tile);
            }
            if let Some(line) = coords.get_mut(&tile.x){
                line.insert(tile.y, tile);
            }
            else {
                let mut new_line = HashMap::new();
                new_line.insert(tile.y, tile);
                coords.insert(tile.x, new_line);
            }
        });

        Grid{
            coords,
            start: start.unwrap(),
            curr: None,
            prev: None,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&Tile> {
        self.coords.get(&x)
            .and_then(|col| col.get(&y)
                .map(|x| *x))
    }
    
    fn is_legal_move(from: char, to: char, direction: Direction) -> bool {
        match direction {
            Direction::Up => match (from, to) {
                ('S' | '|' | 'J' | 'L', '|' | 'F' | '7') => true,
                _ => false
            },
            Direction::Down => match (from, to) {
                ('S' | '|' | 'F' | '7', '|' | 'J' | 'L') => true,
                _ => false
            },
            Direction::Right => match (from, to) {
                ('S' | '-' | 'F' | 'L', '-' | 'J' | '7') => true,
                _ => false
            },
            Direction::Left => match (from, to) {
                ('S' | '-' | 'J' | '7', '-' | 'F' | 'L') => true,
                _ => false
            },
        }
    }

    fn adjacent(&self, tile: &Tile) -> Vec<Option<&Tile>> {
        let dirs = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        dirs.into_iter()
            .map(|d| match d {
                Direction::Up => self.get(tile.x, tile.y - 1),
                Direction::Down => self.get(tile.x, tile.y + 1),
                Direction::Left => self.get(tile.x - 1, tile.y),
                Direction::Right => self.get(tile.x + 1, tile.y),
            })
            .collect::<Vec<_>>()
    }

    fn adjacent_and_legal(&self, tile: &Tile) -> Vec<Option<&Tile>> {
        let dirs = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        dirs.into_iter()
            .map(|d| (d.clone(), match d {
                Direction::Up => self.get(tile.x, tile.y - 1),
                Direction::Down => self.get(tile.x, tile.y + 1),
                Direction::Left => self.get(tile.x - 1, tile.y),
                Direction::Right => self.get(tile.x + 1, tile.y),
            }))
            .map(|(direction, adj_tile)| match adj_tile {
                Some(t) if Grid::is_legal_move(tile.value, t.value, direction) => Some(t),
                _ => None
            })
            .collect::<Vec<_>>()
    }
}

// todo this just isnt working
// impl<'a> Iterator for Grid<'a> {
//     type Item = &'a Tile;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let local_curr = self.curr;
//         let local_prev = self.prev;
//
//         if local_curr.is_none() {
//             self.curr = Some(self.start);
//             return Some(self.start)
//
//
//         let c_tile = local_curr.unwrap();
//         let adj = self.adjacent(c_tile);
//         let next = adj
//             .into_iter()
//             .filter(|&tile| match tile {
//                 None => false,
//                 Some(t) => {
//                     if let Some(prev_tile) = local_prev {
//                         t.x != prev_tile.x && t.y != prev_tile.y
//                     }
//                     else {
//                         true
//                     }
//                 }
//             })
//             .collect::<Vec<_>>();
//
//             // .filter_map(|x| x)
//             // .filter(|t| {
//             //     if let Some(prev_tile) = local_prev {
//             //         t.x != prev_tile.x && t.y != prev_tile.y
//             //     }
//             //     else {
//             //         true
//             //     }
//             // })
//             // .collect::<Vec<_>>(); // collect here so that we no longer borrow self.prev (i think)
//
//         let nexxt = next[0].unwrap();
//
//         self.prev = local_curr;
//         self.curr = Some(nexxt);
//
//         None
//         //Some(nexxt)
//     }
// }

#[derive(Debug)]
struct Tile {
    x: isize,
    y: isize,
    value: char
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input.lines()
        .enumerate()
        .map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, c)| Tile{
                x: x as isize,
                y: y as isize,
                value: c
            }))
        .flatten()
        .collect::<Vec<_>>()
}
