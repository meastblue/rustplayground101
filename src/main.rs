mod sudoku;

use sudoku::{generator, solver};
fn main() {
    let sudoku = generator::Grid::new();

    for r in sudoku.grid.iter() {
        println!("{:?}", r);
    }

    println!("___________________");

    let solver = solver::Solver::new(sudoku);

    for s in solver.grid.iter() {
        println!("{:?}", s);
    }
}
