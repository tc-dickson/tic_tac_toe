use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum SquareType {
    B,
    O,
    X,
}

impl std::fmt::Display for SquareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B => write!(f, " "),
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub enum GameStatus {
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

pub enum Player {
    X,
    O,
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct TicTacToeBoard {
    pub content: Vec<Vec<SquareType>>,
    pub size: usize,
    pub blank_squares_set: HashSet<Point>,
    pub game_status: GameStatus,
}

impl std::fmt::Display for TicTacToeBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_pass = self
            .content
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| format!(" {} ", y))
                    .collect::<Vec<String>>()
                    .as_slice()
                    .join("|")
            })
            // .inspect(|e| println!("Display for TicTacToeBoard: {:?}", e))
            .collect::<Vec<String>>();

        let pass_length = self.size;
        let second_pass = first_pass
            .as_slice()
            .join(format!("\n{}\n", "-".repeat(pass_length * 4)).as_str());

        write!(f, "{}", second_pass)
    }
}

impl TicTacToeBoard {
    pub fn new(
        content: Vec<Vec<SquareType>>,
        size: usize,
        blank_squares_set: HashSet<Point>,
        game_status: GameStatus,
    ) -> Self {
        Self {
            content,
            size,
            blank_squares_set,
            game_status,
        }
    }

    pub fn from_string(string: &str) -> Self {
        let rows = string.split_terminator("|").collect::<Vec<&str>>();
        let size = rows.len();
        let mut cols;
        let mut col_vec: Vec<Vec<SquareType>> = Vec::new();
        let mut blank_squares_set = HashSet::new();
        for (i, i_val) in rows.iter().enumerate() {
            cols = i_val.split_whitespace().collect::<Vec<&str>>();
            let mut row_vec: Vec<SquareType> = Vec::new();
            for (j, j_val) in cols.iter().enumerate() {
                match j_val {
                    &"B" => {
                        row_vec.push(SquareType::B);
                        blank_squares_set.insert(Point { x: i, y: j });
                    }
                    &"O" => row_vec.push(SquareType::O),
                    &"X" => row_vec.push(SquareType::X),
                    _ => println!("Not a matching square type: {j}"),
                };
            }
            col_vec.push(row_vec);
        }

        let mut temp_board =
            TicTacToeBoard::new(col_vec, size, blank_squares_set, GameStatus::StillPlaying);
        temp_board.update_status();
        temp_board
    }

