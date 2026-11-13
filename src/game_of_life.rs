// Game of Life, Weeks 12-13: grid, display, neighbor counts, the rule for one cell,
// generations, and preset patterns.
//
use crate::input::read_int;
use std::io::{self, Write};




const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

pub fn print_grid(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!("{}", if grid[r][c] { '#' } else { '.' });
        }
        println!();
    }
}

// Neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 && grid[r as usize][c as usize] {
                count += 1;
            }
        }
    }
    count
}

pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}
#[allow(dead_code)] 
pub fn print_neighbor_counts(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!("{}", count_neighbors(grid, r, c));
        }
        println!();
    }
}

// The next generation. Read from `grid` and write to a NEW grid, so a cell is never
// judged using neighbors that have already been updated.
pub fn next_generation(grid: &Grid) -> Grid {
    let mut next = new_grid();
    for r in 0..ROWS {
        for c in 0..COLS {
            let neighbors = count_neighbors(grid, r, c);
            next[r][c] = next_state(grid[r][c], neighbors);
        }
    }
    next
}



// Preset patterns placed near the middle of the grid (the glider starts near the
// top-left so it has room to travel):
//   1 = blinker: three cells in a row
//   2 = block: a 2x2 square
//   3 = glider: five cells
pub fn load_pattern(choice: u32) -> Grid {
    let mut grid = new_grid();
    match choice {
        1 => {
            grid[4][10] = true;
            grid[5][10] = true;
            grid[6][10] = true;
        }
        2 => {
            grid[6][14] = true;
            grid[6][15] = true;
            grid[7][14] = true;
            grid[7][15] = true;
        }
        3 => {
            grid[1][2] = true;
            grid[2][3] = true;
            grid[3][1] = true;
            grid[3][2] = true;
            grid[3][3] = true;
        }
        _ => {}
    }
    grid
}



// Number of live cells.
pub fn population(grid: &Grid) -> usize {
    let mut count = 0;
    for r in 0..ROWS {
        for c in 0..COLS {
            if grid[r][c] {
                count += 1;
            }
        }
    }
    count
}


pub fn play() {
    println!("Conway's Game of Life");
    println!("Choose a pattern:");
    println!("  1. Blinker");
    println!("  2. Block");
    println!("  3. Glider");

    let choice = read_int("Enter pattern choice (1-3): ", 1, 3);
    let generations = read_int("How many generations? ", 1, 100) as usize;

    let mut grid = load_pattern(choice);

    for generation in 0..=generations {
        println!(
            "\nGeneration {} (Population: {})",
            generation,
            population(&grid)
        );
        print_grid(&grid);

        if generation < generations {
            print!("Press Enter for next generation...");
            io::stdout().flush().unwrap();
            let mut pause = String::new();
            io::stdin().read_line(&mut pause).unwrap();
            grid = next_generation(&grid);
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(cells: &[(usize, usize)]) -> Grid {
        let mut grid = new_grid();
        for &(r, c) in cells {
            grid[r][c] = true;
        }
        grid
    }

    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    // Supplied by the instructor.
    #[test]
    fn block_is_still() {
        let block = grid_from(&[(4, 10), (4, 11), (5, 10), (5, 11)]);
        assert_eq!(next_generation(&block), block);
    }

    #[test]
    fn blinker_returns_after_two() {
        let start = grid_from(&[(4, 10), (5, 10), (6, 10)]);
        let after_two = next_generation(&next_generation(&start));
        assert_eq!(after_two, start);
    }

        #[test]
    fn glider_translates_after_four_generations() {
        let mut grid = load_pattern(3);
        assert_eq!(population(&grid), 5);

        for _ in 0..4 {
            grid = next_generation(&grid);
        }

        assert_eq!(population(&grid), 5);
                let expected = grid_from(&[(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)]);
        assert_eq!(grid, expected);

    }

    #[test]
    fn load_pattern_sets_expected_populations_and_defaults() {
        assert_eq!(population(&load_pattern(1)), 3);
        assert_eq!(population(&load_pattern(2)), 4);
        assert_eq!(population(&load_pattern(3)), 5);
        assert_eq!(population(&load_pattern(0)), 0);
        assert_eq!(population(&load_pattern(99)), 0);
    }

    #[test]
    fn empty_grid_remains_empty() {
        let grid = new_grid();
        let next = next_generation(&grid);
        assert_eq!(population(&next), 0);
    }


    // Catches a wrong next_generation that mutates the grid in place instead of reading from the original generation.
    #[test]
    fn blinker_after_one_generation_rotates_horizontal() {
        let grid = load_pattern(1);
        let gen1 = next_generation(&grid);
        let expected = grid_from(&[(5, 9), (5, 10), (5, 11)]);
        assert_eq!(gen1, expected);
    }

    // Catches a wrong next_generation whose loops stop one short of ROWS or COLS (e.g., 0..ROWS - 1 or 0..COLS - 1).
    #[test]
    fn bottom_right_block_stays_alive() {
        let grid = grid_from(&[
            (ROWS - 2, COLS - 2),
            (ROWS - 2, COLS - 1),
            (ROWS - 1, COLS - 2),
            (ROWS - 1, COLS - 1),
        ]);
        assert_eq!(next_generation(&grid), grid);
    }
}
