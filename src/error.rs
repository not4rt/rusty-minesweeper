use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid board size: {0}")]
    InvalidBoardSize(usize),
    #[error("Invalid mines count: {0} for board size: {1}")]
    InvalidMinesCount(usize, usize),
    #[error("Invalid cell position: ({0}, {1})")]
    InvalidCellPosition(usize, usize),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type GameResult<T> = Result<T, GameError>;
