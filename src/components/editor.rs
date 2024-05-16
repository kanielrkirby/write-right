use leptos::{component, create_node_ref, create_signal, ev, html, view, IntoView};

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
    };

    view! {
        <div
          ref=input_ref
          on:keydown=handle_keydown
          class="p-8 outline-pink-500"
          contenteditable
          inner_text=text
          on:input=handle_input
        >
        </div>
        <div inner_html=html />
    }
}
