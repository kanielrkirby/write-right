use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(inline_js = "
export function highlight_all() {
    window.Prism.highlightAll();
}
")]
extern "C" {
    /// highlight_all runs Prism.highlightAll
    pub fn highlight_all();
}
