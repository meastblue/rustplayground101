use rand::Rng;

pub struct Grid {
    pub grid: [[i32; 9]; 9],
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Self { grid: [[0; 9]; 9] };

        grid.generate_grid();

        grid
    }

    fn generate_grid(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| {
                *col = rand::thread_rng().gen_range(1..10);
            })
        });

        self.clean_grid();
    }

    fn clean_grid(&mut self) {
        self.clear_rows();
        self.clear_columns();
        self.clean_box();
    }

    fn clear_rows(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            let mut seen = [false; 10];
            row.iter_mut().for_each(|col| {
                if *col != 0 {
                    if seen[*col as usize] {
                        *col = 0;
                    } else {
                        seen[*col as usize] = true;
                    }
                }
            })
        })
    }

    fn clear_columns(&mut self) {
        self.grid.clone().iter().enumerate().for_each(|(ic, col)| {
            let mut seen = [false; 10];
            col.iter().enumerate().for_each(|(ir, _)| {
                let value = self.grid[ir][ic];

                if value != 0 {
                    if seen[value as usize] {
                        self.grid[ir][ic] = 0;
                    } else {
                        seen[value as usize] = true;
                    }
                }
            });
        })
    }

    fn clean_box(&mut self) {
        self.grid
            .clone()
            .iter()
            .step_by(3)
            .enumerate()
            .for_each(|(ir, row)| {
                row.iter().enumerate().step_by(3).for_each(|(ic, _)| {
                    let mut seen = [false; 10];

                    (0..3).for_each(|i| {
                        (0..3).for_each(|j| {
                            let r = ir + i;
                            let c = ic + j;
                            let value = self.grid[r][c];

                            if value != 0 {
                                let index = (value - 1) as usize;

                                if seen[index] {
                                    self.grid[r][c] = 0;
                                } else {
                                    seen[index] = true;
                                }
                            }
                        })
                    })
                })
            })
    }
}
