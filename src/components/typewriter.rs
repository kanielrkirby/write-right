use leptos::{component, create_effect, create_signal, view, IntoView};
use wasm_bindgen_futures::spawn_local;

use crate::utils::sleep::sleep_ms;

/// TypeWriter takes in a `text` string, and "types" it out in a thread. Example usage:
/// <TypeWriter text="Something cool" symbol='_' interval=20 delay=3000 />
#[component]
pub fn TypeWriter(
    #[prop(default = "")]
    text: &'static str,
    #[prop(default = '▄')]
    symbol: char, 
    #[prop(default = 50)]
    interval: u32, 
    #[prop(default = 1000)]
    delay: u32,
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
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
        <span class=format!("font-mono {class}")>
            {move || content().to_string()}{symbol}
        </span>
    }
}

