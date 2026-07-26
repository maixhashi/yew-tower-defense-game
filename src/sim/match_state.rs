use serde::{Deserialize, Serialize};

/// 試合の位相（State パターン）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MatchState {
    #[default]
    Playing,
    Paused,
    /// 城壁突破後の城内戦。
    Interior,
    Won,
    Lost,
}

/// 描画向けシーン区分（snapshot）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SceneMode {
    #[default]
    Exterior,
    Interior,
}
