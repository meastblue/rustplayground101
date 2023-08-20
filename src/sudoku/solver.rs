use super::generator::Grid;
pub struct Solver {
    pub grid: Vec<Vec<i32>>,
}

impl Solver {
    pub fn new(g: Grid) -> (Self, bool) {
        let mut solver = Solver { grid: g.grid };
        let solved = solver.solve();

        (solver, solved)
    }

    fn solve(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty_cell() {
            for key in 1..=9 {
                if self.is_valid_placement(row, col, key) {
                    self.grid[row][col] = key;

                    if self.solve() {
                        return true; // Puzzle solved
                    }

                    self.grid[row][col] = 0;
                }
            }

            false
        } else {
            true
        }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for (row, row_data) in self.grid.iter().enumerate() {
            if let Some(col) = row_data.iter().position(|&value| value == 0) {
                return Some((row, col));
            }
        }

        None
    }

    fn check_row_validity(&self, row: usize, key: i32) -> bool {
        self.grid[row].contains(&key)
    }

    fn check_column_validity(&self, col: usize, key: i32) -> bool {
        self.grid.iter().any(|row| row[col] == key)
    }

    fn check_box_validity(&self, start_row: usize, start_col: usize, key: i32) -> bool {
        let end_row = start_row + 3;
        let end_col = start_col + 3;

        for row in &self.grid[start_row..end_row] {
            if row[start_col..end_col].contains(&key) {
                return true;
            }
        }

        false
    }

    fn is_valid_placement(&self, row: usize, col: usize, key: i32) -> bool {
        !self.check_row_validity(row, key)
            && !self.check_column_validity(col, key)
            && !self.check_box_validity(row - row % 3, col - col % 3, key)
    }
}
