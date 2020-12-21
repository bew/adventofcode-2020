use crate::common::{self, MyResult, ErrWithContext};

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
        // NOTE: `?` works for `Result` and `Option`, so we can avoid to match on
        //       Some & None if ONLY Some is useful for us:
        let row = self.rows.get(y)?;

        // NOTE: The grid is repeated infinitely to the right
        row.get(x % row.len())
    }

    fn get_at_pos(&self, position: &Position) -> Option<&Cell> {
        self.get_at_coords(position.x, position.y)
    }
}

fn parse_input(input_path: &str) -> MyResult<Grid> {
    let lines = common::read_lines(input_path)?;
    // In Rust world, it is known that a sequence of Result can be collected
    // to a Result with a Vec of the valid results.
    //
    // This is because Result implements FromIterator
    // (see https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E)
    // NOTE: I didn't deduced that, many posts around the internet talk about that!
    let rows: MyResult<Vec<Vec<Cell>>> = lines.iter().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, char)|
            match char {
                '.' => Ok(Cell::Space),
                '#' => Ok(Cell::Tree),
                _ => Err(format!("Invalid char '{}' in input grid at x:{} y:{}", char, x, y)),
            }
        ).collect()
    }).collect();

    // Return early if we have an error, and change the type for better use
    let rows = rows?;

    // Sanity check: all rows should have the same length
    let first_row = rows.iter().next().context("Missing first row of grid")?;
    if !rows.iter().all(|row| row.len() == first_row.len()) {
        return Err("Not all rows have the same length".to_string())
    }

    Ok(Grid { rows })
}

fn count_trees_on_descent(grid: &Grid, slope: Slope) -> usize {
    let mut tree_count = 0;
    let mut position = Position { x: 0, y: 0 };
    let mut end_position = position.with_slope(&slope);
    while let Some(cell) = grid.get_at_pos(&end_position) {
        tree_count += match cell {
            Cell::Tree => 1,
            Cell::Space => 0,
        };
        position = end_position;
        end_position = position.with_slope(&slope);
    };
    tree_count
}

pub fn solve_part1(input_path: &str) -> MyResult<usize> {
    let grid = parse_input(input_path).context("Failed to load input")?;

    let trees_count = count_trees_on_descent(&grid, Slope {x: 3, y: 1});
    Ok(trees_count)
}

pub fn solve_part2(input_path: &str) -> MyResult<usize> {
    let grid = parse_input(input_path).context("Failed to load input")?;

    let trees_1_1 = count_trees_on_descent(&grid, Slope {x: 1, y: 1});
    let trees_3_1 = count_trees_on_descent(&grid, Slope {x: 3, y: 1});
    let trees_5_1 = count_trees_on_descent(&grid, Slope {x: 5, y: 1});
    let trees_7_1 = count_trees_on_descent(&grid, Slope {x: 7, y: 1});
    let trees_1_2 = count_trees_on_descent(&grid, Slope {x: 1, y: 2});
    Ok(trees_1_1 * trees_3_1 * trees_5_1 * trees_7_1 * trees_1_2)
}
