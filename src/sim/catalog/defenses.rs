//! Type Object 表（防衛）。インスタンスは type_id だけ持ち、ここを参照する。

#[derive(Debug, Clone, Copy)]
pub struct DefenseStats {
    pub type_id: &'static str,
    pub visual_key: &'static str,
    pub cost: u32,
    pub damage: f32,
    pub range: f32,
    pub cooldown: f32,
    pub blocks_path: bool,
}

pub static DEFENSE_CATALOG: &[DefenseStats] = &[
    DefenseStats {
        type_id: "cannon",
        visual_key: "tower_cannon",
        cost: 40,
        damage: 8.0,
        range: 6.0,
        cooldown: 1.2,
        blocks_path: false,
    },
    DefenseStats {
        type_id: "archer",
        visual_key: "tower_archer",
        cost: 25,
        damage: 3.0,
        range: 8.0,
        cooldown: 0.45,
        blocks_path: false,
    },
    DefenseStats {
        type_id: "barricade",
        visual_key: "tower_barricade",
        cost: 15,
        damage: 0.0,
        range: 0.0,
        cooldown: 0.0,
        blocks_path: true,
    },
    DefenseStats {
        type_id: "frost_archer",
        visual_key: "tower_frost_archer",
        cost: 35,
        damage: 2.5,
        range: 9.0,
        cooldown: 0.55,
        blocks_path: false,
    },
    DefenseStats {
        type_id: "mortar",
        visual_key: "tower_mortar",
        cost: 55,
        damage: 14.0,
        range: 10.0,
        cooldown: 2.0,
        blocks_path: false,
    },
];
