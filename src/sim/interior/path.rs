use crate::sim::exterior::path::Vec3;

/// 城内回廊の仮ウェイポイント（キープ中心へ）。
pub static INTERIOR_WAYPOINTS: &[Vec3] = &[
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: -2.5,
    },
    Vec3 {
        x: 2.0,
        y: 1.0,
        z: -1.0,
    },
    Vec3 {
        x: 1.0,
        y: 1.2,
        z: 1.5,
    },
    Vec3 {
        x: 0.0,
        y: 1.4,
        z: 0.0,
    },
];
