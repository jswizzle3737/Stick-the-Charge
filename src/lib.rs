//! The same egui interface, available to the phone browser as WebAssembly.
#![allow(float_literal_f32_fallback)]

#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod charges;
#[cfg(target_arch = "wasm32")]
mod leaf;
#[cfg(target_arch = "wasm32")]
mod statutes;
#[cfg(target_arch = "wasm32")]
mod theme;

#[cfg(target_arch = "wasm32")]
struct WebApp(app::App);

#[cfg(target_arch = "wasm32")]
impl eframe::App for WebApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.0.ui(ctx);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new() -> Self { Self { runner: eframe::WebRunner::new() } }

    pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
        self.runner.start(canvas, eframe::WebOptions::default(), Box::new(|cc| {
            theme::initialize(&cc.egui_ctx);
            Ok(Box::new(WebApp(app::App::default())))
        })).await
    }
}
