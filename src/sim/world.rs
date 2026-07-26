use std::collections::HashMap;

use super::catalog;
use super::command::Command;
use super::entity::{Enemy, EntityId, Projectile, Tower};
use super::event::GameEvent;
use super::exterior::{advance_along_path, try_place_tower, PlaceError, EXTERIOR_WAYPOINTS};
use super::match_state::MatchState;
use super::snapshot::{EnemySnap, FrameSnapshot, ProjectileSnap, TowerSnap};
use super::waves::WAVES;

/// シミュレーション状態の単一所有者。
#[derive(Debug)]
pub struct World {
    tick: u64,
    paused: bool,
    match_state: MatchState,
    castle_hp: f32,
    resources: u32,
    /// 1-based の現在ウェーブ。0 は開始前。
    wave: u32,
    spawn_group_index: usize,
    spawns_left_in_group: u32,
    spawn_timer: f32,
    wave_spawn_finished: bool,
    next_id: EntityId,
    selected_type: Option<String>,
    towers: HashMap<EntityId, Tower>,
    enemies: HashMap<EntityId, Enemy>,
    projectiles: HashMap<EntityId, Projectile>,
    commands: Vec<Command>,
    events: Vec<GameEvent>,
    /// テストで手動スポーンするとき false。
    auto_waves: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            tick: 0,
            paused: false,
            match_state: MatchState::Playing,
            castle_hp: 100.0,
            resources: 100,
            wave: 0,
            spawn_group_index: 0,
            spawns_left_in_group: 0,
            spawn_timer: 0.0,
            wave_spawn_finished: true,
            next_id: 1,
            selected_type: None,
            towers: HashMap::new(),
            enemies: HashMap::new(),
            projectiles: HashMap::new(),
            commands: Vec::new(),
            events: Vec::new(),
            auto_waves: true,
        }
    }

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn tick(&mut self, dt: f32) {
        self.apply_commands();
        if matches!(self.match_state, MatchState::Won | MatchState::Lost) {
            return;
        }
        if self.paused || self.match_state == MatchState::Paused {
            return;
        }
        if self.auto_waves {
            if self.wave == 0 {
                self.begin_wave(1);
            }
            self.update_wave_spawns(dt);
        }
        self.update_towers(dt);
        self.update_projectiles(dt);
        self.update_enemies(dt);
        if self.auto_waves {
            self.check_wave_cleared();
        }
        self.tick = self.tick.saturating_add(1);
    }

    pub fn take_snapshot(&mut self) -> FrameSnapshot {
        let events = std::mem::take(&mut self.events);
        FrameSnapshot {
            tick: self.tick,
            paused: self.paused,
            match_state: self.match_state,
            castle_hp: self.castle_hp,
            resources: self.resources,
            wave: self.wave,
            total_waves: WAVES.len() as u32,
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

    pub fn wave(&self) -> u32 {
        self.wave
    }

    pub fn total_waves(&self) -> u32 {
        WAVES.len() as u32
    }

    pub fn match_state(&self) -> MatchState {
        self.match_state
    }

    fn alloc_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    fn begin_wave(&mut self, wave_num: u32) {
        let Some(def) = WAVES.get(wave_num as usize - 1) else {
            return;
        };
        self.wave = wave_num;
        self.spawn_group_index = 0;
        self.spawn_timer = 0.0;
        self.wave_spawn_finished = def.spawns.is_empty();
        self.spawns_left_in_group = def
            .spawns
            .first()
            .map(|s| s.count)
            .unwrap_or(0);
    }

    fn update_wave_spawns(&mut self, dt: f32) {
        if self.wave_spawn_finished || self.wave == 0 {
            return;
        }
        let Some(def) = WAVES.get(self.wave as usize - 1) else {
            self.wave_spawn_finished = true;
            return;
        };
        if self.spawn_group_index >= def.spawns.len() {
            self.wave_spawn_finished = true;
            return;
        }

        self.spawn_timer -= dt;
        while self.spawn_timer <= 0.0 {
            let spawn = def.spawns[self.spawn_group_index];
            if self.spawns_left_in_group > 0 {
                self.spawn_enemy(spawn.type_id);
                self.spawns_left_in_group -= 1;
                self.spawn_timer += spawn.interval;
            }
            if self.spawns_left_in_group == 0 {
                self.spawn_group_index += 1;
                if self.spawn_group_index >= def.spawns.len() {
                    self.wave_spawn_finished = true;
                    break;
                }
                let next = def.spawns[self.spawn_group_index];
                self.spawns_left_in_group = next.count;
                self.spawn_timer = 0.0;
            }
            if self.wave_spawn_finished {
                break;
            }
            // 無限ループ防止（interval が 0 の異常データ）
            if spawn.interval <= 0.0 && self.spawns_left_in_group > 0 {
                break;
            }
        }
    }

    fn spawn_enemy(&mut self, type_id: &str) {
        let Some(stats) = catalog::enemy_by_id(type_id) else {
            return;
        };
        let id = self.alloc_id();
        let start = EXTERIOR_WAYPOINTS[0];
        let offset = (id % 5) as f32 * 0.35;
        self.enemies.insert(
            id,
            Enemy {
                id,
                type_id: stats.type_id.into(),
                visual_key: stats.visual_key.into(),
                x: start.x + offset,
                y: start.y,
                z: start.z,
                hp: stats.hp,
                speed: stats.speed,
                phase: 0.0,
                waypoint_index: 0,
            },
        );
    }

    fn check_wave_cleared(&mut self) {
        if matches!(self.match_state, MatchState::Won | MatchState::Lost) {
            return;
        }
        if !self.wave_spawn_finished || !self.enemies.is_empty() || self.wave == 0 {
            return;
        }
        let cleared = self.wave;
        self.events.push(GameEvent::WaveCleared { wave: cleared });
        if (cleared as usize) >= WAVES.len() {
            self.match_state = MatchState::Won;
            self.paused = true;
            self.events.push(GameEvent::MatchEnded { won: true });
        } else {
            self.begin_wave(cleared + 1);
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
                self.match_state = MatchState::Lost;
                self.paused = true;
                self.events.push(GameEvent::MatchEnded { won: false });
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
                    if matches!(self.match_state, MatchState::Won | MatchState::Lost) {
                        continue;
                    }
                    self.paused = !self.paused;
                    self.match_state = if self.paused {
                        MatchState::Paused
                    } else {
                        MatchState::Playing
                    };
                    self.events
                        .push(GameEvent::PauseChanged { paused: self.paused });
                }
                Command::SetPaused { paused } => {
                    if matches!(self.match_state, MatchState::Won | MatchState::Lost) {
                        continue;
                    }
                    if self.paused != paused {
                        self.paused = paused;
                        self.match_state = if paused {
                            MatchState::Paused
                        } else {
                            MatchState::Playing
                        };
                        self.events.push(GameEvent::PauseChanged { paused });
                    }
                }
                Command::SelectCard { card } => {
                    let type_id = match card.as_str() {
                        "cannon" | "Cannon" | "大砲" => "cannon",
                        "archer" | "Archer" | "弓兵" => "archer",
                        "barricade" | "Barricade" | "バリケード" => "barricade",
                        "frost_archer" | "FrostArcher" | "氷弓" => "frost_archer",
                        "mortar" | "Mortar" | "迫撃砲" => "mortar",
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

    /// テスト用: ウェーブ自動スポーンを止める。
    #[cfg(test)]
    fn suppress_wave_spawns(&mut self) {
        self.auto_waves = false;
        self.wave_spawn_finished = true;
        self.spawns_left_in_group = 0;
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
        world.suppress_wave_spawns();
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
        assert_eq!(world.match_state(), MatchState::Paused);
        assert_eq!(world.current_tick(), 0);
    }

    #[test]
    fn when_gold_is_enough_tower_is_placed() {
        let mut world = World::new();
        world.suppress_wave_spawns();
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
        world.suppress_wave_spawns();
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
    fn when_wave_starts_enemies_spawn_from_waves() {
        let mut world = World::new();
        world.tick(0.0);
        assert_eq!(world.wave(), 1);
        assert_eq!(world.enemies.len(), 1);
        world.tick(1.2);
        assert!(world.enemies.len() >= 2);
    }

    #[test]
    fn when_enemies_reach_end_castle_hp_decreases() {
        let mut world = World::new();
        world.tick(0.0);
        assert!(!world.enemies.is_empty());
        for _ in 0..10_000 {
            world.tick(0.05);
            if world.enemies.is_empty() && world.wave_spawn_finished {
                // 次ウェーブが始まる前に HP を確認したいので、全滅かつスポーン完了を待つ
                if world.match_state() == MatchState::Lost
                    || world.castle_hp_value() < 100.0
                {
                    break;
                }
            }
            if world.castle_hp_value() < 100.0 {
                break;
            }
        }
        assert!(world.castle_hp_value() < 100.0);
    }

    #[test]
    fn when_tower_is_in_range_enemy_can_be_killed() {
        let mut world = World::new();
        world.suppress_wave_spawns();
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

    #[test]
    fn when_snapshot_taken_match_state_and_total_waves_are_included() {
        let mut world = World::new();
        let snap = world.take_snapshot();
        assert_eq!(snap.match_state, MatchState::Playing);
        assert_eq!(snap.total_waves, WAVES.len() as u32);
    }
}
