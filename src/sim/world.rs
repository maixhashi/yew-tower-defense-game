use std::collections::HashMap;

use super::catalog;
use super::command::Command;
use super::entity::{Enemy, EntityId, Projectile, Tower};
use super::event::GameEvent;
use super::exterior::{advance_along_path, try_place_tower, PlaceError, EXTERIOR_WAYPOINTS};
use super::snapshot::{EnemySnap, FrameSnapshot, ProjectileSnap, TowerSnap};

/// シミュレーション状態の単一所有者。
#[derive(Debug)]
pub struct World {
    tick: u64,
    paused: bool,
    castle_hp: f32,
    resources: u32,
    wave: u32,
    next_id: EntityId,
    selected_type: Option<String>,
    towers: HashMap<EntityId, Tower>,
    enemies: HashMap<EntityId, Enemy>,
    projectiles: HashMap<EntityId, Projectile>,
    commands: Vec<Command>,
    events: Vec<GameEvent>,
    debug_spawned: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            tick: 0,
            paused: false,
            castle_hp: 100.0,
            resources: 100,
            wave: 0,
            next_id: 1,
            selected_type: None,
            towers: HashMap::new(),
            enemies: HashMap::new(),
            projectiles: HashMap::new(),
            commands: Vec::new(),
            events: Vec::new(),
            debug_spawned: false,
        }
    }

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn tick(&mut self, dt: f32) {
        self.apply_commands();
        if !self.debug_spawned {
            self.spawn_debug_enemies();
            self.debug_spawned = true;
        }
        if self.paused {
            return;
        }
        self.update_towers(dt);
        self.update_projectiles(dt);
        self.update_enemies(dt);
        self.tick = self.tick.saturating_add(1);
    }

    pub fn take_snapshot(&mut self) -> FrameSnapshot {
        let events = std::mem::take(&mut self.events);
        FrameSnapshot {
            tick: self.tick,
            paused: self.paused,
            castle_hp: self.castle_hp,
            resources: self.resources,
            wave: self.wave,
            towers: self
                .towers
                .values()
                .map(|t| TowerSnap {
                    id: t.id,
                    type_id: t.type_id.clone(),
                    visual_key: t.visual_key.clone(),
                    x: t.x,
                    y: t.y,
                    z: t.z,
                })
                .collect(),
            enemies: self
                .enemies
                .values()
                .map(|e| EnemySnap {
                    id: e.id,
                    type_id: e.type_id.clone(),
                    visual_key: e.visual_key.clone(),
                    x: e.x,
                    y: e.y,
                    z: e.z,
                    hp: e.hp,
                })
                .collect(),
            projectiles: self
                .projectiles
                .values()
                .map(|p| ProjectileSnap {
                    id: p.id,
                    x: p.x,
                    y: p.y,
                    z: p.z,
                })
                .collect(),
            events,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    pub fn resources(&self) -> u32 {
        self.resources
    }

    pub fn castle_hp_value(&self) -> f32 {
        self.castle_hp
    }

    fn alloc_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    fn spawn_debug_enemies(&mut self) {
        for (i, type_id) in ["grunt", "climber", "grunt"].iter().enumerate() {
            let Some(stats) = catalog::enemy_by_id(type_id) else {
                continue;
            };
            let id = self.alloc_id();
            let start = EXTERIOR_WAYPOINTS[0];
            self.enemies.insert(
                id,
                Enemy {
                    id,
                    type_id: stats.type_id.into(),
                    visual_key: stats.visual_key.into(),
                    x: start.x + i as f32 * 0.8,
                    y: start.y,
                    z: start.z,
                    hp: stats.hp,
                    speed: stats.speed,
                    phase: 0.0,
                    waypoint_index: 0,
                },
            );
        }
    }

    fn update_enemies(&mut self, dt: f32) {
        let mut breached = Vec::new();
        for enemy in self.enemies.values_mut() {
            let reached = advance_along_path(
                &mut enemy.waypoint_index,
                &mut enemy.x,
                &mut enemy.y,
                &mut enemy.z,
                enemy.speed,
                dt,
            );
            if reached {
                breached.push(enemy.id);
            }
        }
        for id in breached {
            let damage = self
                .enemies
                .get(&id)
                .and_then(|e| catalog::enemy_by_id(&e.type_id))
                .map(|s| s.breach_damage)
                .unwrap_or(5.0);
            self.castle_hp = (self.castle_hp - damage).max(0.0);
            self.events.push(GameEvent::Breach {
                damage,
                enemy_id: id,
            });
            self.enemies.remove(&id);
            if self.castle_hp <= 0.0 {
                self.events.push(GameEvent::MatchEnded { won: false });
                self.paused = true;
            }
        }
    }

    fn cell_occupied(&self, cell_x: i32, cell_z: i32) -> bool {
        self.towers
            .values()
            .any(|t| t.cell_x == cell_x && t.cell_z == cell_z)
    }

    fn update_towers(&mut self, dt: f32) {
        let enemy_positions: Vec<(EntityId, f32, f32, f32)> = self
            .enemies
            .values()
            .map(|e| (e.id, e.x, e.y, e.z))
            .collect();
        let mut shots = Vec::new();
        for tower in self.towers.values_mut() {
            let Some(stats) = catalog::defense_by_id(&tower.type_id) else {
                continue;
            };
            if stats.damage <= 0.0 {
                continue;
            }
            tower.cooldown = (tower.cooldown - dt).max(0.0);
            if tower.cooldown > 0.0 {
                continue;
            }
            let mut best: Option<(EntityId, f32)> = None;
            for &(id, x, y, z) in &enemy_positions {
                let dx = x - tower.x;
                let dy = y - tower.y;
                let dz = z - tower.z;
                let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                if dist <= stats.range && best.map(|(_, d)| dist < d).unwrap_or(true) {
                    best = Some((id, dist));
                }
            }
            if let Some((target_id, _)) = best {
                tower.cooldown = stats.cooldown;
                shots.push((tower.x, tower.y + 0.8, tower.z, target_id, stats.damage));
            }
        }
        for (x, y, z, target_id, damage) in shots {
            let id = self.alloc_id();
            self.projectiles.insert(
                id,
                Projectile {
                    id,
                    target_id,
                    x,
                    y,
                    z,
                    speed: 14.0,
                    damage,
                },
            );
        }
    }

    fn update_projectiles(&mut self, dt: f32) {
        let mut hit = Vec::new();
        let mut orphan = Vec::new();
        for proj in self.projectiles.values_mut() {
            let Some(target) = self.enemies.get(&proj.target_id) else {
                orphan.push(proj.id);
                continue;
            };
            let dx = target.x - proj.x;
            let dy = target.y - proj.y;
            let dz = target.z - proj.z;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            let step = proj.speed * dt;
            if dist <= step || dist < 0.15 {
                hit.push((proj.id, proj.target_id, proj.damage));
            } else {
                let inv = step / dist;
                proj.x += dx * inv;
                proj.y += dy * inv;
                proj.z += dz * inv;
            }
        }
        for id in orphan {
            self.projectiles.remove(&id);
        }
        for (proj_id, enemy_id, damage) in hit {
            self.projectiles.remove(&proj_id);
            if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
                enemy.hp -= damage;
                if enemy.hp <= 0.0 {
                    let reward = 8;
                    self.enemies.remove(&enemy_id);
                    self.resources = self.resources.saturating_add(reward);
                    self.events.push(GameEvent::EnemyKilled {
                        enemy_id,
                        reward,
                    });
                }
            }
        }
    }

    fn apply_commands(&mut self) {
        let commands = std::mem::take(&mut self.commands);
        for command in commands {
            match command {
                Command::TogglePause => {
                    self.paused = !self.paused;
                    self.events
                        .push(GameEvent::PauseChanged { paused: self.paused });
                }
                Command::SetPaused { paused } => {
                    if self.paused != paused {
                        self.paused = paused;
                        self.events.push(GameEvent::PauseChanged { paused });
                    }
                }
                Command::SelectCard { card } => {
                    let type_id = match card.as_str() {
                        "cannon" | "Cannon" | "大砲" => "cannon",
                        "archer" | "Archer" | "弓兵" => "archer",
                        "barricade" | "Barricade" | "バリケード" => "barricade",
                        other => other,
                    };
                    if catalog::defense_by_id(type_id).is_some() {
                        self.selected_type = Some(type_id.into());
                    }
                }
                Command::PlaceTower {
                    type_id,
                    cell_x,
                    cell_z,
                } => {
                    let chosen = if type_id.is_empty() {
                        self.selected_type.clone().unwrap_or_default()
                    } else {
                        type_id
                    };
                    if self.cell_occupied(cell_x, cell_z) {
                        continue;
                    }
                    match try_place_tower(
                        &mut self.next_id,
                        &mut self.resources,
                        &mut |_, _| false,
                        &chosen,
                        cell_x,
                        cell_z,
                    ) {
                        Ok(tower) => {
                            self.towers.insert(tower.id, tower);
                        }
                        Err(PlaceError::NotEnoughGold)
                        | Err(PlaceError::InvalidCell)
                        | Err(PlaceError::Occupied)
                        | Err(PlaceError::UnknownType) => {}
                    }
                }
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_not_paused_tick_advances() {
        let mut world = World::new();
        world.tick(1.0 / 60.0);
        world.tick(1.0 / 60.0);
        assert_eq!(world.current_tick(), 2);
    }

    #[test]
    fn when_paused_via_command_tick_does_not_advance() {
        let mut world = World::new();
        world.push_command(Command::SetPaused { paused: true });
        world.tick(1.0 / 60.0);
        assert!(world.is_paused());
        assert_eq!(world.current_tick(), 0);
    }

    #[test]
    fn when_gold_is_enough_tower_is_placed() {
        let mut world = World::new();
        world.push_command(Command::PlaceTower {
            type_id: "cannon".into(),
            cell_x: 4,
            cell_z: 0,
        });
        world.tick(0.0);
        assert_eq!(world.towers.len(), 1);
        assert_eq!(world.resources(), 60);
    }

    #[test]
    fn when_gold_is_not_enough_tower_is_not_placed() {
        let mut world = World::new();
        world.resources = 10;
        world.push_command(Command::PlaceTower {
            type_id: "cannon".into(),
            cell_x: 4,
            cell_z: 0,
        });
        world.tick(0.0);
        assert!(world.towers.is_empty());
        assert_eq!(world.resources(), 10);
    }

    #[test]
    fn when_enemies_reach_end_castle_hp_decreases() {
        let mut world = World::new();
        world.tick(0.0);
        assert_eq!(world.enemies.len(), 3);
        for _ in 0..10_000 {
            world.tick(0.05);
            if world.enemies.is_empty() {
                break;
            }
        }
        assert!(world.castle_hp_value() < 100.0);
    }

    #[test]
    fn when_tower_is_in_range_enemy_can_be_killed() {
        let mut world = World::new();
        world.debug_spawned = true;
        world.enemies.clear();
        world.enemies.insert(
            99,
            Enemy {
                id: 99,
                type_id: "grunt".into(),
                visual_key: "enemy_box".into(),
                x: 8.0,
                y: 0.6,
                z: 0.0,
                hp: 5.0,
                speed: 0.0,
                phase: 0.0,
                waypoint_index: 0,
            },
        );
        world.push_command(Command::PlaceTower {
            type_id: "cannon".into(),
            cell_x: 4,
            cell_z: 0,
        });
        world.tick(0.0);
        for _ in 0..500 {
            world.tick(0.05);
            if world.enemies.is_empty() {
                break;
            }
        }
        assert!(world.enemies.is_empty());
        assert!(world.resources() > 60);
    }
}
