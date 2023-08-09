use super::generator::Grid;
pub struct Solver {
    grid: [[i32; 9]; 9],
}

impl Solver {
    pub fn new(g: Grid) -> Self {
        let mut solver = Solver { grid: g.grid };
        let key = 3;

        solver.solve(&key);

        solver
    }

    fn solve(&mut self, key: &i32) {
        self.grid.iter().enumerate().for_each(|(_, row)| {
            row.clone().iter().enumerate().for_each(|(ic, _)| {
                self.check_row_validity(row, &key);
                self.check_column_validity(&ic, &key);
            });
        });
    }

    fn check_row_validity(&self, row: &[i32; 9], key: &i32) -> bool {
        row.contains(key)
    }

    // TODO work on check column
    fn check_column_validity(&self, col: &usize, key: &i32) -> bool {
        let mut in_column = false;

        (0..9).for_each(|row| {
            if self.grid[row][*col] == *key {
                in_column = true
            }
        });

        in_column
    }

}
