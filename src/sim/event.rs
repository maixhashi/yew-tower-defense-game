use serde::{Deserialize, Serialize};

/// sim → UI / 演出向けの所有イベント。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
    PauseChanged { paused: bool },
    Breach { damage: f32, enemy_id: u32 },
    EnemyKilled { enemy_id: u32, reward: u32 },
    WaveCleared { wave: u32 },
    MatchEnded { won: bool },
}
