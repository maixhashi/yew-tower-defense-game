use yew::prelude::*;
use yewdux::prelude::*;

use crate::bridge;
use crate::sim::Command;

use super::store::{CommandHqStore, DefenseCard, UiSettingsStore};

#[function_component(App)]
pub fn app() -> Html {
    let (hq, hq_dispatch) = use_store::<CommandHqStore>();
    let (settings, settings_dispatch) = use_store::<UiSettingsStore>();
    let sim_tick = use_state(|| 0u64);
    let sim_paused = use_state(|| false);
    let sim_resources = use_state(|| 0u32);
    let sim_castle_hp = use_state(|| 100.0f32);

    {
        let sim_tick = sim_tick.clone();
        let sim_paused = sim_paused.clone();
        let sim_resources = sim_resources.clone();
        let sim_castle_hp = sim_castle_hp.clone();
        let hq_dispatch = hq_dispatch.clone();
        use_effect_with((), move |_| {
            let mut last = 0u64;
            let interval = gloo_timers::callback::Interval::new(200, move || {
                let tick = bridge::peek_tick();
                let paused = bridge::peek_paused();
                let resources = bridge::peek_resources();
                let castle_hp = bridge::peek_castle_hp();
                if tick != last {
                    last = tick;
                    sim_tick.set(tick);
                }
                sim_paused.set(paused);
                sim_resources.set(resources);
                sim_castle_hp.set(castle_hp);
                hq_dispatch.reduce_mut(|store| {
                    store.is_paused = paused;
                });
            });
            move || drop(interval)
        });
    }

    let toggle_pause = Callback::from(move |_| {
        bridge::push_command(Command::TogglePause);
    });

    let select_card = {
        let hq_dispatch = hq_dispatch.clone();
        Callback::from(move |card: DefenseCard| {
            hq_dispatch.reduce_mut(move |store| {
                store.selected_card = Some(card);
            });
            bridge::push_command(Command::SelectCard {
                card: card.type_id().into(),
            });
            if let Some(window) = web_sys::window() {
                let _ = js_sys::Reflect::set(
                    &window,
                    &wasm_bindgen::JsValue::from_str("__tdSelectedType"),
                    &wasm_bindgen::JsValue::from_str(card.type_id()),
                );
            }
        })
    };

    let place_demo = {
        let hq = hq.clone();
        Callback::from(move |_| {
            let type_id = hq
                .selected_card
                .map(DefenseCard::type_id)
                .unwrap_or("cannon")
                .to_string();
            bridge::push_command(Command::PlaceTower {
                type_id,
                cell_x: 4,
                cell_z: 0,
            });
        })
    };

    let toggle_mute = settings_dispatch.reduce_mut_callback(|store| {
        store.sound_muted = !store.sound_muted;
    });

    let selected_label = hq
        .selected_card
        .map(DefenseCard::label)
        .unwrap_or("未選択");

    html! {
        <main class="app">
            <h1>{ "古城防衛戦" }</h1>
            <p>{ "城壁リングをクリック／デモ配置で防衛を置く" }</p>

            <section class="command-hq">
                <h2>{ "司令部" }</h2>
                <p>
                    { "状態: " }
                    { if *sim_paused { "ポーズ中" } else { "進行中" } }
                    { format!(" / tick {}", *sim_tick) }
                </p>
                <p>{ format!("資源: {} / 城HP: {:.0}", *sim_resources, *sim_castle_hp) }</p>
                <p>{ format!("選択カード: {selected_label}") }</p>
                <p>
                    { "効果音: " }
                    { if settings.sound_muted { "ミュート" } else { "オン" } }
                </p>

                <div class="actions">
                    <button type="button" onclick={toggle_pause}>{ "ポーズ切替" }</button>
                    <button type="button" onclick={select_card.reform(|_| DefenseCard::Cannon)}>{ "大砲" }</button>
                    <button type="button" onclick={select_card.reform(|_| DefenseCard::Archer)}>{ "弓兵" }</button>
                    <button type="button" onclick={select_card.reform(|_| DefenseCard::Barricade)}>{ "バリケード" }</button>
                    <button type="button" onclick={place_demo}>{ "デモ配置(4,0)" }</button>
                    <button type="button" onclick={toggle_mute}>{ "ミュート切替" }</button>
                </div>
            </section>
        </main>
    }
}
