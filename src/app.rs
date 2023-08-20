use crate::sudoku::{generator, solver};
use std::io::{Error, ErrorKind};

pub struct App;

impl App {
    pub fn new() {
        Self::run();
    }

    fn run() {
        let grd = generator::Grid::new();

        println!("Hello, sudoku player!");
        println!("This is a simple sudoku game.");
        println!("It's generated randomly sudoku grid.");
        println!("You can solve it by yourself or you can ask the program to solve it for you.");
        println!("Would you to generate a new sudoku grid? (y/n)");

        match Self::get_user_input() {
            true => {
                println!("Generating a new sudoku grid...");
                println!("This is your new sudoku grid:");
                grd.print_grid();
            }
            false => {
                println!("You have chosen to exit the program.");
                println!("Goodbye!");
                std::process::exit(0);
            }
        }

        let (slv, is_legit) = solver::Solver::new(grd);

        println!("Would you like to see the solution of the sudoku grid? (y/n)");

        loop {
            match Self::get_user_input() {
                true => {
                    if !is_legit {
                        println!("The sudoku grid is not solvable.");
                        println!("Goodbye!");
                        std::process::exit(0);
                    }

                    println!("Solving the sudoku grid...");
                    println!("This is the solved sudoku grid:");
                    slv.grid.iter().for_each(|row| {
                        println!("{:?}", row);
                    });
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                false => {
                    println!("You have chosen to continue by yourself.");
                }
            }
        }
    }

    fn get_user_input() -> bool {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        loop {
            match App::check_input(&input) {
                Ok(valid) => break valid,
                Err(_) => Self::get_user_input(),
            };
        }
    }

    fn check_input(input: &str) -> Result<bool, Error> {
        match input.trim() {
            "y" | "Y" | "yes" | "Yes" | "YES" => Ok(true),
            "n" | "N" | "no" | "No" | "NO" => Ok(false),
            _ => {
                println!("Please enter a valid input (y/n)");
                Err(Error::new(ErrorKind::InvalidInput, "Invalid input"))
            }
        }
    }
}
