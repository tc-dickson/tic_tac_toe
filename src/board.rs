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
