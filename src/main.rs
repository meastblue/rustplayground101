mod app;
mod sudoku;

fn main() {
    // app::App::new();
    let mut grid = sudoku::generator::Grid::new();
    grid.fill_values();
    grid.print_grid();
}
