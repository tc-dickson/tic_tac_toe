/// This defines the possible states of the tic-tac-toe board
#[derive(Debug, Clone)]
pub enum SquareType {
    B, // Blank square
    O, 
    X, 
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

/// This along with `Opponent` helps define what pieces the user/computer is playing wirh 
pub enum Player {
    X,
    O,
}

impl Player {
    /// Return the player type that is not the one currently initialized
    pub fn other(&self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    /// `Player` X playes with X pieces
    /// `Player` O playes with O pieces
    pub fn square_type(&self) -> SquareType {
        match self {
            Player::X => SquareType::X,
            Player::O => SquareType::O,
        }
    }
}

/// The x, y coordinates of a square on a board
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
    // Define an associated constant to help eliminate "magic numbers"
    pub const NUM_ARGUMENTS: usize = 2;
}
