// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust

#[derive(Debug)]
enum SquareType {
    B,
    O,
    X,
}

#[derive(Debug)]
enum PartialLineStatus<'a> {
    PartialLine(&'a SquareType),
    PartialDraw,
}

impl<'a> PartialLineStatus<'a> {
    fn combine(lhs: &Self, rhs: &Self) -> Self {
        type S = SquareType;

        match lhs {
            Self::PartialLine(S::B) => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) => Self::PartialLine(&S::B),
                Self::PartialLine(S::X) => Self::PartialLine(&S::B),
                Self::PartialDraw => Self::PartialLine(&S::B),
            },
            Self::PartialLine(S::O) => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) => Self::PartialLine(&S::O),
                Self::PartialLine(S::X) => Self::PartialDraw,
                Self::PartialDraw => Self::PartialDraw,
            },
            Self::PartialLine(S::X) => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) => Self::PartialDraw,
                Self::PartialLine(S::X) => Self::PartialLine(&S::X),
                Self::PartialDraw => Self::PartialDraw,
            },
            Self::PartialDraw => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) => Self::PartialDraw,
                Self::PartialLine(S::X) => Self::PartialDraw,
                Self::PartialDraw => Self::PartialDraw,
            },
        }
    }

    fn upgrade(&self) -> GameStatus {
        match self {
            Self::PartialLine(SquareType::B) => GameStatus::StillPlaying,
            Self::PartialLine(SquareType::O) => GameStatus::OWin,
            Self::PartialLine(SquareType::X) => GameStatus::XWin,
            Self::PartialDraw => GameStatus::Draw,
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameStatus {
    XWin,
    OWin,
    Draw,
    StillPlaying,
}

impl GameStatus {
    fn combine(lhs: Self, rhs: Self) -> Self {
        match lhs {
            Self::XWin => match rhs {
                Self::XWin => Self::XWin,
                Self::OWin => todo!("Two winners"),
                Self::Draw => Self::XWin,
                Self::StillPlaying => Self::XWin,
            },
            Self::OWin => match rhs {
                Self::XWin => todo!("Two winners"),
                Self::OWin => Self::OWin,
                Self::Draw => Self::OWin,
                Self::StillPlaying => Self::OWin,
            },
            Self::Draw => match rhs {
                Self::XWin => Self::XWin,
                Self::OWin => Self::OWin,
                Self::Draw => Self::Draw,
                Self::StillPlaying => Self::StillPlaying,
            },
            Self::StillPlaying => match rhs {
                Self::XWin => Self::XWin,
                Self::OWin => Self::OWin,
                Self::Draw => Self::StillPlaying,
                Self::StillPlaying => Self::StillPlaying,
            },
        }
    }
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
        GameStatus::StillPlaying
    }

    fn check_rows(&self) -> GameStatus {
        self.content
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| PartialLineStatus::PartialLine(y))
                    .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            })
            .map(|x| x.unwrap().upgrade())
            .reduce(|acc, e| GameStatus::combine(acc, e))
            .unwrap()
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
    my_board.check_rows();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_x_win() {
        let x_win_board = TicTacToeBoard::from_string(
            "X X X |
             O O X |
             X O O",
        );
        assert_eq!(x_win_board.check_rows(), GameStatus::XWin);
    }

    #[test]
    fn row_o_win() {
        let o_win_board = TicTacToeBoard::from_string(
            "O O O |
             X B X |
             X O X",
        );
        assert_eq!(o_win_board.check_rows(), GameStatus::OWin);
    }

    #[test]
    fn row_still_playing() {
        let still_playing_board = TicTacToeBoard::from_string(
            "X B X |
             O B X |
             X B O",
        );
        assert_eq!(still_playing_board.check_rows(), GameStatus::StillPlaying);
    }

    #[test]
    fn row_draw() {
        let draw_board = TicTacToeBoard::from_string(
            "X X O |
             O O X |
             X O X",
        );
        assert_eq!(draw_board.check_rows(), GameStatus::Draw);
    }

    #[test]
    #[should_panic]
    fn row_x_and_o_win() {
        let x_and_o_win_board = TicTacToeBoard::from_string(
            "X X X |
             O O O |
             X O X",
        );
        x_and_o_win_board.check_rows();
    }

    #[test]
    #[should_panic]
    fn row_o_and_x_win() {
        let o_and_x_win_board = TicTacToeBoard::from_string(
            "O O O |
             X X X |
             X O X",
        );
        o_and_x_win_board.check_rows();
    }

    #[test]
    fn row_draw_col_win() {
        let draw_board = TicTacToeBoard::from_string(
            "X X O |
             X O O |
             X O X",
        );
        assert_eq!(draw_board.check_rows(), GameStatus::Draw);
    }
}
