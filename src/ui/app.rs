use yew::prelude::*;
use yewdux::prelude::*;

use super::store::{CommandHqStore, DefenseCard, UiSettingsStore};

#[function_component(App)]
pub fn app() -> Html {
    let (hq, hq_dispatch) = use_store::<CommandHqStore>();
    let (settings, settings_dispatch) = use_store::<UiSettingsStore>();

    let toggle_pause = hq_dispatch.reduce_mut_callback(|store| {
        store.is_paused = !store.is_paused;
    });

    let select_cannon = hq_dispatch.reduce_mut_callback(|store| {
        store.selected_card = Some(DefenseCard::Cannon);
    });
    let select_archer = hq_dispatch.reduce_mut_callback(|store| {
        store.selected_card = Some(DefenseCard::Archer);
    });
    let select_barricade = hq_dispatch.reduce_mut_callback(|store| {
        store.selected_card = Some(DefenseCard::Barricade);
    });

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
            <p>{ "ウトガルド城風・立体古城防衛タワーディフェンス（スキャフォールド）" }</p>

            <section class="command-hq">
                <h2>{ "司令部" }</h2>
                <p>
                    { "状態: " }
                    { if hq.is_paused { "ポーズ中" } else { "進行中" } }
                </p>
                <p>{ format!("選択カード: {selected_label}") }</p>
                <p>
                    { "効果音: " }
                    { if settings.sound_muted { "ミュート" } else { "オン" } }
                    { "（LocalStorage 永続）" }
                </p>

                <div class="actions">
                    <button type="button" onclick={toggle_pause}>{ "ポーズ切替" }</button>
                    <button type="button" onclick={select_cannon}>{ "大砲" }</button>
                    <button type="button" onclick={select_archer}>{ "弓兵" }</button>
                    <button type="button" onclick={select_barricade}>{ "バリケード" }</button>
                    <button type="button" onclick={toggle_mute}>{ "ミュート切替" }</button>
                </div>
            </section>
        </main>
    }
}
