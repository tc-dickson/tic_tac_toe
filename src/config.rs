// Functionality for configuration of the tic-tac-toe game
#[derive(Debug)]
pub enum Error {
    PlayerPieceType(String),
    FirstOrSecond(String),
    NoArgument(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::FirstOrSecond(e) | Error::PlayerPieceType(e) | Error::NoArgument(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for Error {}

// New types to help with input parsing
pub struct PlayerIsX(pub bool);
pub struct MoveFirst(pub bool);

pub struct Config {
    pub player_piece_type: PlayerIsX,
    pub first_or_second: MoveFirst,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Error> {
        // Get rid of the first item in the iterator (which is the command name?)
        args.next();

        let player_piece_type;
        if let Some(arg) = args.next() {
            match arg.as_str() {
                "x" | "X" => player_piece_type = PlayerIsX(true),
                "o" | "O" => player_piece_type = PlayerIsX(false),
                x => {
                    return Err(Error::PlayerPieceType(format!(
                        "Expected \"X\" or \"O\". Got {x:?}"
                    )))
                }
            }
        } else {
            return Err(Error::NoArgument("No player_piece_type arg".to_string()));
        };

        let first_or_second;
        if let Some(arg) = args.next() {
            match arg.as_str() {
                "1" => first_or_second = MoveFirst(true),
                "2" => first_or_second = MoveFirst(false),
                x => return Err(Error::FirstOrSecond(format!( "Expected \"1\" or \"2\". Got {x:?}")))
                
            }
        } else {
            return Err(Error::NoArgument("No first_or_second arg".to_string()));
        }

        Ok(Config {
            player_piece_type,
            first_or_second,
        })
    }
}
