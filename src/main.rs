// This is a test to impliment the min-max algorithm for tic-tac-toe in Rust
mod tic_tac_toe;

fn main() {
    let mut my_board = tic_tac_toe::TicTacToeBoard::from_string(
        "X X B |
         O O B |
         X O O",
    );

    println!("Display version of my_board:\n\n{}\n", my_board);
    println!("size: {:?}", my_board.size);
    println!("blank_squares_set: {:?}", my_board.blank_squares_set);
    println!("check_status: {:?}", my_board.game_status);

    my_board.insert(
        &tic_tac_toe::Point { x: 0, y: 2 },
        tic_tac_toe::SquareType::X,
    );
    println!("Display version of my_board:\n\n{}\n", my_board);
    println!("blank_squares_set: {:?}", my_board.blank_squares_set);
    println!("check_status: {:?}", my_board.game_status);

    my_board.insert(
        &tic_tac_toe::Point { x: 1, y: 2 },
        tic_tac_toe::SquareType::O,
    );
    println!("Display version of my_board:\n\n{}\n", my_board);
    println!("blank_squares_set: {:?}", my_board.blank_squares_set);
    println!("check_status: {:?}", my_board.game_status);

    my_board.insert(
        &tic_tac_toe::Point { x: 0, y: 1 },
        tic_tac_toe::SquareType::X,
    );
    println!("Display version of my_board:\n\n{}\n", my_board);
    println!("blank_squares_set: {:?}", my_board.blank_squares_set);
    println!("check_status: {:?}", my_board.game_status);

    my_board.insert(
        &tic_tac_toe::Point { x: 0, y: 2 },
        tic_tac_toe::SquareType::X,
    );
    my_board.insert(
        &tic_tac_toe::Point { x: 3, y: 3 },
        tic_tac_toe::SquareType::X,
    );

    // println!("check_rows: {:?}", my_board.check_rows());
    // println!("check_cols: {:?}", my_board.check_cols());
    // println!("check_diag: {:?}", my_board.check_diag());

    //  let blank_board = TicTacToeBoard::initialize_blank_board(3);
    //  println!("Display version of blank_board:\n\n{}\n", blank_board);
    //  println!("Debug version of blank_board:\n\n{:?}\n", blank_board);
}
