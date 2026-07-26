mod defenses;
mod enemies;

pub use defenses::{DefenseStats, DEFENSE_CATALOG};
pub use enemies::{EnemyStats, ENEMY_CATALOG};

pub fn defense_by_id(type_id: &str) -> Option<&'static DefenseStats> {
    DEFENSE_CATALOG.iter().find(|d| d.type_id == type_id)
}

pub fn enemy_by_id(type_id: &str) -> Option<&'static EnemyStats> {
    enemies::enemy_by_id(type_id)
}
