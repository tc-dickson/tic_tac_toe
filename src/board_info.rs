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

/// This helps with configuring the user's preference for moving first/second and playing
/// with X/O pieces
pub enum Opponent {
    User,
    Computer,
}

impl Opponent {
    pub fn other(&self) -> Opponent {
        match self {
            Opponent::User => Opponent::Computer,
            Opponent::Computer => Opponent::User,
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
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    pub const NUM_ARGUMENTS: usize = 2;
}
