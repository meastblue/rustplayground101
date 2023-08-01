use rand::Rng;

enum Level {
    Easy,
    Medium,
    Hard,
}

pub struct Sudoku {
    grid: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn new() {
        Self {
            grid: Self::generate_grid(),
        };
    }

    fn generate_grid() -> [[i32; 9]; 9] {
        let mut grid = [[0; 9]; 9];
        let mut rng = rand::thread_rng();

        for row in grid.iter_mut() {
            for col in row.iter_mut() {
                *col = rng.gen_range(1..10);
            }
        }

        grid
    }
}
