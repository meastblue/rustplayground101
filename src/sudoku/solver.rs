use super::generator::Grid;
pub struct Solver {
    pub grid: [[i32; 9]; 9],
}

impl Solver {
    pub fn new(g: Grid) -> Self {
        // let mut solver = Solver { grid: g.grid };
        let mut solver = Solver {
            grid: [
                [0, 0, 1, 0, 0, 0, 0, 0, 0],
                [2, 0, 0, 0, 0, 0, 0, 7, 0],
                [0, 7, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 4, 0, 6, 0, 0, 7],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 2, 5, 4, 6],
                [3, 0, 2, 7, 6, 0, 9, 8, 0],
                [0, 6, 4, 9, 0, 3, 0, 0, 1],
                [9, 8, 0, 5, 2, 1, 0, 6, 0],
            ],
        };

        if solver.solve() {
            println!("Solved successfully");
        } else {
            println!("Unsolvable board");
        }

        solver
    }

    // TODO fix insert value in grid
    fn solve(&mut self) -> bool {
        let mut solved = false;

        self.grid.iter().enumerate().for_each(|(ir, row)| {
            row.iter().enumerate().for_each(|(ic, _)| {
                if self.grid[ir][ic] == 0 && !solved {
                    for key in 1..=9 {
                        if self.is_valid_placement(&ir, &ic, &key) {
                            self.grid[ir][ic] = key;

                            if !self.solve() {
                                self.grid[ir][ic] = 0;
                            }

                            solved = true;
                            return;
                        }
                    }
                    solved = false;
                    return;
                }
            });
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

        (local_box_row..local_box_row + 3)
            .step_by(3)
            .map(|i| {
                (local_box_col..local_box_col + 3)
                    .step_by(3)
                    .map(|j| self.grid[i][j] == *key)
                    .all(|valid| !valid)
            })
            .all(|valid| valid)
    }

    fn is_valid_placement(&self, ir: &usize, ic: &usize, key: &i32) -> bool {
        !self.check_row_validity(ir, key)
            && !self.check_column_validity(ic, key)
            && !self.check_box_validity(ir, ic, key)
            && self.grid[*ir][*ic] != *key
    }
}
