use crate::error::GameResult;
use crate::models::board::{Board, RevealResult};
use crate::models::cell::CellPosition;
use crate::models::game::{GameDifficulty, GameStatus};
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct GameState {
    board: Board,
    difficulty: GameDifficulty,
    status: GameStatus,
    start_time: Instant,
    elapsed_seconds: u64,
    revealed_cells: HashSet<CellPosition>,
    flagged_cells: HashSet<CellPosition>,
}

impl GameState {
    /// Creates a new game state with the given [difficulty](GameDifficulty).
    ///
    /// # Errors
    /// Will return `GameError` if the board size is 0 or the mines count is invalid.
    pub fn new(difficulty: GameDifficulty) -> GameResult<Self> {
        let board: Board = Board::new(difficulty)?;
        Ok(Self {
            board,
            difficulty,
            status: GameStatus::InProgress,
            start_time: Instant::now(),
            elapsed_seconds: 0,
            revealed_cells: HashSet::with_capacity(difficulty.board_size.pow(2)),
            flagged_cells: HashSet::with_capacity(difficulty.mines_count),
        })
    }

    pub fn check_win_condition(&mut self) -> bool {
        if self.board.revealed_count()
            == (self.difficulty.board_size.pow(2) - self.difficulty.mines_count)
        {
            self.status = GameStatus::Won;
            self.board.flag_mines();
            self.flagged_cells.extend(self.board.mine_positions());
            return true;
        }
        false
    }

    /// Reveals the cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to reveal
    ///
    /// # Returns
    /// Returns a vector of revealed positions.
    ///
    /// # Errors
    /// Will return `GameError` if the position is invalid.
    pub fn reveal_cell(&mut self, pos: CellPosition) -> GameResult<()> {
        if self.status != GameStatus::InProgress {
            return Ok(());
        }

        if self.board.cell(pos)?.is_revealed() || self.board.cell(pos)?.is_flagged() {
            return Ok(());
        }

        let _ = self.reveal_area(pos);

        if self.status != GameStatus::Lost {
            self.check_win_condition();
        }
        Ok(())
    }

    fn reveal_area(&mut self, pos: CellPosition) -> GameResult<()> {
        match self.board.reveal(pos) {
            Ok(RevealResult::Continue) => {
                self.revealed_cells.insert(pos);

                // If the cell is empty, reveal surrounding area
                if self.board.cell(pos)?.is_empty() {
                    let adjacent_positions: Vec<CellPosition> =
                        self.board.adjacent_positions(pos).collect();
                    for adj_pos in adjacent_positions {
                        if !self.board.cell(adj_pos)?.is_revealed()
                            && !self.board.cell(adj_pos)?.is_flagged()
                        {
                            self.reveal_area(adj_pos)?;
                        }
                    }
                }
            }
            Ok(RevealResult::GameOver) => {
                self.revealed_cells.insert(pos);
                self.status = GameStatus::Lost;
                self.board.reveal_mines();
                self.revealed_cells.extend(self.board.mine_positions());
            }
            Ok(RevealResult::AlreadyRevealed) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Toggles the flag of the cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to toggle the flag
    ///
    /// # Returns
    /// Returns `true` if the flag was toggled, `false` if the cell is already revealed.
    ///
    /// # Errors
    /// Will return `GameError` if position is invalid
    pub fn toggle_flag(&mut self, pos: CellPosition) -> GameResult<bool> {
        if self.status.is_over() {
            return Ok(false);
        }

        if self.board.cell(pos)?.is_flagged() {
            return self.board.unflag(pos);
        } else if self.board.cell(pos)?.is_hidden() {
            return self.board.flag(pos);
        }

        Ok(false)
    }

    /// Toggles the flag of the cell at the given position.
    ///
    /// # Errors
    /// Will return `GameError` if the board size is 0 or the mines count is invalid.
    pub fn restart(&mut self) -> GameResult<()> {
        self.board = Board::new(self.difficulty)?;
        self.status = GameStatus::InProgress;
        self.elapsed_seconds = 0;
        self.start_time = Instant::now();
        self.revealed_cells.clear();
        self.flagged_cells.clear();

        Ok(())
    }

    #[must_use]
    pub const fn flags_remaining(&self) -> usize {
        self.difficulty
            .mines_count
            .saturating_sub(self.board.flagged_count())
    }

    #[must_use]
    pub const fn status(&self) -> &GameStatus {
        &self.status
    }

    #[must_use]
    pub const fn elapsed_seconds(&self) -> u64 {
        self.elapsed_seconds
    }

    #[must_use]
    pub const fn difficulty(&self) -> &GameDifficulty {
        &self.difficulty
    }

    pub fn tick(&mut self) {
        if !self.status.is_over() {
            self.elapsed_seconds = self.start_time.elapsed().as_secs();
        }
    }

    /// Changes the difficulty of the game.
    ///
    /// # Arguments
    /// * `difficulty` - The new difficulty to set
    ///
    /// # Panics
    /// Will panic if the game cannot be restarted with the new difficulty.
    pub fn change_difficulty(&mut self, difficulty: GameDifficulty) {
        self.difficulty = difficulty;
        self.restart()
            .expect("Failed to restart game. Bad difficulty?");
    }

    /// Returns the display string for the cell at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position of the cell to display
    ///
    /// # Errors
    /// Will return `GameError` if the position is invalid
    pub fn display_cell(&self, pos: CellPosition) -> GameResult<String> {
        Ok(self.board.cell(pos)?.to_string())
    }

    #[must_use]
    pub const fn revealed_cells(&self) -> &HashSet<CellPosition> {
        &self.revealed_cells
    }

    pub fn clear_revealed_cells(&mut self) {
        self.revealed_cells.clear();
    }

    #[must_use]
    pub const fn flagged_cells(&self) -> &HashSet<CellPosition> {
        &self.flagged_cells
    }

    pub fn clear_flagged_cells(&mut self) {
        self.flagged_cells.clear();
    }
}
