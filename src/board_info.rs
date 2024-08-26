use crate::scoring::GameStatus;

/// An enum that holds the possible states of the tic-tac-toe board
/// The player plays with the X pieces and the opponent with the O pieces
#[derive(Debug, Clone)]
pub enum SquareType {
    B, // Blank square
    O, // Opponent's square
    X, // Player's square
}

/// For `SquareType::B` render a space. For the others, render the corresponding letter
impl std::fmt::Display for SquareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B => write!(f, " "),
            Self::O => write!(f, "O"),
            Self::X => write!(f, "X"),
        }
    }
}

pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    pub fn square_type(&self) -> SquareType {
        match self {
            Player::X => SquareType::X,
            Player::O => SquareType::O,
        }
    }

    pub fn desired_game_status(&self) -> GameStatus {
        match self {
            Player::X => GameStatus::XWin,
            Player::O => GameStatus::OWin,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