    pub fn initialize_blank_board(size: usize) -> TicTacToeBoard {
        let blank_array = vec![vec![SquareType::B; size]; size];
        let mut blank_squares_set: HashSet<Point> = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                blank_squares_set.insert(Point { x: i, y: j });
            }
        }
        TicTacToeBoard::new(
            blank_array,
            size,
            blank_squares_set,
            GameStatus::StillPlaying,
        )
    }

    pub fn minmax(board: &mut TicTacToeBoard, player: Player) -> Vec<(Point, GameStatus)> {
        let square_type = match player {
            Player::X => SquareType::X,
            Player::O => SquareType::O,
        };

        // Play in a blank square
        while let Some(next_blank_square) = board.blank_squares_set.iter().next() {
            let point_to_play = next_blank_square.clone();
            board.insert(&point_to_play, square_type);
            TicTacToeBoard::minmax(board, player.other());
        }
        ()
    }

    pub fn update_status(&mut self) {
        match self.game_status {
            GameStatus::StillPlaying => {
                self.game_status = self.check_status();
            }
            _ => (),
        }
    }

    pub fn insert(&mut self, point: &Point, value: SquareType) {
        if let Some(point) = self.blank_squares_set.take(point) {
            self.content[point.x][point.y] = value;
            self.update_status();
        } else {
            println!("Not a valid insert position: {:?}", point);
        }
    }

    fn check_status(&self) -> GameStatus {
        let all_lines_statuses = [self.check_rows(), self.check_cols(), self.check_diag()];
        all_lines_statuses
            .into_iter()
            .reduce(|acc, e| GameStatus::combine(acc, e))
            .unwrap()
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

    fn check_cols(&self) -> GameStatus {
        let partial_line_board = self
            .content
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| PartialLineStatus::PartialLine(y))
                    .collect::<Vec<PartialLineStatus>>()
            })
            .collect::<Vec<Vec<PartialLineStatus>>>();

        let column_reduction = partial_line_board
            .into_iter()
            // .inspect(|e| println!("partial_line_board: {:?}", e))
            .reduce(|acc, e| {
                acc.iter()
                    .zip(e.iter())
                    .map(|(x, y)| PartialLineStatus::combine(x, y))
                    .collect::<Vec<PartialLineStatus>>()
            })
            .unwrap();

        let final_reduction = column_reduction
            .into_iter()
            // .inspect(|e| println!("column_reduction: {:?}", e))
            .map(|e| e.upgrade())
            .reduce(|acc, e| GameStatus::combine(acc, e))
            .unwrap();

        // println!("final_reduction: {:?}", final_reduction);

        final_reduction
    }

    fn check_diag(&self) -> GameStatus {
        if self.content.len() != self.content[0].len() {
            todo!("Not a square board");
        }

        let l_to_r_diag = self
            .content
            .iter()
            .enumerate()
            // .inspect(|e| println!("{:?}", e))
            .map(|(i, e)| &e[i])
            // .inspect(|e| println!("{:?}", e))
            .map(|x| PartialLineStatus::PartialLine(x))
            .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            .unwrap()
            .upgrade();

        let r_to_l_diag = self
            .content
            .iter()
            .rev()
            .enumerate()
            // .inspect(|e| println!("r_to_l_diag: {:?}", e))
            .map(|(i, e)| &e[i])
            // .inspect(|e| println!("r_to_l_diag: {:?}", e))
            .map(|x| PartialLineStatus::PartialLine(x))
            .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            .unwrap()
            .upgrade();

        GameStatus::combine(l_to_r_diag, r_to_l_diag)
    }
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
        let col_win_board = TicTacToeBoard::from_string(
            "X X O |
             X O O |
             X O X",
        );
        assert_eq!(col_win_board.check_rows(), GameStatus::Draw);
    }

    #[test]
    fn col_x_win() {
        let x_win_board = TicTacToeBoard::from_string(
            "X X O |
             X O O |
             X O X",
        );
        assert_eq!(x_win_board.check_cols(), GameStatus::XWin);
    }

    #[test]
    fn col_o_win() {
        let o_win_board = TicTacToeBoard::from_string(
            "O X X |
             O B O |
             O X X",
        );
        assert_eq!(o_win_board.check_cols(), GameStatus::OWin);
    }

    #[test]
    fn col_draw() {
        let draw_board = TicTacToeBoard::from_string(
            "X X O |
             O X X |
             X O O",
        );
        assert_eq!(draw_board.check_cols(), GameStatus::Draw);
    }

    #[test]
    fn col_still_playing() {
        let still_playing_board = TicTacToeBoard::from_string(
            "X X O |
             B O O |
             X O X",
        );
        assert_eq!(still_playing_board.check_cols(), GameStatus::StillPlaying);
    }

    #[test]
    #[should_panic]
    fn col_x_and_o_win() {
        let x_and_o_win_board = TicTacToeBoard::from_string(
            "X O O |
             X O X |
             X O X",
        );
        x_and_o_win_board.check_cols();
    }

    #[test]
    #[should_panic]
    fn col_o_and_x_win() {
        let o_and_x_win_board = TicTacToeBoard::from_string(
            "O X O |
             O X X |
             O X X",
        );
        o_and_x_win_board.check_cols();
    }

    #[test]
    fn col_draw_row_win() {
        let x_win_board = TicTacToeBoard::from_string(
            "X X X |
             O B O |
             X O X",
        );
        assert_eq!(x_win_board.check_cols(), GameStatus::StillPlaying);
    }

    #[test]
    fn diag_x_win() {
        let x_win_board = TicTacToeBoard::from_string(
            "X X O |
             O X O |
             X O X",
        );
        assert_eq!(x_win_board.check_diag(), GameStatus::XWin);
    }

    #[test]
    fn diag_o_win() {
        let o_win_board = TicTacToeBoard::from_string(
            "X X O |
             O O X |
             O X X",
        );
        assert_eq!(o_win_board.check_diag(), GameStatus::OWin);
    }

    #[test]
    fn diag_draw() {
        let draw_board = TicTacToeBoard::from_string(
            "X X O |
             O O X |
             X O X",
        );
        assert_eq!(draw_board.check_diag(), GameStatus::Draw);
    }

    #[test]
    fn diag_still_playing() {
        let still_playing_board = TicTacToeBoard::from_string(
            "X X O |
             O B O |
             X O X",
        );
        assert_eq!(still_playing_board.check_diag(), GameStatus::StillPlaying);
    }
}
