use wasm_bindgen::prelude::wasm_bindgen;
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

/// sleep_ms converts a JS Promise to a Rust Future and awaits the specified `ms`.
pub async fn sleep_ms(ms: u32) {
    let promise = sleep(ms);
    let js_future = JsFuture::from(promise);
    js_future.await.unwrap();
}

