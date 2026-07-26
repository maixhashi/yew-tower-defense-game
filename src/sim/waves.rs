#[derive(Debug, Clone, Copy)]
pub struct WaveSpawn {
    pub type_id: &'static str,
    pub count: u32,
    pub interval: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WaveDef {
    pub spawns: &'static [WaveSpawn],
}

pub static WAVES: &[WaveDef] = &[
    WaveDef {
        spawns: &[WaveSpawn {
            type_id: "grunt",
            count: 3,
            interval: 1.2,
        }],
    },
    WaveDef {
        spawns: &[
            WaveSpawn {
                type_id: "grunt",
                count: 3,
                interval: 1.0,
            },
            WaveSpawn {
                type_id: "climber",
                count: 2,
                interval: 1.4,
            },
            WaveSpawn {
                type_id: "raider",
                count: 2,
                interval: 0.9,
            },
        ],
    },
    WaveDef {
        spawns: &[
            WaveSpawn {
                type_id: "armored",
                count: 2,
                interval: 1.6,
            },
            WaveSpawn {
                type_id: "climber",
                count: 3,
                interval: 0.9,
            },
            WaveSpawn {
                type_id: "grunt",
                count: 4,
                interval: 0.8,
            },
        ],
    },
    WaveDef {
        spawns: &[
            WaveSpawn {
                type_id: "raider",
                count: 4,
                interval: 0.7,
            },
            WaveSpawn {
                type_id: "armored",
                count: 2,
                interval: 1.5,
            },
            WaveSpawn {
                type_id: "sieger",
                count: 1,
                interval: 2.0,
            },
        ],
    },
];
