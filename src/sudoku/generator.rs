use rand::Rng;

pub struct Grid {
    pub grid: Vec<Vec<i32>>,
    srn: usize,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: vec![vec![0; 9]; 9],
            srn: (9 as f64).sqrt() as usize,
        }
    }

    pub fn fill_values(&mut self) {
        self.fill_diagonal();
        self.fill_remaining(0, self.srn);
        self.remove_digits();
    }

    fn fill_diagonal(&mut self) {
        for i in (0..9).step_by(self.srn) {
            self.fill_box(i, i);
        }
    }

    fn fill_box(&mut self, row: usize, col: usize) {
        for i in 0..self.srn {
            for j in 0..self.srn {
                let mut num;
                loop {
                    num = self.random_generator(9);
                    if self.unused_in_box(row, col, num) {
                        break;
                    }
                }
                self.grid[row + i][col + j] = num;
            }
        }
    }

    fn random_generator(&self, num: i32) -> i32 {
        rand::thread_rng().gen_range(1..=num)
    }

    fn unused_in_row(&self, i: usize, num: i32) -> bool {
        !self.grid[i].contains(&num)
    }

    fn unused_in_col(&self, j: usize, num: i32) -> bool {
        !self.grid.iter().any(|row| row[j] == num)
    }

    fn unused_in_box(&self, row: usize, col: usize, num: i32) -> bool {
        for i in 0..self.srn {
            for j in 0..self.srn {
                if self.grid[row + i][col + j] == num {
                    return false;
                }
            }
        }
        true
    }

    fn fill_remaining(&mut self, i: usize, j: usize) -> bool {
        if j >= 9 && i < 8 {
            return self.fill_remaining(i + 1, 0);
        }

        if i >= 9 && j >= 9 {
            return true;
        }

        let mut next_i = i;
        let mut next_j = j;

        if i < self.srn {
            if j < self.srn {
                next_i = i;
                next_j = j;
            }
        } else if i < 9 - self.srn {
            if j == (i / self.srn) * self.srn {
                next_i = i;
                next_j = j + self.srn;
            }
        } else {
            if j == 9 - self.srn {
                next_i = i + 1;
                next_j = 0;
                if next_i >= 9 {
                    return true;
                }
            }
        }

        for num in 1..=9 as i32 {
            if self.check_if_safe(i, j, num) {
                self.grid[i][j] = num;
                if self.fill_remaining(next_i, next_j) {
                    return true;
                }
            }
        }

        false
    }

    fn check_if_safe(&self, i: usize, j: usize, num: i32) -> bool {
        self.unused_in_row(i, num)
            && self.unused_in_col(j, num)
            && self.unused_in_box(i - i % self.srn, j - j % self.srn, num)
    }

    fn remove_digits(&mut self) {
        let mut count = 20;
        while count != 0 {
            let cell_id = self.random_generator(9 * 9) as usize;
            let i = cell_id / 9;
            let mut j = cell_id % 9;

            if j != 0 {
                j -= 1;
            }

            if self.grid[i][j] != 0 {
                self.grid[i][j] = 0;
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
