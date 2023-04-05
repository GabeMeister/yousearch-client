mod utils;

use gloo_console::log;
use gloo_net::http::Request;
use std::env;
use utils::inputs;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
// use gloo_console::{__macro::JsValue, log};
// use gloo_net::http::Request;
// use gloo_net::Error;

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
    // let fallback = html! {<div>{"Loading..."}</div>};
    let counter = use_state(|| 0);
    let random_text = use_state(|| "default text".to_string());
    let header_text = use_state(|| "This is the Default Header".to_string());

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let on_text_change = {
        let random_text_clone = random_text.clone();

        Callback::from(move |event: KeyboardEvent| {
            let value = inputs::get_keyboard_input_value(event);
            random_text_clone.set(value);
        })
    };

    let on_btn_click = {
        let header_text_clone = header_text.clone();

        async fn my_async_fn() -> String {
            let res = Request::get("https://gabemeister-rocket-rust-hello-world.onrender.com")
                .send()
                .await
                .unwrap();

            res.text().await.unwrap()
        }

        Callback::from(move |_| {
            spawn_local(async {
                let data = my_async_fn().await;

                // TODO: figure out how to set state here
                log!(&data);
            });
        })
    };

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
            <input
                type="text"
                placeholder="Enter something"
                class="border-2 border-gray-500 rounded-lg py-2 px-4"
                onkeyup={on_text_change}
            />
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
                onclick={on_btn_click}
            >{"Update Header Text"}</button>
            <p>{&*random_text}</p>
            <p>{&*random_text}</p>
            <p>{&*random_text}</p>
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
