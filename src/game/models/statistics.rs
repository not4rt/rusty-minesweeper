use std::time::Duration;

use super::game::GameDifficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FastestTime {
    // pub nickname: String,
    pub total_time: Duration,
}

impl FastestTime {
    #[must_use]
    pub const fn new(
        // nickname: String,
        total_time: Duration,
    ) -> Self {
        Self {
            // nickname,
            total_time,
        }
    }
}

impl Default for FastestTime {
    fn default() -> Self {
        Self {
            // nickname: "Anonymous".to_string(),
            total_time: Duration::from_secs(999),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Statistics {
    begginer: FastestTime,
    intermediate: FastestTime,
    expert: FastestTime,
}

impl Statistics {
    pub fn add_best_time(
        &mut self,
        difficulty: GameDifficulty,
        // nickname: String,
        total_time: Duration,
    ) {
        match difficulty {
            GameDifficulty::Beginner => {
                self.begginer = FastestTime {
                    // nickname,
                    total_time,
                }
            }
            GameDifficulty::Intermediate => {
                self.intermediate = FastestTime {
                    // nickname,
                    total_time,
                }
            }
            GameDifficulty::Expert => {
                self.expert = FastestTime {
                    // nickname,
                    total_time,
                }
            }
            GameDifficulty::Custom(_, _) => (),
        }
    }

    pub fn reset_scores(&mut self) {
        self.begginer = FastestTime::default();
        self.intermediate = FastestTime::default();
        self.expert = FastestTime::default();
    }

    #[must_use]
    pub fn get_statistic(&self, difficulty: GameDifficulty) -> FastestTime {
        match difficulty {
            GameDifficulty::Beginner => self.begginer,
            GameDifficulty::Intermediate => self.intermediate,
            GameDifficulty::Expert => self.expert,
            GameDifficulty::Custom(_, _) => FastestTime::default(),
        }
    }

    #[must_use]
    pub const fn beginner(&self) -> &FastestTime {
        &self.begginer
    }

    #[must_use]
    pub const fn intermediate(&self) -> &FastestTime {
        &self.intermediate
    }

    #[must_use]
    pub const fn expert(&self) -> &FastestTime {
        &self.expert
    }
}
