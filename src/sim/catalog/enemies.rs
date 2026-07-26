#[derive(Debug, Clone, Copy)]
pub struct EnemyStats {
    pub type_id: &'static str,
    pub visual_key: &'static str,
    pub hp: f32,
    pub speed: f32,
    pub breach_damage: f32,
}

pub static ENEMY_CATALOG: &[EnemyStats] = &[
    EnemyStats {
        type_id: "grunt",
        visual_key: "enemy_box",
        hp: 20.0,
        speed: 2.2,
        breach_damage: 5.0,
    },
    EnemyStats {
        type_id: "climber",
        visual_key: "enemy_climber",
        hp: 14.0,
        speed: 3.0,
        breach_damage: 4.0,
    },
    EnemyStats {
        type_id: "armored",
        visual_key: "enemy_armored",
        hp: 45.0,
        speed: 1.4,
        breach_damage: 8.0,
    },
    EnemyStats {
        type_id: "raider",
        visual_key: "enemy_raider",
        hp: 12.0,
        speed: 4.2,
        breach_damage: 3.0,
    },
    EnemyStats {
        type_id: "sieger",
        visual_key: "enemy_sieger",
        hp: 60.0,
        speed: 1.1,
        breach_damage: 15.0,
    },
];

pub fn enemy_by_id(type_id: &str) -> Option<&'static EnemyStats> {
    ENEMY_CATALOG.iter().find(|e| e.type_id == type_id)
}
