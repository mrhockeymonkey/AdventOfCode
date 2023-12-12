use crate::Pixel::{Galaxy, Space};

type Grid = Vec<Vec<Pixel>>;

fn main() {
    //let input = include_str!("sample.txt");
    let input = include_str!("input.txt");

    let grid = parse_input(input);
    let expanded = expand_universe(grid, 1_000_000);
    let points = get_points(&expanded);
    let vectors = get_vectors(points);

    let distances = vectors.iter()
        .map(|v| get_distance(v, &expanded))
        .collect::<Vec<_>>();

    println!("The sum of {} pairs is {}", distances.len(), distances.iter().sum::<u64>())
}

#[derive(Debug, Copy, Clone)]
enum Pixel {
    Space{ x_mag: u32, y_mag: u32},
    Galaxy(u32),
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vector {
    from: Point,
    to: Point,
}

fn parse_input(input: &str) -> Grid {
    let mut galaxy_id = 0;
    input.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '.' => Space{ x_mag: 1, y_mag: 1},
                '#' => {
                    galaxy_id += 1;
                    Galaxy(galaxy_id)
                },
                _ => panic!("There should be no other pixel types!")
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn expand_universe(mut grid: Grid, factor: u32) -> Grid {
    let height = grid.len();
    let width = grid[0].len();

    let mut y = 0;
    while y < height {
        if grid[y].iter().all(|pixel| if let Pixel::Space{ x_mag: _, y_mag: _ } = pixel {true} else {false}) {
            println!("{} is empty row", y);

            // replace the row with spaces that have a factored y_mag
            grid[y] = vec![Pixel::Space{ x_mag: 1, y_mag: 1 * factor}; width];
        }
        y += 1;
    }

    let mut x = 0;
    while x < width {
        if grid.iter().map(|y| y[x]).all(|pixel| if let Pixel::Space{ x_mag: _, y_mag: _ } = pixel {true} else {false}) {
            println!("{} is empty col", x);

            // replace the column spaces that have a factored x_mag and keep any existing y_mag
            grid.iter_mut().for_each(|row| {
                if let Pixel::Space{ x_mag: _, y_mag } = row[x] {
                    row[x] = Pixel::Space{ x_mag: 1 * factor, y_mag}
                }
                else {
                    unreachable!()
                }
            });
        }
        x += 1;
    }

    grid
}

fn get_points(grid: &Grid) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| row.iter()
            .enumerate()
            .filter_map(|(x, &pixel)| match pixel {
                Galaxy(_) => Some(Point{x, y}),
                _ => None
            })
            .collect::<Vec<_>>())
        .flatten()
        .collect()
}

fn get_vectors(points: Vec<Point>) -> Vec<Vector> {
    points.iter()
        .enumerate()
        .map(|(i, from)| points.iter()
            .skip(i + 1)
            .map(|to| Vector{from: from.clone(), to: to.clone()})
            .collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>()
}

fn get_distance(vector: &Vector, grid: &Grid) -> u64 {
    let (mut from_x, to_x) = match (vector.from.x, vector.to.x) {
        (from, to) if from <= to => (from, to),
        (from, to) if from > to => (to, from),
        _ => unreachable!()
    };

    let (mut from_y, to_y) = match (vector.from.y, vector.to.y) {
        (from, to) if from <= to => (from, to),
        (from, to) if from > to => (to, from),
        _ => unreachable!()
    };

    let mut x_component: u64 = 0;
    let mut y_component: u64 = 0;

    while from_x < to_x {
        match grid[from_y][from_x]{
            Pixel::Galaxy(_) => {
                x_component += 1;
            },
            Pixel::Space {x_mag, y_mag} => {
                x_component += x_mag as u64;
            }
        }
        from_x += 1;
    }

    while from_y < to_y {
        match grid[from_y][from_x]{
            Pixel::Galaxy(_) => {
                y_component += 1;
            },
            Pixel::Space {x_mag, y_mag} => {
                y_component += y_mag as u64;
            }
        }
        from_y += 1;
    }

    x_component + y_component
}

