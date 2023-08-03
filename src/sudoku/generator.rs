use rand::Rng;

pub struct Grid {
    pub grid: [[i32; 9]; 9],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: Self::generate_grid(),
        }
    }

    fn generate_grid() -> [[i32; 9]; 9] {
        let mut grid = [[0; 9]; 9];
        let mut rng = rand::thread_rng();

        for row in grid.iter_mut() {
            for col in row.iter_mut() {
                *col = rng.gen_range(1..10);
            }
        }

        Self::clean_grid(&mut grid);
        grid
    }

    fn clean_grid(grid: &mut [[i32; 9]; 9]) -> &mut [[i32; 9]; 9] {
        Self::clear_rows(grid);
        Self::clear_columns(grid);
        Self::clean_square(grid);

        grid
    }

    fn clear_rows(grid: &mut [[i32; 9]; 9]) -> &mut [[i32; 9]; 9] {
        for row in grid.iter_mut() {
            let mut seen = [false; 10];
            for col in row.iter_mut() {
                let value = *col;
                if value != 0 {
                    if seen[value as usize] {
                        *col = 0;
                    } else {
                        seen[value as usize] = true;
                    }
                }
            }
        }

        grid
    }

    fn clear_columns(grid: &mut [[i32; 9]; 9]) -> &mut [[i32; 9]; 9] {
        for i_col in 0..9 {
            let mut seen = [false; 10];

            for i_row in 0..9 {
                let value = grid[i_row][i_col];
                if value != 0 {
                    if seen[value as usize] {
                        grid[i_row][i_col] = 0;
                    } else {
                        seen[value as usize] = true;
                    }
                }
            }
        }

        grid
    }

    fn clean_square(grid: &mut [[i32; 9]; 9]) -> &mut [[i32; 9]; 9] {
        for row in (0..9).step_by(3) {
            for col in (0..9).step_by(3) {
                let mut seen = [false; 10];

                for i in 0..3 {
                    for j in 0..3 {
                        let r = row + i;
                        let c = col + j;
                        let value = grid[r][c];

                        if value != 0 {
                            let index = (value - 1) as usize;

                            if seen[index] {
                                grid[r][c] = 0;
                            } else {
                                seen[index] = true;
                            }
                        }
                    }
                }
            }
        }

        grid
    }
}
