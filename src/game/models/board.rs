use std::collections::HashSet;

use crate::error::{GameError, GameResult};
use crate::game::models::cell::{Cell, CellContent, CellPosition};
use crate::game::models::game::GameDifficulty;

#[derive(PartialEq, Eq)]
pub enum RevealResult {
    Continue,
    GameOver(CellPosition),
    CantReveal,
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    size: usize,
    mine_positions: HashSet<CellPosition>,
    revealed_count: usize,
    flagged_count: isize,
}

impl Board {
    /// Creates a new game board with the specified difficulty settings.
    ///
    /// # Arguments
    /// * `difficulty` - Contains board size and number of mines
    ///
    /// # Returns
    /// * `GameResult<Self>` - New game board if parameters are valid
    ///
    /// # Errors
    /// * Returns `GameError::InvalidBoardSize` if board size is 0
    /// * Returns `GameError::InvalidMinesCount` if mines count is 0 or exceeds board capacity
    pub fn new(difficulty: GameDifficulty) -> GameResult<Self> {
        if difficulty.board_size == 0 {
            return Err(GameError::InvalidBoardSize(0));
        }

        let board_capacity = difficulty.board_size.pow(2);
        if difficulty.mines_count >= board_capacity || difficulty.mines_count == 0 {
            return Err(GameError::InvalidMinesCount(
                difficulty.mines_count,
                difficulty.board_size,
            ));
        }

        let mut board = Self {
            cells: vec![vec![Cell::default(); difficulty.board_size]; difficulty.board_size],
            size: difficulty.board_size,
            mine_positions: HashSet::with_capacity(difficulty.mines_count),
            revealed_count: 0,
            flagged_count: 0,
        };

        board.place_mines(difficulty.mines_count);
        board.calculate_adjacent_mines();

        Ok(board)
    }

    const fn validate_position(&self, pos: CellPosition) -> GameResult<()> {
        if pos.x >= self.size || pos.y >= self.size {
            return Err(GameError::InvalidCellPosition(pos.x, pos.y));
        }
        Ok(())
    }

