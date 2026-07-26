//! エンティティはすべて所有データ + EntityId。相互参照フィールドは持たない。

pub type EntityId = u32;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: EntityId,
    pub type_id: String,
    pub visual_key: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hp: f32,
    pub speed: f32,
    /// デバッグ移動用の位相（ラジアン）。
    pub phase: f32,
}

#[derive(Debug, Clone)]
pub struct Tower {
    pub id: EntityId,
    pub type_id: String,
    pub visual_key: String,
    pub cell_x: i32,
    pub cell_z: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cooldown: f32,
}
