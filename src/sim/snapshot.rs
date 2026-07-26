use serde::{Deserialize, Serialize};

use super::event::GameEvent;

/// 描画 / UI 向けの所有スナップショット（World を外に貸さない）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameSnapshot {
    pub tick: u64,
    pub paused: bool,
    pub castle_hp: f32,
    pub resources: u32,
    pub wave: u32,
    pub towers: Vec<TowerSnap>,
    pub enemies: Vec<EnemySnap>,
    pub events: Vec<GameEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TowerSnap {
    pub id: u32,
    pub type_id: String,
    pub visual_key: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnemySnap {
    pub id: u32,
    pub type_id: String,
    pub visual_key: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hp: f32,
}
