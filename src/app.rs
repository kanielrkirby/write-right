use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::{
    typewriter::TypeWriter,
    editor::Editor,

};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/write-right.css"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.22.0/prism.min.js" />

        <Script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.22.0/plugins/autoloader/prism-autoloader.min.js" />
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/prism/9000.0.1/themes/prism-tomorrow.min.css" />
        <Script>
        "
            window.Prism = window.Prism || {};
        "
        </Script>

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
    view! {
        <Editor />
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
