use std::collections::HashSet;
use std::io;

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
                Self::PartialLine(S::B | S::O | S::X) | Self::PartialDraw => {
                    Self::PartialLine(&S::B)
                }
            },
            Self::PartialLine(S::O) => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) => Self::PartialLine(&S::O),
                Self::PartialLine(S::X) | Self::PartialDraw => Self::PartialDraw,
            },
            Self::PartialLine(S::X) => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O) | Self::PartialDraw => Self::PartialDraw,
                Self::PartialLine(S::X) => Self::PartialLine(&S::X),
            },
            Self::PartialDraw => match rhs {
                Self::PartialLine(S::B) => Self::PartialLine(&S::B),
                Self::PartialLine(S::O | S::X) | Self::PartialDraw => Self::PartialDraw,
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameStatus {
    XWin,
    OWin,
    Draw,
    StillPlaying,
}

impl std::fmt::Display for GameStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XWin => write!(f, "XWin"),
            Self::OWin => write!(f, "OWin"),
            Self::Draw => write!(f, "Draw"),
            Self::StillPlaying => write!(f, "StillPlaying"),
        }
    }
}

impl GameStatus {
    fn combine(lhs: Self, rhs: Self) -> Self {
        match lhs {
            Self::XWin => match rhs {
                Self::OWin => unimplemented!("Two winners"),
                Self::Draw | Self::XWin | Self::StillPlaying => Self::XWin,
            },
            Self::OWin => match rhs {
                Self::XWin => unimplemented!("Two winners"),
                Self::OWin | Self::Draw | Self::StillPlaying => Self::OWin,
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
                Self::Draw | Self::StillPlaying => Self::StillPlaying,
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

    fn square_type(&self) -> SquareType {
        match self {
            Player::X => SquareType::X,
            Player::O => SquareType::O,
        }
    }

    fn desired_game_status(&self) -> GameStatus {
        match self {
            Player::X => GameStatus::XWin,
            Player::O => GameStatus::OWin,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct MoveScoreDepth {
    player_move: Point,
    score: GameStatus,
}

impl std::fmt::Display for MoveScoreDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}):{}",
            self.player_move.x, self.player_move.y, self.score
        )
    }
}

#[derive(Clone)]
pub struct Board {
    content: Vec<Vec<SquareType>>,
    size: usize,
    blank_squares_set: HashSet<Point>,
    game_status: GameStatus,
}

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

    fn initialize_blank_board(size: usize) -> Board {
        let blank_array = vec![vec![SquareType::B; size]; size];
        let mut blank_squares_set: HashSet<Point> = HashSet::new();
        for i in 0..size {
            for j in 0..size {
                blank_squares_set.insert(Point { x: i, y: j });
            }
        }
        Board::new(
            blank_array,
            size,
            blank_squares_set,
            GameStatus::StillPlaying,
        )
    }

