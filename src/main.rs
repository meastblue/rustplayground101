mod sudoku;

use sudoku::{generator, solver};
fn main() {
    let sudoku = generator::Grid::new();

    for r in sudoku.grid.iter() {
        println!("{:?}", r);
    }

    solver::Solver::new(sudoku);
}
