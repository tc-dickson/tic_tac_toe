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

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum GameStatus {
    OWin,
    Draw,
    #[default]
    StillPlaying,
    XWin,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveScoreTurns {
    pub score: GameStatus,
    pub turns_to_win: u32,
    pub player_move: Point,
}

impl Ord for MoveScoreTurns {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        type GS = GameStatus;
        match (self.score, other.score) {
            (GS::OWin, GS::OWin) => {
                std::cmp::Ordering::reverse(self.turns_to_win.cmp(&other.turns_to_win))
            } // The higher turns to win should be less in this case
            (GS::XWin, GS::XWin) => self.turns_to_win.cmp(&other.turns_to_win), // The higher turns to win should be greater in this case

            (GS::Draw | GS::StillPlaying, GS::OWin)
            | (GS::XWin, GS::Draw | GS::StillPlaying | GS::OWin) => std::cmp::Ordering::Greater,

            (GS::Draw | GS::StillPlaying, GS::Draw | GS::StillPlaying) => std::cmp::Ordering::Equal,

            (GS::OWin, GS::Draw | GS::StillPlaying | GS::XWin)
            | (GS::Draw | GS::StillPlaying, GS::XWin) => std::cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for MoveScoreTurns {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for MoveScoreTurns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}):{}",
            self.player_move.x, self.player_move.y, self.score
        )
    }
}

impl MoveScoreTurns {
    pub const MAX: Self = Self {
        score: GameStatus::XWin,
        player_move: Point { x: 0, y: 0 },
        turns_to_win: 0,
    };

    pub const MIN: Self = Self {
        score: GameStatus::OWin,
        player_move: Point { x: 0, y: 0 },
        turns_to_win: 0,
    };
}