    pub fn run() {
        // This is the function to run the tic-tac-toe game.
        let mut tic_tac_toe_board = Board::initialize_blank_board(3);
        let player = Player::X;

        while tic_tac_toe_board.game_status == GameStatus::StillPlaying {
            // Print board
            println!("\n{tic_tac_toe_board}\n");

            // Get user input to know where to play a move.
            let mut player_move = String::new();
            io::stdin()
                .read_line(&mut player_move)
                .expect("Failed to read input");
            let user_input_as_usize = player_move
                .split_whitespace()
                .map(|e| e.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let user_move = Point {
                x: user_input_as_usize[0],
                y: user_input_as_usize[1],
            };
            tic_tac_toe_board.insert(&user_move, player.square_type());

            // Calculate where the opponent should move
            let opponent_move = tic_tac_toe_board.minmax(&player.other(), 0).player_move;
            tic_tac_toe_board.insert(&opponent_move, player.other().square_type());
        }

        // Print the final result of the game
        println!("Final Board: \n{tic_tac_toe_board}\n");
        println!("Final Status: {:?}", tic_tac_toe_board.game_status);
    }

    fn minmax(&mut self, player: &Player, depth: u32) -> MoveScoreDepth {
        // Create a vector to hold the pairs of played squares and the eventual result (assuming optimal play)
        let mut move_score_table: Vec<MoveScoreDepth> = Vec::new();

        if self.game_status != GameStatus::StillPlaying {
            // Base case: if the code reaches here it means the game is over
            return MoveScoreDepth {
                // The player_move field will get updated so this is just a placeholder value
                player_move: Point { x: 0, y: 0 },
                score: self.game_status,
            };
        }

        // Recursive case: if the code reaches here it means the game is not yet over
        assert!(!self.blank_squares_set.is_empty());
        for i in &self.blank_squares_set {
            // Play in the next blank square
            let mut new_board = self.clone();
            new_board.insert(i, player.square_type());

            // Update the selected move's 'player_move' field to reflect the new information
            let mut selected_move = new_board.minmax(&player.other(), 0);
            selected_move.player_move = *i;
            move_score_table.push(selected_move);
        }
        Board::select_score(&mut move_score_table, player)
    }

    fn select_score(
        move_score_depth_table: &mut [MoveScoreDepth],
        player: &Player,
    ) -> MoveScoreDepth {
        // A helper function only used within select_score
        fn partition_to_front<F>(vec: Vec<MoveScoreDepth>, f: F) -> Vec<MoveScoreDepth>
        where
            F: FnMut(&MoveScoreDepth) -> bool,
        {
            let (mut first, last): (Vec<MoveScoreDepth>, Vec<MoveScoreDepth>) =
                vec.into_iter().partition(f);
            first.extend(last);
            first
        }

        let desired_game_status = player.desired_game_status();

        //let (desired_move_scores, other_move_scores): (Vec<MoveScoreDepth>, Vec<MoveScoreDepth>) =
        //    move_score_depth_table.iter().partition(|e| {
        //        matches!(
        //            e,
        //            MoveScoreDepth {
        //                player_move: _,
        //                score: x,
        //            } if *x == desired_game_status
        //        )
        //    });
        //
        //let (draw_move_scores, other_move_scores): (Vec<MoveScoreDepth>, Vec<MoveScoreDepth>) =
        //    other_move_scores.iter().partition(|e| {
        //        matches!(
        //            e,
        //            MoveScoreDepth {
        //                player_move: _,
        //                score: x,
        //            } if *x == GameStatus::Draw
        //        )
        //    });

        partition_to_front(move_score_depth_table.to_vec(), |e| {
            matches!(
                e,
                MoveScoreDepth {
                    player_move: _,
                    score: x,
                } if *x == desired_game_status
            )
        });
        
        unimplemented!();
        // Return the first element of the new sorted iterator
        //desired_move_scores
        //    .into_iter()
        //    .chain(draw_move_scores)
        //    .chain(other_move_scores)
        //    .next()
        //    .unwrap()
    }

    fn update_status(&mut self) {
        if GameStatus::StillPlaying == self.game_status {
            self.game_status = self.check_status();
        }
    }

    fn insert(&mut self, point: &Point, value: SquareType) {
        if let Some(point) = self.blank_squares_set.take(point) {
            self.content[point.x][point.y] = value;
            self.update_status();
        } else {
            println!("Not a valid insert position: {point:?}");
        }
    }

    fn check_status(&self) -> GameStatus {
        let all_lines_statuses = [self.check_rows(), self.check_cols(), self.check_diag()];
        all_lines_statuses
            .into_iter()
            .reduce(GameStatus::combine)
            .unwrap()
    }

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

    impl Board {
        fn from_string(string: &str) -> Self {
            let rows: Vec<&str> = string.split_terminator('|').collect();
            let size = rows.len();
            let mut cols;
            let mut col_vec: Vec<Vec<SquareType>> = Vec::new();
            let mut blank_squares_set = HashSet::new();
            for (i, i_val) in rows.iter().enumerate() {
                cols = i_val.split_whitespace().collect::<Vec<&str>>();
                let mut row_vec: Vec<SquareType> = Vec::new();
                for (j, j_val) in cols.iter().enumerate() {
                    match *j_val {
                        "B" => {
                            row_vec.push(SquareType::B);
                            blank_squares_set.insert(Point { x: i, y: j });
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

    #[test]
    fn select_score_1() {
        let mut manual_move_score_table = vec![
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin,
            },
        ];

        let selected_score = Board::select_score(&mut manual_move_score_table, &Player::X);
        assert_eq!(
            selected_score,
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::XWin
            }
        );
    }

    #[test]
    fn select_score_2() {
        let mut manual_move_score_table = vec![
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin,
            },
        ];

        let selected_score = Board::select_score(&mut manual_move_score_table, &Player::O);
        assert_eq!(
            selected_score,
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::OWin
            }
        );
    }

    #[test]
    fn select_score_3() {
        let mut manual_move_score_table = vec![
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 2 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin,
            },
        ];

        let selected_score = Board::select_score(&mut manual_move_score_table, &Player::O);
        assert_eq!(
            selected_score,
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            }
        );
    }

    #[test]
    fn select_score_4() {
        let mut manual_move_score_table = vec![
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin,
            },
        ];

        let selected_score = Board::select_score(&mut manual_move_score_table, &Player::X);
        assert_eq!(
            selected_score,
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin
            }
        );
    }

    #[test]
    fn select_score_5() {
        let mut manual_move_score_table = vec![
            MoveScoreDepth {
                player_move: Point { x: 0, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 0, y: 2 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 0 },
                score: GameStatus::XWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 1 },
                score: GameStatus::Draw,
            },
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::OWin,
            },
            MoveScoreDepth {
                player_move: Point { x: 2, y: 0 },
                score: GameStatus::XWin,
            },
        ];

        let selected_score = Board::select_score(&mut manual_move_score_table, &Player::O);
        assert_eq!(
            selected_score,
            MoveScoreDepth {
                player_move: Point { x: 1, y: 2 },
                score: GameStatus::OWin
            }
        );
    }
}
