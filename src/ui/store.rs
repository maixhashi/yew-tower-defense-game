use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

/// 司令部 UI のセッション状態（ポーズ・選択カードなど）。
/// ゲームシミュレーション（HP・敵位置など）はここに載せない。
#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct CommandHqStore {
    pub is_paused: bool,
    pub selected_card: Option<DefenseCard>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenseCard {
    Cannon,
    Archer,
    Barricade,
    FrostArcher,
    Mortar,
}

impl DefenseCard {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cannon => "大砲",
            Self::Archer => "弓兵",
            Self::Barricade => "バリケード",
            Self::FrostArcher => "氷弓",
            Self::Mortar => "迫撃砲",
        }
    }

    pub const fn type_id(self) -> &'static str {
        match self {
            Self::Cannon => "cannon",
            Self::Archer => "archer",
            Self::Barricade => "barricade",
            Self::FrostArcher => "frost_archer",
            Self::Mortar => "mortar",
        }
    }
}

/// 設定など、ブラウザ再訪後も残したい UI 状態。
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct UiSettingsStore {
    pub sound_muted: bool,
}
