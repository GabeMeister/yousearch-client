use gloo_console::{__macro::JsValue, log};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

async fn print_response(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = Request::get(url).send().await?;
    let text = response.text().await?;
    log!("{}", JsValue::from(text));
    Ok(())
}

#[function_component(YouSearch)]
fn gabe_state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    // https://gabemeister-yousearch.shuttleapp.rs/hello
    // https://ron-swanson-quotes.herokuapp.com/v2/quotes
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                // let _ = print_response("https://ron-swanson-quotes.herokuapp.com/v2/quotes").await;
                let _ = print_response("https://gabemeister-yousearch.shuttleapp.rs/hello").await;
                // let _ = print_response("http://127.0.0.1:8000/hello").await;
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
            <h1>{ "Welcome to YouSearch!" }</h1>
            <YouSearch />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
