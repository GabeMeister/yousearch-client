mod utils;

use gloo_console::{__macro::JsValue, log};
use gloo_net::http::Request;
use std::env;
use utils::inputs;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const RUST_ENV: &str = env!("RUST_ENV");
const BACKEND_API: &str = env!("BACKEND_API");

#[function_component(YouSearch)]
fn yousearch_component() -> Html {
    let header_text = use_state(|| "This is the Default Header".to_string());

    let on_btn_click = {
        let header_text = header_text.clone();

        Callback::from(move |_| {
            let header_text = header_text.clone();

            spawn_local(async move {
                let url = "https://gabemeister-rocket-rust-hello-world.onrender.com";
                let data = Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                log!(&data);

                header_text.set(data);
            });
        })
    };

    let on_btn_click2 = {
        let header_text = header_text.clone();

        Callback::from(move |_| {
            let header_text = header_text.clone();

            spawn_local(async move {
                let url = "https://gabemeister-rocket-rust-hello-world.onrender.com";
                let data = Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                log!(&data);

                header_text.set(data.to_uppercase());
            });
        })
    };

    html! {
        <div>
            <h1 class="text-5xl font-bold">{&*header_text}</h1>
            <button
                class="
                    border-black
                    border-2
                    rounded-md
                    p-1
                    px-3
                    mt-3
                    bg-blue-300
                    hover:bg-blue-400
                    block
                "
                onclick={on_btn_click.clone()}
            >{"Update Header Text"}</button>
            <button
                class="
                    border-black
                    border-2
                    rounded-md
                    p-1
                    px-3
                    mt-3
                    bg-green-300
                    hover:bg-green-400
                    block
                "
                onclick={on_btn_click2.clone()}
            >{"Update Header Text to all Caps"}</button>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="p-6">
            <YouSearch />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
