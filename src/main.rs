use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="app">
            <h1>{ "古城防衛戦" }</h1>
            <p>{ "ウトガルド城風・立体古城防衛タワーディフェンス（スキャフォールド）" }</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
