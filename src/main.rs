// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust
mod board_info;
mod config;
mod scoring;
mod tic_tac_toe_board;

fn main() {
    match config::Config::build(std::env::args()) {
        Ok(config) => tic_tac_toe_board::Board::run(&config),
        Err(e) => println!("{e}"),
    }
}
