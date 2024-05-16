use std::time::Duration;

use leptos::{leptos_dom::logging::console_log, logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use markdown::to_html;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use markdown;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;
use crate::components::typewriter::TypeWriter;

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
            <a href="/" aria-label="Home">
                <TypeWriter class="text-xl" text="Write Right" />
            </a>
        </nav>
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
