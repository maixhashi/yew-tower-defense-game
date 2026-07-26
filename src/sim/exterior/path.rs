//! 固定ウェイポイント列（立体 A* は後続）。

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// 外周 → 城壁よじ登り近似 → 突破点。
pub static EXTERIOR_WAYPOINTS: &[Vec3] = &[
    Vec3 {
        x: -12.0,
        y: 0.6,
        z: -12.0,
    },
    Vec3 {
        x: 0.0,
        y: 0.6,
        z: -12.0,
    },
    Vec3 {
        x: 0.0,
        y: 3.2,
        z: -8.0,
    },
    Vec3 {
        x: 0.0,
        y: 3.2,
        z: -4.0,
    },
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    },
];

pub fn advance_along_path(
    waypoint_index: &mut usize,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
    speed: f32,
    dt: f32,
) -> bool {
    advance_along_waypoints(
        EXTERIOR_WAYPOINTS,
        waypoint_index,
        x,
        y,
        z,
        speed,
        dt,
    )
}

pub fn advance_along_waypoints(
    waypoints: &[Vec3],
    waypoint_index: &mut usize,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
    speed: f32,
    dt: f32,
) -> bool {
    if *waypoint_index >= waypoints.len() {
        return true;
    }
    let target = waypoints[*waypoint_index];
    let dx = target.x - *x;
    let dy = target.y - *y;
    let dz = target.z - *z;
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
    let step = speed * dt;
    if dist <= step || dist < 1e-4 {
        *x = target.x;
        *y = target.y;
        *z = target.z;
        *waypoint_index += 1;
        return *waypoint_index >= waypoints.len();
    }
    let inv = step / dist;
    *x += dx * inv;
    *y += dy * inv;
    *z += dz * inv;
    false
}
