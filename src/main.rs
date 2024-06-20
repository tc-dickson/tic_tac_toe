// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust

#[derive(Debug)]
enum SquareType {
    B,
    O,
    X,
}

#[derive(Debug, PartialEq)]
enum GameStatus {
    XWin,
    OWin,
    Draw,
    StillPlaying,
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

    pub fn check_status(&self) -> GameStatus {
        //self.content.iter().reduce;
        //Hey
        return GameStatus::StillPlaying;
    }

    fn check_rows(&self, square_type: SquareType) -> bool {
        // let len = self.content.len();
        // self.content[0..len].iter().map(|x| println!("{x:?}"));
        true
        //      x == square_type)
        //  .reduce(|acc, e| acc && e)
        //self.content[0..len].iter().map(|x| x == square_type)
    }
}

fn main() {
    let my_board = TicTacToeBoard::from_string(
        "X O X |
         B O X |
         X O O",
    );
    my_board.print_board();
    println!("{my_board:?}");
    println!("content.len: {}", my_board.content.len());
    println!("content[0].len: {}", my_board.content[0].len());
    my_board.check_rows(SquareType::X);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_win() {
        let x_win_board = TicTacToeBoard::from_string(
            "X O X |
             X O O |
             X X O",
        );
        assert_eq!(x_win_board.check_status(), GameStatus::XWin);
    }
}
