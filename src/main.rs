// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust
mod tic_tac_toe;
use tic_tac_toe::*;

fn main() {
    let mut my_board = tic_tac_toe::TicTacToeBoard::from_string(
        "O X O |
         O X X |
         X O X",
    );

    // println!("Display version of my_board:\n\n{}\n", my_board);

    //  my_board.insert(
    //      &tic_tac_toe::Point { x: 0, y: 2 },
    //      tic_tac_toe::SquareType::X,
    //  );

    // println!("Display version of my_board:\n\n{}\n", my_board);

    let mut manual_move_score_table = vec![
        MoveScorePair {
            player_move: Point { x: 1, y: 1 },
            score: GameStatus::OWin,
        },
        MoveScorePair {
            player_move: Point { x: 2, y: 2 },
            score: GameStatus::Draw,
        },
        MoveScorePair {
            player_move: Point { x: 0, y: 0 },
            score: GameStatus::OWin,
        },
        MoveScorePair {
            player_move: Point { x: 1, y: 0 },
            score: GameStatus::XWin,
        },
        MoveScorePair {
            player_move: Point { x: 2, y: 0 },
            score: GameStatus::Draw,
        },
        MoveScorePair {
            player_move: Point { x: 3, y: 0 },
            score: GameStatus::OWin,
        },
        MoveScorePair {
            player_move: Point { x: 4, y: 0 },
            score: GameStatus::XWin,
        },
    ];

    //  println!("manual_move_score_table: {:?}", manual_move_score_table);
    //  println!(
    //      "select_score: {:?}",
    //      TicTacToeBoard::select_score(&mut manual_move_score_table, Player::X)
    //  );
    // println!("my_board GameStatus: {:?}", my_board.game_status);

    // println!("min-max output: {:?}", my_board.minmax(Player::O, 0));

    TicTacToeBoard::run();
}
