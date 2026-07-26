pub mod bridge;
pub mod sim;
pub mod ui;

pub fn run_app() {
    let root = gloo_utils::document()
        .get_element_by_id("yew-root")
        .expect("yew-root element");
    yew::Renderer::<ui::App>::with_root(root).render();
    bridge::game_init();
}
