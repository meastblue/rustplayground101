use rand::Rng;

enum Level {
    Easy,
    Medium,
    Hard,
}

pub struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new() {
        let mut grid = [[0; 9]; 9];
        Self::generate_grid(&mut grid);
    }

    fn generate_grid(grid: &mut [[u8; 9]; 9]) {
        let mut rng = rand::thread_rng();

        for row in grid.iter_mut() {
            for col in row.iter_mut() {
                loop {
                    let rand = rng.gen_range(1..10);

                    if row.contains(&rand) {
                        *col = rand;
                        break;
                    }
                }
            }
        }

        for g in grid.iter() {
            println!("{:?}", g);
        }
    }
}
