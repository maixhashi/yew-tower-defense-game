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
                count: 4,
                interval: 1.0,
            },
            WaveSpawn {
                type_id: "climber",
                count: 2,
                interval: 1.4,
            },
        ],
    },
    WaveDef {
        spawns: &[
            WaveSpawn {
                type_id: "climber",
                count: 4,
                interval: 0.9,
            },
            WaveSpawn {
                type_id: "grunt",
                count: 5,
                interval: 0.8,
            },
        ],
    },
];