    /// Attempts to flag a cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to flag
    ///
    /// # Returns
    /// * `GameResult<bool>` - Ok(true) if cell was flagged, Ok(false) if it couldn't be flagged
    ///
    /// # Errors
    /// Returns error if the position is invalid
    pub fn flag(&mut self, pos: CellPosition) -> GameResult<bool> {
        self.validate_position(pos)?;

        // Bug in the MineSweeper XP version: On the original game, you can have more flags than mines.
        // Maybe this is a feature, not a bug. This minesweeper aim to reproduce the original game.
        // On future versions, it could have this check below.
        // if self.flagged_count >= self.mine_positions.len() as isize {
        //     return Ok(false);
        // }

        // Bug in the MineSweeper XP version: On the original game, If you flag more than 99 + mines_count, the counter go to -00.
        // This minesweeper aim to reproduce the original game.
        // On future versions, it could have this check below.
        // if self.flagged_count > (99 + self.mine_positions.len()) as isize {
        //     return Ok(false);
        // }

        let was_flagged = self.cells[pos.x][pos.y].flag();

        if was_flagged {
            self.flagged_count = self.flagged_count.saturating_add(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Attempts to unflag a cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to unflag
    ///
    /// # Returns
    /// * `GameResult<bool>` - Ok(true) if cell was unflagged, Ok(false) if it couldn't be unflagged
    ///
    /// # Errors
    /// Returns error if the position is invalid
    pub fn unflag(&mut self, pos: CellPosition) -> GameResult<bool> {
        self.validate_position(pos)?;

        let was_unflagged = self.cells[pos.x][pos.y].unflag();

        if was_unflagged {
            self.flagged_count = self.flagged_count.saturating_sub(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Attempts to reveals the cell at the given position. If the cell is a mine, the game is over.
    /// If the cell is empty, it will reveal all adjacent empty cells.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to reveal
    ///
    /// # Returns
    /// * `GameResult<RevealResult>` -
    ///   `Ok(RevealResult::GameOver)` if the cell is a mine
    ///   `Ok(RevealResult::Continue)`e) if the game continues
    ///   `Ok(RevealResult::CantReveal)` if the cell is already revealed or flagged
    ///
    /// # Errors
    /// Returns error if the position is invalid or already revealed
    pub fn reveal(&mut self, pos: CellPosition) -> GameResult<RevealResult> {
        self.validate_position(pos)?;

        if self.cells[pos.x][pos.y].is_revealed() || self.cells[pos.x][pos.y].is_flagged() {
            return Ok(RevealResult::CantReveal);
        }

        self.cells[pos.x][pos.y].reveal();

        if self.cells[pos.x][pos.y].is_mine() {
            return Ok(RevealResult::GameOver(pos));
        }

        self.revealed_count = self.revealed_count.saturating_add(1);

        Ok(RevealResult::Continue)
    }

    fn place_mines(&mut self, mines_count: usize) {
        let mut mines_placed = 0;

        let mut rng = fastrand::Rng::new();

        while mines_placed < mines_count {
            let mine_pos = CellPosition {
                x: rng.usize(..self.size),
                y: rng.usize(..self.size),
            };

            if !self.cells[mine_pos.x][mine_pos.y].is_mine() {
                self.cells[mine_pos.x][mine_pos.y].content = CellContent::Mine;
                self.mine_positions.insert(mine_pos);
                mines_placed += 1;
            }
        }
    }

    /// Reveals all mines on the board.
    /// Used when the game is over.
    ///
    /// # Panics
    /// Will panic if `self.mine_positions` vector has invalid positions.
    pub fn reveal_mines(&mut self) {
        let mine_positions = self.mine_positions().clone();

        for mine_pos in mine_positions {
            if self.cells[mine_pos.x][mine_pos.y].is_flagged() {
                continue;
            }
            self.cells[mine_pos.x][mine_pos.y].reveal();
        }
    }

    /// Flags all mines on the board.
    /// Used when the game is over.
    ///
    /// # Panics
    /// Will panic if `self.mine_positions` vector has invalid positions.
    pub fn flag_mines(&mut self) {
        let mine_positions = self.mine_positions().clone();

        for mine_pos in mine_positions {
            self.cells[mine_pos.x][mine_pos.y].flag();
        }
    }

    pub fn adjacent_positions(&self, pos: CellPosition) -> impl Iterator<Item = CellPosition> + '_ {
        const OFFSETS: &[(isize, isize)] = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        OFFSETS.iter().filter_map(move |(dx, dy)| {
            let new_x = pos.x.checked_add_signed(*dx)?;
            let new_y = pos.y.checked_add_signed(*dy)?;

            (new_x < self.size && new_y < self.size).then_some(CellPosition { x: new_x, y: new_y })
        })
    }

    fn calculate_adjacent_mines(&mut self) {
        for mine in &self.mine_positions {
            let adj_positions: Vec<CellPosition> = self.adjacent_positions(*mine).collect();
            for adj_pos in adj_positions {
                self.cells[adj_pos.x][adj_pos.y].content.add_one();
            }
        }
    }

    /// Returns the cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to retrieve
    ///
    /// # Returns
    /// * `GameResult<&Cell>` - The cell at the given position
    ///
    /// # Errors
    /// Returns error if the position is invalid
    pub fn cell(&self, pos: CellPosition) -> GameResult<&Cell> {
        self.validate_position(pos)?;

        Ok(&self.cells[pos.x][pos.y])
    }

    #[must_use]
    pub const fn mine_positions(&self) -> &HashSet<CellPosition> {
        &self.mine_positions
    }

    #[must_use]
    pub const fn revealed_count(&self) -> usize {
        self.revealed_count
    }

    #[must_use]
    pub const fn flagged_count(&self) -> isize {
        self.flagged_count
    }

    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }
}
