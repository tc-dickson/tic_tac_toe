use crate::board_info::{Player, Point, SquareType};
use crate::scoring::{GameStatus, MoveScoreTurns, PartialLineStatus};
use std::collections::HashSet;
use std::io;

// Contains the errors that can occur when playing the game
#[derive(Debug)]
enum BoardErr {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NumInputArgs(String),
    Move(String),
}

impl std::fmt::Display for BoardErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardErr::Io(e) => write!(f, "{e}"),
            BoardErr::Parse(e) => write!(f, "{e}"),
            BoardErr::NumInputArgs(e) | BoardErr::Move(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for BoardErr {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::num::ParseIntError> for BoardErr {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Parse(value)
    }
}

impl std::error::Error for BoardErr {}

/// A newtype wrapper to allow for custom `Display` of `Board.blank_squares_set`
#[derive(Clone, Debug)]
pub struct PointCollection(HashSet<Point>);

impl std::fmt::Display for PointCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.debug_set().entries(self.0.iter()).finish()
        let string_to_print = self
            .0
            .iter()
            .map(|point| format!("{point}"))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{string_to_print}")
    }
}

impl From<HashSet<Point>> for PointCollection {
    fn from(value: HashSet<Point>) -> Self {
        Self(value)
    }
}

/// This is the core of the entire program and contains all the data and functions needed to play a
/// game of tic-tac-toe.
#[derive(Clone)]
pub struct Board {
    content: Vec<Vec<SquareType>>,
    size: usize,
    blank_squares_set: PointCollection,
    game_status: GameStatus,
}

// This creates the classic *hashtag* board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_pass: Vec<String> = self
            .content
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| format!(" {y} "))
                    .collect::<Vec<String>>()
                    .as_slice()
                    .join("|")
            })
            .collect();

        let pass_length = self.size;
        let second_pass = first_pass
            .as_slice()
            .join(format!("\n{}\n", "-".repeat(pass_length * 4)).as_str());

        write!(f, "{second_pass}")
    }
}

impl Board {
    fn new(
        content: Vec<Vec<SquareType>>,
        size: usize,
        blank_squares_set: PointCollection,
        game_status: GameStatus,
    ) -> Self {
        Self {
            content,
            size,
            blank_squares_set,
            game_status,
        }
    }

    fn initialize_blank_board(size: usize) -> Board {
        let blank_array = vec![vec![SquareType::B; size]; size];
        let mut blank_squares_set: PointCollection = HashSet::new().into();
        for i in 0..size {
            for j in 0..size {
                blank_squares_set.0.insert(Point { x: i, y: j });
            }
        }
        Board::new(
            blank_array,
            size,
            blank_squares_set,
            GameStatus::StillPlaying,
        )
    }

    /// This is the function to run the tic-tac-toe game.
    ///
    /// It consists of the player alternating turns with the ai opponent. The player moves by
    /// typing two integer coordinates on the board separated by a space. If the player inputs
    /// invlaid information, they will receive an error message. Invalid input includes data that
    /// can't be read (this should rarely--if ever--happen), input that is more or less than two
    /// strings separated by a space, non-numeric input, and numeric input that is out of bounds or
    /// in an already played location. The game ends when either opponent wins or when the board is
    /// filled (i.e. a draw).
    ///
    pub fn run() {
        const BOARD_SIZE: usize = 3;

        fn get_user_move() -> Result<Point, BoardErr> {
            let mut player_move = String::new();
            io::stdin().read_line(&mut player_move)?;
            let user_input_as_usize = player_move
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<usize>, _>>()?;

            if user_input_as_usize.len() == Point::NUM_ARGUMENTS {
                Ok(Point {
                    x: user_input_as_usize[0],
                    y: user_input_as_usize[1],
                })
            } else {
                Err(BoardErr::NumInputArgs(format!(
                    "Incorrect number of input arguments. Got {}, Expected {}",
                    user_input_as_usize.len(),
                    Point::NUM_ARGUMENTS
                )))
            }
        }

        fn try_move(board: &mut Board, player: &Player) -> Result<(), BoardErr> {
            let user_move = get_user_move()?;
            board
                .insert(&user_move, player.square_type())
                .map_err(BoardErr::Move)?;
            Ok(())
        }

        let mut tic_tac_toe_board = Board::initialize_blank_board(BOARD_SIZE);
        let mut current_player = Player::X;

        while tic_tac_toe_board.game_status == GameStatus::StillPlaying {
            // Print board
            println!("\n{tic_tac_toe_board}\n");

            match current_player {
                Player::X => {
                    //let mut attempt_move = try_move(&mut tic_tac_toe_board, &current_player);
                    while let Err(e) = try_move(&mut tic_tac_toe_board, &current_player) {
                        println! {"{e}"};
                        //attempt_move = try_move(&mut tic_tac_toe_board, &current_player);
                    }
                }
                Player::O => {
                    // Calculate where the opponent should move
                    let opponent_move = tic_tac_toe_board
                        .alpha_beta(
                            &current_player,
                            9,
                            &MoveScoreTurns::MIN,
                            &MoveScoreTurns::MAX,
                        )
                        .player_move;
                    tic_tac_toe_board
                        .insert(&opponent_move, current_player.square_type())
                        .expect("alpha_beta() should not choose an invalid insert position");
                }
            }
            current_player = current_player.other();
        }

        // Print the final result of the game
        println!("Final Board: \n{tic_tac_toe_board}\n");
        println!("Final Status: {:?}", tic_tac_toe_board.game_status);
    }

