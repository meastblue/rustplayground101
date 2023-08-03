mod sudoku;

use sudoku::*;
fn main() {
    let sudoku = generator::Grid::new();

    for r in sudoku.grid.iter() {
        println!("{:?}", r);
    }
}
