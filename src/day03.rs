use crate::common;

#[derive(Debug)]
struct Slope {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn with_slope(&self, slope: &Slope) -> Position {
        Position {
            x: (self.x as i32 + slope.x) as usize,
            y: (self.y as i32 + slope.y) as usize,
        }
    }
}

#[derive(Debug)]
enum Cell {
    Tree,
    Space,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn get_at_coords(&self, x: usize, y: usize) -> Option<&Cell> {
        // x:0 y:0 is the top-left cell
        match self.rows.get(y) {
            // NOTE: The grid is repeated infinitely to the right
            Some(row) => row.get(x % row.len()),
            None => None,
        }
    }

    fn get_at_pos(&self, position: &Position) -> Option<&Cell> {
        self.get_at_coords(position.x, position.y)
    }
}

fn parse_input(input_path: &str) -> Grid {
    let lines = common::read_lines(input_path);
    let rows: Vec<Vec<Cell>> = lines.iter().map(|line| {
        line.chars().map(|char| match char {
            '.' => Cell::Space,
            '#' => Cell::Tree,
            _ => panic!(format!("Invalid char '{}' in input grid", char)),
        }).collect()
    }).collect();

    // Sanity check: all rows should have the same length
    let first_row = rows.iter().next().unwrap();
    if !rows.iter().all(|row| row.len() == first_row.len()) {
        panic!("Not all rows have the same length")
    }

    Grid { rows }
}

fn count_trees_on_descent(grid: &Grid, slope: Slope) -> usize {
    let mut tree_count = 0;
    let mut position = Position { x: 0, y: 0 };
    let mut end_position = position.with_slope(&slope);
    while let Some(cell) = grid.get_at_pos(&end_position) {
        tree_count += match cell {
            Cell::Tree => 1,
            _ => 0,
        };
        position = end_position;
        end_position = position.with_slope(&slope);
    };
    tree_count
}

pub fn solve(input_path: &str) {
    // let grid = parse_input("./inputs/day03_example_grid.txt");
    let grid = parse_input(input_path);

    // part1
    let trees_count = count_trees_on_descent(&grid, Slope {x: 3, y: 1});
    println!("Day03 Part1: {}", trees_count);

    // part2
    let trees_1_1 = count_trees_on_descent(&grid, Slope {x: 1, y: 1});
    let trees_3_1 = count_trees_on_descent(&grid, Slope {x: 3, y: 1});
    let trees_5_1 = count_trees_on_descent(&grid, Slope {x: 5, y: 1});
    let trees_7_1 = count_trees_on_descent(&grid, Slope {x: 7, y: 1});
    let trees_1_2 = count_trees_on_descent(&grid, Slope {x: 1, y: 2});
    let part2_result = trees_1_1 * trees_3_1 * trees_5_1 * trees_7_1 * trees_1_2;
    println!("Day03 Part2: {}", part2_result);
}
