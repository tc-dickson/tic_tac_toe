// This is a test to impliment the min-max algorithm in tic-tac-toe in Rust
pub mod board {
    pub enum SquareType {
        XSquare,
        OSquare,
        BlankSquare,
    }

    pub struct TicTacToeBoard {
        rows: i32,
        cols: i32,
        content: Vec<SquareType>,
    }

    impl TicTacToeBoard {
        pub fn new(rows: i32, cols: i32, content: Vec<SquareType>) -> Self {
            Self { rows, cols, content }
        }

        pub fn print_board(&self) {
            for i in 0..self.rows {
                for j in self 0..self.cols {
                    print!("i:{i}, j:{j}");
                }
            }
        }
    }
}

fn main() {
    let my_board = TicTacToeBoard.new(3, 3, vec![]);
}
