use rand::Rng;

const GRID_SIZE: usize = 9;
const BOX_SIZE: usize = 3;
const EMPTY_CELL: i32 = 0;
const NUM_TO_FILL: usize = 20;

pub struct Grid {
    pub grid: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Grid {
            grid: vec![vec![EMPTY_CELL; GRID_SIZE]; GRID_SIZE],
        };

        grid.fill_values();

        grid
    }

    pub fn fill_values(&mut self) {
        self.fill_diagonal();
        self.fill_remaining(0, 0);
        self.remove_digits();
    }

    fn fill_diagonal(&mut self) {
        for i in (0..GRID_SIZE).step_by(BOX_SIZE) {
            self.fill_box(i, i);
        }
    }

    fn fill_box(&mut self, start_row: usize, start_col: usize) {
        for i in 0..BOX_SIZE {
            for j in 0..BOX_SIZE {
                let num = self.generate_unique_random(start_row, start_col);
                self.grid[start_row + i][start_col + j] = num;
            }
        }
    }

    fn generate_unique_random(&self, row: usize, col: usize) -> i32 {
        let mut num;
        let mut rng = rand::thread_rng();
        loop {
            num = rng.gen_range(1..=GRID_SIZE as i32);
            if self.is_unused_in_row(row, num)
                && self.is_unused_in_col(col, num)
                && self.is_unused_in_box(row, col, num)
            {
                break;
            }
        }
        num
    }

    fn is_unused_in_row(&self, row: usize, num: i32) -> bool {
        !self.grid[row].contains(&num)
    }

    fn is_unused_in_col(&self, col: usize, num: i32) -> bool {
        !self.grid.iter().any(|row| row[col] == num)
    }

    fn is_unused_in_box(&self, start_row: usize, start_col: usize, num: i32) -> bool {
        for i in 0..BOX_SIZE {
            for j in 0..BOX_SIZE {
                if self.grid[start_row + i][start_col + j] == num {
                    return false;
                }
            }
        }
        true
    }

    fn fill_remaining(&mut self, row: usize, col: usize) -> bool {
        if row == GRID_SIZE {
            return true;
        }

        let mut next_row = row;
        let mut next_col = col + 1;

        if next_col >= GRID_SIZE {
            next_row += 1;
            next_col = 0;
        }

        if self.grid[row][col] != EMPTY_CELL {
            return self.fill_remaining(next_row, next_col);
        }

        for num in 1..=GRID_SIZE as i32 {
            if self.check_if_safe(row, col, num) {
                self.grid[row][col] = num;
                if self.fill_remaining(next_row, next_col) {
                    return true;
                }
                self.grid[row][col] = EMPTY_CELL;
            }
        }

        false
    }

    fn check_if_safe(&self, row: usize, col: usize, num: i32) -> bool {
        self.is_unused_in_row(row, num)
            && self.is_unused_in_col(col, num)
            && self.is_unused_in_box(row - row % BOX_SIZE, col - col % BOX_SIZE, num)
    }

    fn remove_digits(&mut self) {
        let mut rng = rand::thread_rng();
        let mut count = NUM_TO_FILL;
        while count > 0 {
            let cell_id = rng.gen_range(0..GRID_SIZE * GRID_SIZE);
            let i = cell_id / GRID_SIZE;
            let j = cell_id % GRID_SIZE;

            if self.grid[i][j] != EMPTY_CELL {
                self.grid[i][j] = EMPTY_CELL;
                count -= 1;
            }
        }
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
    }
}
