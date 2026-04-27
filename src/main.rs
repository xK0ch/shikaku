use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Hello, Shikaku!"</h1>
            <p>"Leptos läuft im Browser via WASM."</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
