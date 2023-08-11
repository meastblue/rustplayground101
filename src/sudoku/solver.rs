use super::generator::Grid;
pub struct Solver {
    pub grid: [[i32; 9]; 9],
}

impl Solver {
    pub fn new(g: Grid) -> (Self, bool) {
        let mut solver = Solver { grid: g.grid };
        let solved = solver.solve();

        (solver, solved)
    }

    fn solve(&mut self) -> bool {
        let mut solved = false;

        self.grid.clone().iter().enumerate().for_each(|(ir, row)| {
            row.iter().enumerate().for_each(|(ic, col)| {
                if *col != 0 || solved {
                    return;
                }

                (1..=9).for_each(|key| {
                    if self.is_valid_placement(&ir, &ic, &key) {
                        self.grid[ir][ic] = key;

                        if !self.solve() {
                            self.grid[ir][ic] = 0;
                        }

                        solved = true;
                        return;
                    }
                });
                solved = false;
                return;
            })
        });
        solved = true;

        solved
    }

    fn check_row_validity(&self, row: &usize, key: &i32) -> bool {
        self.grid[*row].contains(key)
    }

    fn check_column_validity(&self, col: &usize, key: &i32) -> bool {
        let mut in_column = false;

        (0..9).for_each(|row| {
            if self.grid[row][*col] == *key {
                in_column = true
            }
        });

        in_column
    }

    fn check_box_validity(&self, row: &usize, col: &usize, key: &i32) -> bool {
        let local_box_row = row - row % 3;
        let local_box_col = col - col % 3;
        let mut is_in_box = false;

        (local_box_row..local_box_row + 3).for_each(|i| {
            (local_box_col..local_box_col + 3).for_each(|j| is_in_box = self.grid[i][j] == *key)
        });

        is_in_box
    }
    fn is_valid_placement(&self, ir: &usize, ic: &usize, key: &i32) -> bool {
        !self.check_row_validity(ir, key)
            && !self.check_column_validity(ic, key)
            && !self.check_box_validity(ir, ic, key)
            && self.grid[*ir][*ic] != *key
    }
}
