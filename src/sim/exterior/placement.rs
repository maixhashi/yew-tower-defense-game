use crate::sim::catalog::defense_by_id;
use crate::sim::entity::{EntityId, Tower};

pub const CELL_SIZE: f32 = 2.0;

pub fn cell_to_world(cell_x: i32, cell_z: i32) -> (f32, f32, f32) {
    (
        cell_x as f32 * CELL_SIZE,
        0.8,
        cell_z as f32 * CELL_SIZE,
    )
}

pub fn is_buildable_cell(cell_x: i32, cell_z: i32) -> bool {
    // 城壁外周リング（内側 keep 付近は不可）
    let ax = cell_x.abs();
    let az = cell_z.abs();
    (ax == 4 || az == 4) && ax <= 4 && az <= 4
}

pub fn try_place_tower(
    next_id: &mut EntityId,
    resources: &mut u32,
    occupied: &mut impl FnMut(i32, i32) -> bool,
    type_id: &str,
    cell_x: i32,
    cell_z: i32,
) -> Result<Tower, PlaceError> {
    let stats = defense_by_id(type_id).ok_or(PlaceError::UnknownType)?;
    if !is_buildable_cell(cell_x, cell_z) {
        return Err(PlaceError::InvalidCell);
    }
    if occupied(cell_x, cell_z) {
        return Err(PlaceError::Occupied);
    }
    if *resources < stats.cost {
        return Err(PlaceError::NotEnoughGold);
    }
    *resources -= stats.cost;
    let id = *next_id;
    *next_id = next_id.saturating_add(1);
    let (x, y, z) = cell_to_world(cell_x, cell_z);
    Ok(Tower {
        id,
        type_id: stats.type_id.into(),
        visual_key: stats.visual_key.into(),
        cell_x,
        cell_z,
        x,
        y,
        z,
        cooldown: 0.0,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceError {
    UnknownType,
    InvalidCell,
    Occupied,
    NotEnoughGold,
}
