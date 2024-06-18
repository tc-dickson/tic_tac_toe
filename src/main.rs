// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust

#[derive(Debug)]
enum SquareType {
    B,
    O,
    X,
}

#[derive(Debug)]
struct TicTacToeBoard {
    content: Vec<Vec<SquareType>>,
}

impl TicTacToeBoard {
    pub fn new(content: Vec<Vec<SquareType>>) -> Self {
        Self { content }
    }

    pub fn from_string(string: &str) -> Self {
        let rows = string.split_terminator("|").collect::<Vec<&str>>();
        let mut cols;
        let mut col_vec: Vec<Vec<SquareType>> = Vec::new();
        for i in rows.iter() {
            cols = i.split_whitespace().collect::<Vec<&str>>();
            let mut row_vec: Vec<SquareType> = Vec::new();
            for j in cols.iter() {
                match j {
                    &"B" => row_vec.push(SquareType::B),
                    &"O" => row_vec.push(SquareType::O),
                    &"X" => row_vec.push(SquareType::X),
                    _ => println!("Not a matching square type: {j}"),
                };
            }
            col_vec.push(row_vec);
        }

        TicTacToeBoard::new(col_vec)
    }

    pub fn print_board(&self) {
        for i in self.content.iter() {
            for j in i.iter() {
                print!("{j:?} ");
            }
            println!();
        }
    }
}

fn main() {
    let my_board = TicTacToeBoard::from_string("X O X | B O X | X O O");
    my_board.print_board();
}
