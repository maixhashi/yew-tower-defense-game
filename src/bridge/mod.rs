//! Wasm 境界: Command 投入と FrameSnapshot 取り出し。
//! `World` はここが単一所有（JS 窓口のみ `RefCell`）。

use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit};

use crate::sim::{Command, FrameSnapshot, World};

thread_local! {
    static WORLD: RefCell<Option<World>> = const { RefCell::new(None) };
    static LOOP_STARTED: RefCell<bool> = const { RefCell::new(false) };
    static RAF_CALLBACK: RefCell<Option<Closure<dyn FnMut(f64)>>> = const { RefCell::new(None) };
    static LAST_MS: RefCell<Option<f64>> = const { RefCell::new(None) };
}

fn ensure_world() {
    WORLD.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(World::new());
        }
    });
}

fn with_world<R>(f: impl FnOnce(&World) -> R) -> R {
    ensure_world();
    WORLD.with(|slot| f(slot.borrow().as_ref().expect("world")))
}

fn with_world_mut<R>(f: impl FnOnce(&mut World) -> R) -> R {
    ensure_world();
    WORLD.with(|slot| f(slot.borrow_mut().as_mut().expect("world")))
}

/// 組み立て入口（薄い Service Locator 相当の初期化）。
#[wasm_bindgen(js_name = gameInit)]
pub fn game_init() {
    ensure_world();
    start_game_loop();
}

#[wasm_bindgen(js_name = gamePushCommandJson)]
pub fn game_push_command_json(json: &str) -> Result<(), JsValue> {
    let command: Command = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&format!("invalid command: {e}")))?;
    with_world_mut(|world| world.push_command(command));
    Ok(())
}

pub fn push_command(command: Command) {
    with_world_mut(|world| world.push_command(command));
}

#[wasm_bindgen(js_name = gameTick)]
pub fn game_tick(dt: f32) {
    with_world_mut(|world| world.tick(dt));
}

#[wasm_bindgen(js_name = gameTakeSnapshotJson)]
pub fn game_take_snapshot_json() -> String {
    with_world_mut(|world| {
        let snap: FrameSnapshot = world.take_snapshot();
        serde_json::to_string(&snap).unwrap_or_else(|_| "{}".into())
    })
}

pub fn peek_paused() -> bool {
    with_world(|world| world.is_paused())
}

pub fn peek_tick() -> u64 {
    with_world(|world| world.current_tick())
}

fn emit_snapshot(json: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let init = CustomEventInit::new();
    init.set_detail(&JsValue::from_str(json));
    if let Ok(event) = CustomEvent::new_with_event_init_dict("td-snapshot", &init) {
        let _ = window.dispatch_event(&event);
    }
}

fn on_animation_frame(now_ms: f64) {
    let dt = LAST_MS.with(|last| {
        let prev = *last.borrow();
        *last.borrow_mut() = Some(now_ms);
        match prev {
            Some(p) => ((now_ms - p) / 1000.0).clamp(0.0, 0.05) as f32,
            None => 1.0 / 60.0,
        }
    });
    game_tick(dt);
    let json = game_take_snapshot_json();
    emit_snapshot(&json);
    schedule_next_frame();
}

fn schedule_next_frame() {
    let Some(window) = web_sys::window() else {
        return;
    };
    RAF_CALLBACK.with(|slot| {
        if let Some(cb) = slot.borrow().as_ref() {
            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        }
    });
}

fn start_game_loop() {
    let already = LOOP_STARTED.with(|f| {
        let started = *f.borrow();
        if !started {
            *f.borrow_mut() = true;
        }
        started
    });
    if already {
        return;
    }

    let callback = Closure::<dyn FnMut(f64)>::new(on_animation_frame);
    RAF_CALLBACK.with(|slot| {
        *slot.borrow_mut() = Some(callback);
    });
    schedule_next_frame();
}
