use serde::{Deserialize, Serialize};

/// UI / render → sim へ渡す操作（所有 enum。キューに move で積む）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    TogglePause,
    SetPaused { paused: bool },
}
