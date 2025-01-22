use crate::error::GameResult;
use crate::game::models::board::{Board, RevealResult};
use crate::game::models::cell::CellPosition;
use crate::game::models::game::{GameDifficulty, GameStatus};
use std::collections::{HashSet, VecDeque};
use std::time::{Duration, Instant};

pub struct GameState {
    board: Board,
    difficulty: GameDifficulty,
    status: GameStatus,
    start_time: Option<Instant>,
    elapsed_seconds: u64,
    revealed_cells: HashSet<CellPosition>,
    flagged_cells: HashSet<CellPosition>,
    custom_flags_remaining: isize,
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
            status: GameStatus::New,
            start_time: None,
            elapsed_seconds: 0,
            revealed_cells: HashSet::with_capacity(difficulty.board_size.pow(2)),
            flagged_cells: HashSet::with_capacity(difficulty.mines_count),
            custom_flags_remaining: 0,
        })
    }

    fn check_win_condition(&mut self) -> bool {
        if self.board.revealed_count()
            == (self.difficulty.board_size.pow(2) - self.difficulty.mines_count)
            && !self.status.is_lost()
        {
            self.status = GameStatus::Won;
            self.board.flag_mines();
            self.flagged_cells.extend(self.board.mine_positions());
            return true;
        }
        false
    }

    // Starts the game with already 1 second elapsed as the original game does.
    fn start_game(&mut self) {
        self.start_time = Some(
            Instant::now()
                .checked_sub(Duration::from_secs(1))
                .unwrap_or_else(Instant::now),
        );
        self.status = GameStatus::InProgress;
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
    pub fn reveal_cell(&mut self, pos: CellPosition) -> GameResult<RevealResult> {
        if self.board.cell(pos)?.is_revealed() || self.board.cell(pos)?.is_flagged() {
            return Ok(RevealResult::CantReveal);
        }

        if self.status.is_new() {
            self.start_game();
        }

        let reveal_result = self.reveal_area(pos)?;

        match reveal_result {
            RevealResult::Continue => {
                if !self.status.is_lost() {
                    self.check_win_condition();
                }
                Ok(RevealResult::Continue)
            }
            RevealResult::GameOver(mine_pos) => Ok(RevealResult::GameOver(mine_pos)),
            RevealResult::CantReveal => Ok(RevealResult::CantReveal),
        }
    }

    fn reveal_area(&mut self, start_pos: CellPosition) -> GameResult<RevealResult> {
        let mut to_reveal: VecDeque<CellPosition> = VecDeque::with_capacity(8);
        let mut visited: HashSet<CellPosition> = HashSet::with_capacity(32);

        to_reveal.push_back(start_pos);
        visited.insert(start_pos);

        while let Some(pos) = to_reveal.pop_front() {
            match self.board.reveal(pos)? {
                RevealResult::Continue => {
                    self.revealed_cells.insert(pos);

                    if self.board.cell(pos)?.is_empty() {
                        for adj_pos in self.board.adjacent_positions(pos) {
                            if !visited.insert(adj_pos) {
                                continue;
                            }

                            let cell = self.board.cell(adj_pos)?;
                            if !cell.is_revealed() && !cell.is_flagged() {
                                to_reveal.push_back(adj_pos);
                            }
                        }
                    }
                }
                RevealResult::GameOver(mine_pos) => {
                    self.revealed_cells.insert(pos);
                    self.status = GameStatus::Lost;
                    self.board.reveal_mines();

                    self.revealed_cells.extend(self.board.mine_positions());
                    return Ok(RevealResult::GameOver(mine_pos));
                }
                RevealResult::CantReveal => return Ok(RevealResult::CantReveal),
            }
        }

        Ok(RevealResult::Continue)
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

        if self.status.is_new() {
            self.start_game();
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
        self.status = GameStatus::New;
        self.elapsed_seconds = 0;
        self.start_time = None;
        self.custom_flags_remaining = 0;
        self.revealed_cells.clear();
        self.flagged_cells.clear();

        Ok(())
    }

    #[must_use]
    pub fn flags_remaining(&self) -> isize {
        if self.elapsed_seconds == 0 {
            self.custom_flags_remaining
        } else {
            let mines_count = self.difficulty.mines_count.try_into().unwrap_or(isize::MAX);
            mines_count - (self.board.flagged_count())
        }
    }

    #[must_use]
    pub const fn status(&self) -> &GameStatus {
        &self.status
    }

    #[must_use]
    pub const fn elapsed_seconds(&self) -> u64 {
        if self.elapsed_seconds < 999 {
            self.elapsed_seconds
        } else {
            999
        }
    }

    #[must_use]
    pub const fn difficulty(&self) -> &GameDifficulty {
        &self.difficulty
    }

    pub fn tick(&mut self) {
        if self.status.is_in_progress() {
            if let Some(start_time) = self.start_time {
                self.elapsed_seconds = start_time.elapsed().as_secs();
            }
        }

        if self.custom_flags_remaining
            < self.difficulty.mines_count.try_into().unwrap_or(isize::MAX)
        {
            self.custom_flags_remaining = self.custom_flags_remaining.saturating_add(1);
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
        if self.difficulty != difficulty {
            self.difficulty = difficulty;
        }
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
    #[inline]
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

    pub fn adjacent_positions(&self, pos: CellPosition) -> impl Iterator<Item = CellPosition> + '_ {
        self.board.adjacent_positions(pos)
    }

    /// Attempts to reveal all adjacent cells to the given position.
    ///
    /// # Errors
    /// Will return `GameError` if the game is already over.
    pub fn chording(&mut self, pos: CellPosition) -> GameResult<RevealResult> {
        if self.status.is_over() || self.board.cell(pos)?.is_hidden() {
            return Ok(RevealResult::CantReveal);
        }

        let mut flagged_adjacent = 0;
        let mut hidden: Vec<CellPosition> = Vec::with_capacity(8);

        for adj_pos in self.board.adjacent_positions(pos) {
            if self.board.cell(adj_pos)?.is_flagged() {
                flagged_adjacent += 1;
            } else if self.board.cell(adj_pos)?.is_hidden() {
                hidden.push(adj_pos);
            }
        }

        if flagged_adjacent != self.board.cell(pos)?.content.as_number() {
            return Ok(RevealResult::CantReveal);
        }

        let mut game_over = false;
        let mut end_cell = CellPosition::new(0, 0);
        let mut revealed = false;

        for adj_pos in hidden {
            match self.reveal_cell(adj_pos)? {
                RevealResult::GameOver(mine_pos) => {
                    game_over = true;
                    end_cell = mine_pos;
                }
                RevealResult::Continue => revealed = true,
                RevealResult::CantReveal => (),
            }
        }

        if game_over {
            Ok(RevealResult::GameOver(end_cell))
        } else if revealed {
            Ok(RevealResult::Continue)
        } else {
            Ok(RevealResult::CantReveal)
        }
    }
}