    // This is an implementation of a depth-limited minmax algorithm with alpha-beta pruning
    // [Wikipedia][1] has a good explanation of the algorithm
    //
    // [1]: https://en.wikipedia.org/wiki/Alpha-beta_pruning
    fn alpha_beta(
        &self,
        player: &Player,
        depth: u32,
        alpha: &MoveScoreTurns,
        beta: &MoveScoreTurns,
    ) -> MoveScoreTurns {
        // Base case
        if depth == 0 || self.game_status != GameStatus::StillPlaying {
            return MoveScoreTurns {
                score: self.game_status,
                turns_to_win: depth,
                ..Default::default() // Point is immediately overwritten, so initialize it with
                                     // something convenient
            };
        }

        // Recursive case
        match player {
            // Maximizing player
            Player::X => {
                // Initialize values
                let mut value = MoveScoreTurns::MIN;
                let mut new_alpha = *alpha;
                let mut new_value; // This value is outside the loop due to lifetime considerations

                for blank_square in &self.blank_squares_set.0 {
                    // Create a copy of the board to explore potential moves and their outcomes
                    let mut new_board = self.clone();
                    let _ = new_board.insert(blank_square, player.square_type());
                    new_value = new_board.alpha_beta(&player.other(), depth - 1, &new_alpha, beta);
                    new_value.player_move = *blank_square; // Overwrite the returned board.player_move value
                                                           // to the move that was most recently played.
                                                           // This associates the correct return value with the correct move.

                    value = std::cmp::max(value, new_value);
                    if new_value > *beta {
                        break; // This is where the pruning takes place
                    }
                    new_alpha = std::cmp::max(value, new_alpha);
                }
                value
            }
            // Minimizing Player
            Player::O => {
                // Initialize values
                let mut value = MoveScoreTurns::MAX;
                let mut new_beta = *beta;
                let mut new_value; // This value is outside the loop due to lifetime considerations

                for blank_square in &self.blank_squares_set.0 {
                    // Create a copy of the board to explore potential moves and their outcomes
                    let mut new_board = self.clone();
                    let _ = new_board.insert(blank_square, player.square_type());
                    new_value = new_board.alpha_beta(&player.other(), depth - 1, alpha, &new_beta);
                    new_value.player_move = *blank_square; // Overwrite the returned board.player_move value
                                                           // to the move that was most recently played.
                                                           // This associates the correct return value with the correct move.

                    value = std::cmp::min(value, new_value);
                    if new_value < *alpha {
                        break; // This is where the pruning takes place
                    }

                    new_beta = std::cmp::min(value, new_beta);
                }
                value
            }
        }
    }

    fn update_status(&mut self) {
        if GameStatus::StillPlaying == self.game_status {
            self.game_status = self.check_status();
        }
    }

    // Adds a new `SquareType` to the `Board` and removes the corresponding value from the `blank_squares_set`
    fn insert(&mut self, point: &Point, value: SquareType) -> Result<(), String> {
        if let Some(point) = self.blank_squares_set.0.take(point) {
            self.content[point.x][point.y] = value;
            self.update_status();
            Ok(())
        } else {
            Err(format!(
                "Not a valid insert position: {point}. Valid positions are {}",
                self.blank_squares_set
            ))
        }
    }

