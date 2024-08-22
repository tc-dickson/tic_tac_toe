use crate::board_info::{Point, SquareType};

#[derive(Debug)]
pub enum PartialLineStatus<'a> {
    PartialLine(&'a SquareType),
    PartialDraw,
}

impl<'a> PartialLineStatus<'a> {
    pub fn combine(lhs: &Self, rhs: &Self) -> Self {
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

    pub fn upgrade(&self) -> GameStatus {
        match self {
            Self::PartialLine(SquareType::B) => GameStatus::StillPlaying,
            Self::PartialLine(SquareType::O) => GameStatus::OWin,
            Self::PartialLine(SquareType::X) => GameStatus::XWin,
            Self::PartialDraw => GameStatus::Draw,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum GameStatus {
    XWin,
    Draw,
    StillPlaying,
    OWin,
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
    pub fn combine(lhs: Self, rhs: Self) -> Self {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MoveScoreDepth {
    pub player_move: Point,
    pub score: GameStatus,
    pub depth: u32,
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
