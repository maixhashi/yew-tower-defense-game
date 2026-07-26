mod defenses;

pub use defenses::{DefenseStats, DEFENSE_CATALOG};

pub fn defense_by_id(type_id: &str) -> Option<&'static DefenseStats> {
    DEFENSE_CATALOG.iter().find(|d| d.type_id == type_id)
}