    // In order for a game of tic-tac-toe to be won, a player needs to have marks in either an
    // entire horizontal row, an entire vertical column, or an entire diagonal. This implementation
    // does this checking in three parts and combines the results of the partial checks.
    fn check_status(&self) -> GameStatus {
        let all_lines_statuses = [self.check_rows(), self.check_cols(), self.check_diag()];
        all_lines_statuses
            .into_iter()
            .reduce(GameStatus::combine)
            .unwrap()
    }

    // Check each individual row if there is a either a win condition, a draw, or if it is
    // incomplete. The main functionality happens due to how the `SquareType`s combine together in
    // the first `reduce`, and how the fully checked lines then combine in the second `reduce`.
    fn check_rows(&self) -> GameStatus {
        self.content
            .iter()
            .map(|x| {
                x.iter()
                    .map(PartialLineStatus::PartialLine)
                    .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            })
            .map(|x| x.unwrap().upgrade())
            .reduce(GameStatus::combine)
            .unwrap()
    }

    // Operates in a similar manner to `check_rows()` but with additional set-up code to get the
    // right `SquareType`s to easily `reduce`
    fn check_cols(&self) -> GameStatus {
        let partial_line_board = self
            .content
            .iter()
            .map(|x| {
                x.iter()
                    .map(PartialLineStatus::PartialLine)
                    .collect::<Vec<PartialLineStatus>>()
            })
            .collect::<Vec<Vec<PartialLineStatus>>>();

        let column_reduction = partial_line_board
            .into_iter()
            .reduce(|acc, e| {
                acc.iter()
                    .zip(e.iter())
                    .map(|(x, y)| PartialLineStatus::combine(x, y))
                    .collect::<Vec<PartialLineStatus>>()
            })
            .unwrap();

        column_reduction
            .into_iter()
            .map(|e| e.upgrade())
            .reduce(GameStatus::combine)
            .unwrap()

        // println!("final_reduction: {:?}", final_reduction);
    }

