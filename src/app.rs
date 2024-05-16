use std::time::Duration;

use leptos::{leptos_dom::logging::console_log, logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use markdown::to_html;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use markdown;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

#[wasm_bindgen(inline_js = "
export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
")]
extern "C" {
    fn sleep(ms: u32) -> Promise;
}

pub async fn sleep_ms(ms: u32) {
    let promise = sleep(ms);
    let js_future = JsFuture::from(promise);
    js_future.await.unwrap();
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/write-right.css"/>

        <Title text="My site"/>

        <NavBar />

        <Router>
            <main class="flex flex-col overflow-hidden">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (html, set_html) = create_signal("".to_string());

    let input_ref = create_node_ref::<html::Div>();

    let handle_click = move |_e: ev::MouseEvent| {
        input_ref().unwrap().focus();
    };

    let handle_update = move |e: ev::KeyboardEvent| {
        let key = e.key();

        if key == "Enter" {};

        if key == "Backspace" {};

        set_html(markdown::to_html(&input_ref().unwrap().inner_text()));
    };

    view! {
        <div
          ref=input_ref
          on:keydown=handle_update
          contenteditable
          class="hidden"
        >
        </div>
        <div
          on:click=handle_click
          class="p-8"
          inner_html=html
        >
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div>About!!!</div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav class="flex gap-8 items-center p-4">
            <a class="font-logo text-xl" href="/" aria-label="Home">
                <TypeWriter text="Write Right".to_string() symbol='▄' interval=50 delay=1000 />
            </a>
        </nav>
    }
}

#[component]
fn TypeWriter(text: String, symbol: char, interval: u32, delay: u32) -> impl IntoView {
    let (content, set_content) = create_signal("".to_string());
    let text_chars: Vec<char> = text.chars().collect();
    let text_length = text.len();

    create_effect(move |_| {
        let text_chars = text_chars.clone();
        spawn_local(async move {
            sleep_ms(delay).await;
            for i in 0..text_length {
                set_content(text_chars[..=i].iter().collect());
                sleep_ms(interval).await;
            };
        })
    });

    view! {
        {move || content().to_string()}{symbol}
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
