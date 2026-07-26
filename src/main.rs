mod ui;

fn main() {
    let root = gloo_utils::document()
        .get_element_by_id("yew-root")
        .expect("yew-root element");
    yew::Renderer::<ui::App>::with_root(root).render();
}
