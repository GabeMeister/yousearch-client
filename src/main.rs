use gloo_console::{__macro::JsValue, log};
use gloo_net::http::Request;
use std::env;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const RUST_ENV: &str = env!("RUST_ENV");
const BACKEND_API: &str = env!("BACKEND_API");

async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = Request::get(url).send().await?;
    let text = response.text().await?;

    Ok(text)
}

#[function_component(YouSearch)]
fn gabe_state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    // https://gabemeister-rocket-rust-hello-world.onrender.com
    // https://ron-swanson-quotes.herokuapp.com/v2/quotes
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                // let result = fetch("https://ron-swanson-quotes.herokuapp.com/v2/quotes").await;
                let result = fetch(BACKEND_API).await;
                // let result =
                //     fetch("https://gabemeister-rocket-rust-hello-world.onrender.com/").await;

                match result {
                    Ok(text) => log!(JsValue::from(&text)),
                    Err(_) => log!("Failed to fetch"),
                }
            })
        },
        (),
    );

    html! {
        <div>
            <button
                class="border-2 border-black rounded-md py-1 px-2"
                {onclick}
            >
                {"Add +1"}
            </button>
            <p>
                <b class="text-blue-400">{ "Counter: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="p-6">
            <h1>{ BACKEND_API }</h1>
            <h1>{ RUST_ENV }</h1>
            <YouSearch />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
