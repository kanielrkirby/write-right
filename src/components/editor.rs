use leptos::{component, create_node_ref, create_signal, ev, html, view, IntoView};
use crate::utils::highlight::highlight_all;

#[component]
pub fn Editor() -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let html = leptos::create_memo(move |_| markdown::to_html(&text()));
    let input_ref = create_node_ref::<html::Div>();

    let handle_keydown = move |e: ev::KeyboardEvent| {
        let key = e.key();

        if key == "Enter" {};

        if key == "Backspace" {};
    };

    let handle_input = move |_| {
        set_text(input_ref().unwrap().inner_text());
        highlight_all();
    };

    view! {
        <div
          ref=input_ref
          on:keydown=handle_keydown
          class="p-8 text-left"
          contenteditable
          inner_text=text
          on:input=handle_input
        >
        </div>
        <div class="markdown-body" inner_html=html />
    }
}
