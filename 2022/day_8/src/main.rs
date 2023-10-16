use crate::grid::{Grid, GridCoord};

mod grid;

fn main() {
    let grid = parse_lines(include_str!("input.txt"));
    
    let all_coords = (0..grid.height())
        .into_iter()
        .flat_map(|y| {
            (0..grid.width())
                .into_iter()
                .map(move |x| GridCoord::from((x, y)))
        });
    
    let num_visible_cells = all_coords
        .filter(|&coord| {
            let cell_height = grid.cell(coord).unwrap();
            let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            // check each direction for any visibility
            deltas.iter().any(|&(dx, dy)| { 
                let mut cells_in_line = (1..)
                    .into_iter()
                    .map_while(|i| {
                        let curr = GridCoord {
                            x: coord.x.checked_add_signed(dx * i)?,
                            y: coord.y.checked_add_signed(dy * i)?,
                        };
                        grid.cell(curr)
                    });
                // a cell is visible if all the cells in one direction are less than it
                let is_visible = cells_in_line.all(|height| height < cell_height);
                
                println!("cell ({}, {}) isVisible:{} from direction [{},{}]", &coord.x, &coord.y, is_visible, dx, dy);
                is_visible
            })
        })
        .count();
    
    dbg!(num_visible_cells);

}

fn parse_lines(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    
    let mut grid = Grid::new(width, height);
    
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }
    
    grid
}