    // Operates in a similar manner to `check_rows()` but with additional set-up code to get the
    // right `SquareType`s to easily `reduce`
    fn check_diag(&self) -> GameStatus {
        if self.content.len() != self.content[0].len() {
            todo!("Not a square board");
        }

        let l_to_r_diag = self
            .content
            .iter()
            .enumerate()
            .map(|(i, e)| &e[i])
            .map(PartialLineStatus::PartialLine)
            .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            .unwrap()
            .upgrade();

        let r_to_l_diag = self
            .content
            .iter()
            .rev()
            .enumerate()
            .map(|(i, e)| &e[i])
            .map(PartialLineStatus::PartialLine)
            .reduce(|acc, e| PartialLineStatus::combine(&acc, &e))
            .unwrap()
            .upgrade();

        GameStatus::combine(l_to_r_diag, r_to_l_diag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Used to easily initialize tic-tac-toe boards for testing purposes.
    /// Values within a row are separated by a space. Each row is separated 
    /// by a space and a vertical bar ('|') character.

    impl Board {
        fn from_string(string: &str) -> Self {
            let rows: Vec<&str> = string.split_terminator('|').collect();
            let size = rows.len();
            let mut cols;
            let mut col_vec: Vec<Vec<SquareType>> = Vec::new();
            let mut blank_squares_set: PointCollection = HashSet::new().into();
            for (i, i_val) in rows.iter().enumerate() {
                cols = i_val.split_whitespace().collect::<Vec<&str>>();
                let mut row_vec: Vec<SquareType> = Vec::new();
                for (j, j_val) in cols.iter().enumerate() {
                    match *j_val {
                        "B" => {
                            row_vec.push(SquareType::B);
                            blank_squares_set.0.insert(Point { x: i, y: j });
                        }
                        "O" => row_vec.push(SquareType::O),
                        "X" => row_vec.push(SquareType::X),
                        _ => println!("Not a matching square type: {j}"),
                    };
                }
                col_vec.push(row_vec);
            }

            let mut temp_board =
                Board::new(col_vec, size, blank_squares_set, GameStatus::StillPlaying);
            temp_board.update_status();
            temp_board
        }
    }

    #[test]
    fn row_x_win() {
        let x_win_board = Board::from_string(
            "X X X |
             O O X |
             X O O",
        );
        assert_eq!(x_win_board.check_rows(), GameStatus::XWin);
    }

    #[test]
    fn row_o_win() {
        let o_win_board = Board::from_string(
            "O O O |
             X B X |
             X O X",
        );
        assert_eq!(o_win_board.check_rows(), GameStatus::OWin);
    }

    #[test]
    fn row_still_playing() {
        let still_playing_board = Board::from_string(
            "X B X |
             O B X |
             X B O",
        );
        assert_eq!(still_playing_board.check_rows(), GameStatus::StillPlaying);
    }

    #[test]
    fn row_draw() {
        let draw_board = Board::from_string(
            "X X O |
             O O X |
             X O X",
        );
        assert_eq!(draw_board.check_rows(), GameStatus::Draw);
    }

    #[test]
    #[should_panic(expected = "Two winners")]
    fn row_x_and_o_win() {
        let x_and_o_win_board = Board::from_string(
            "X X X |
             O O O |
             X O X",
        );
        x_and_o_win_board.check_rows();
    }

    #[test]
    #[should_panic(expected = "Two winners")]
    fn row_o_and_x_win() {
        let o_and_x_win_board = Board::from_string(
            "O O O |
             X X X |
             X O X",
        );
        o_and_x_win_board.check_rows();
    }

    #[test]
    fn row_draw_col_win() {
        let col_win_board = Board::from_string(
            "X X O |
             X O O |
             X O X",
        );
        assert_eq!(col_win_board.check_rows(), GameStatus::Draw);
    }

    #[test]
    fn col_x_win() {
        let x_win_board = Board::from_string(
            "X X O |
             X O O |
             X O X",
        );
        assert_eq!(x_win_board.check_cols(), GameStatus::XWin);
    }

    #[test]
    fn col_o_win() {
        let o_win_board = Board::from_string(
            "O X X |
             O B O |
             O X X",
        );
        assert_eq!(o_win_board.check_cols(), GameStatus::OWin);
    }

    #[test]
    fn col_draw() {
        let draw_board = Board::from_string(
            "X X O |
             O X X |
             X O O",
        );
        assert_eq!(draw_board.check_cols(), GameStatus::Draw);
    }

    #[test]
    fn col_still_playing() {
        let still_playing_board = Board::from_string(
            "X X O |
             B O O |
             X O X",
        );
        assert_eq!(still_playing_board.check_cols(), GameStatus::StillPlaying);
    }

    #[test]
    #[should_panic(expected = "Two winners")]
    fn col_x_and_o_win() {
        let x_and_o_win_board = Board::from_string(
            "X O O |
             X O X |
             X O X",
        );
        x_and_o_win_board.check_cols();
    }

    #[test]
    #[should_panic(expected = "Two winners")]
    fn col_o_and_x_win() {
        let o_and_x_win_board = Board::from_string(
            "O X O |
             O X X |
             O X X",
        );
        o_and_x_win_board.check_cols();
    }

    #[test]
    fn col_draw_row_win() {
        let x_win_board = Board::from_string(
            "X X X |
             O B O |
             X O X",
        );
        assert_eq!(x_win_board.check_cols(), GameStatus::StillPlaying);
    }

    #[test]
    fn diag_x_win() {
        let x_win_board = Board::from_string(
            "X X O |
             O X O |
             X O X",
        );
        assert_eq!(x_win_board.check_diag(), GameStatus::XWin);
    }

    #[test]
    fn diag_o_win() {
        let o_win_board = Board::from_string(
            "X X O |
             O O X |
             O X X",
        );
        assert_eq!(o_win_board.check_diag(), GameStatus::OWin);
    }

    #[test]
    fn diag_draw() {
        let draw_board = Board::from_string(
            "X X O |
             O O X |
             X O X",
        );
        assert_eq!(draw_board.check_diag(), GameStatus::Draw);
    }

    #[test]
    fn diag_still_playing() {
        let still_playing_board = Board::from_string(
            "X X O |
             O B O |
             X O X",
        );
        assert_eq!(still_playing_board.check_diag(), GameStatus::StillPlaying);
    }
}
