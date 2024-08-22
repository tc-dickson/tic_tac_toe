// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust
mod tic_tac_toe_board;
mod scoring;
mod board_info;

fn main() {
    tic_tac_toe_board::Board::run();
}
