mod utils;

use gloo_console::{__macro::JsValue, log};
use gloo_net::http::Request;
use gloo_net::Error;
use std::env;
use utils::inputs;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// use yew::suspense::{Suspension, SuspensionResult};

const RUST_ENV: &str = env!("RUST_ENV");
const BACKEND_API: &str = env!("BACKEND_API");

// #[derive(Debug)]
// struct User {
//     id: i32,
//     name: String,
// }

// async fn load_user() -> Result<String, Error> {
//     let url = format!("{BACKEND_API}/user/1");
//     let response = Request::get(&url).send().await?;
//     let text = response.text().await;

//     text
// }

// fn on_load_user_complete<F: FnOnce()>(_fn: F) {
//     todo!() // implementation omitted.
// }

// async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     Ok(text)
// }

// #[hook]
// fn use_user() -> SuspensionResult<User> {
//     match load_user() {
//         // If a user is loaded, then we return it as Ok(user).
//         Some(m) => Ok(m),
//         None => {
//             // When user is still loading, then we create a `Suspension`
//             // and call `SuspensionHandle::resume` when data loading
//             // completes, the component will be re-rendered
//             // automatically.
//             let (s, handle) = Suspension::new();
//             on_load_user_complete(move || {
//                 handle.resume();
//             });
//             Err(s)
//         }
//     }
// }

// #[function_component(Content)]
// fn content() -> HtmlResult {
//     let user = use_user()?;

//     Ok(html! {<div>{"Hello, "}{&user.name}</div>})
// }

// https://gabemeister-rocket-rust-hello-world.onrender.com
// https://ron-swanson-quotes.herokuapp.com/v2/quotes

#[function_component(YouSearch)]
fn yousearch_component() -> Html {
    let header_text = use_state(|| "This is the Default Header".to_string());
    let header_text_clone = header_text.clone();

    let on_btn_click = {
        Callback::from(move |_| {
            let header_text_clone2 = header_text_clone.clone();

            spawn_local(async move {
                let url = "https://gabemeister-rocket-rust-hello-world.onrender.com";
                let data = Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                header_text_clone2.set(data);
            });
        })
    };

    html! {
        <div>
            <button
                class="
                    border-black
                    border-2
                    rounded-md
                    p-1
                    px-3
                    ml-8
                    bg-blue-300
                    hover:bg-blue-400
                "
                onclick={on_btn_click.clone()}
            >{"Update Header Text"}</button>
            <h1 class="text-5xl font-bold">{&*header_text}</h1>
            //<Suspense {fallback}>
            //    <Content />
            //</Suspense>
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
