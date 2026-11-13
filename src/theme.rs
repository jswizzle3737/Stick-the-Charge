//! High-contrast light and dark palettes for the phone interface.
use eframe::egui::{self, Color32};
use std::cell::Cell;

thread_local! { static DARK: Cell<bool> = const { Cell::new(false) }; }
pub fn is_dark() -> bool { DARK.with(Cell::get) }
fn color(light: [u8; 3], dark: [u8; 3]) -> Color32 {
    let [r, g, b] = if is_dark() { dark } else { light };
    Color32::from_rgb(r, g, b)
}
pub fn page() -> Color32 { color([240, 239, 233], [7, 7, 10]) }
pub fn body() -> Color32 { color([18, 22, 28], [0, 0, 0]) }
pub fn screen() -> Color32 { color([246, 245, 239], [15, 17, 22]) }
pub fn surface() -> Color32 { color([255, 255, 255], [30, 34, 41]) }
pub fn surface_hover() -> Color32 { color([240, 239, 233], [43, 49, 58]) }
pub fn surface_press() -> Color32 { color([232, 231, 224], [56, 64, 74]) }
pub fn line() -> Color32 { color([225, 226, 220], [50, 56, 66]) }
pub fn ink() -> Color32 { color([20, 39, 49], [247, 248, 250]) }
pub fn ink_dim() -> Color32 { color([74, 88, 96], [221, 227, 235]) }
pub fn ink_faint() -> Color32 { color([83, 98, 107], [170, 180, 192]) }
pub fn accent() -> Color32 { color([236, 75, 67], [255, 110, 100]) }
pub fn crimson() -> Color32 { color([183, 54, 49], [255, 110, 100]) }
#[allow(dead_code)]
pub fn brand() -> Color32 { Color32::from_rgb(183, 54, 49) }

#[allow(dead_code)]
fn preference_path() -> Option<std::path::PathBuf> {
    #[cfg(target_arch = "wasm32")]
    { Some(std::path::PathBuf::from("stick-the-charge-theme")) }
    #[cfg(not(target_arch = "wasm32"))]
    { crate::app::saved::storage_path().map(|path| path.with_file_name("theme.txt")) }
}

#[allow(dead_code)]
pub fn initialize(ctx: &egui::Context) {
    let dark = preference_path().and_then(|path| crate::app::saved::read_saved(&path).ok())
        .and_then(|values| values.into_iter().next()).as_deref() == Some("dark");
    DARK.with(|value| value.set(dark));
    apply(ctx);
}

#[allow(dead_code)]
pub fn toggle(ctx: &egui::Context) {
    let dark = !is_dark();
    DARK.with(|value| value.set(dark));
    if let Some(path) = preference_path() {
        let _ = crate::app::saved::write_saved(&path, &[if dark { "dark" } else { "light" }.to_owned()]);
    }
    apply(ctx);
    ctx.request_repaint();
}

pub fn apply(ctx: &egui::Context) {
    let mut visuals = if is_dark() { egui::Visuals::dark() } else { egui::Visuals::light() };
    visuals.panel_fill = screen();
    visuals.window_fill = screen();
    visuals.extreme_bg_color = surface();
    visuals.override_text_color = Some(ink());
    visuals.hyperlink_color = if is_dark() { Color32::from_rgb(0x9e, 0xd0, 0xff) } else { crimson() };
    visuals.selection.bg_fill = crimson();
    visuals.selection.stroke.color = screen();
    ctx.set_visuals(visuals);
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    ctx.set_style(style);
    #[cfg(target_arch = "wasm32")]
    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = root.set_attribute("data-theme", if is_dark() { "dark" } else { "light" });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contrast(a: Color32, b: Color32) -> f32 {
        fn luminance(c: Color32) -> f32 {
            let linear = |v: u8| {
                let v = f32::from(v) / 255.0;
                if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
            };
            0.2126 * linear(c.r()) + 0.7152 * linear(c.g()) + 0.0722 * linear(c.b())
        }
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn reading_colors_have_sufficient_contrast_in_both_modes() {
        for dark in [false, true] {
            DARK.with(|value| value.set(dark));
            for (name, foreground, background) in [
                ("body", ink_dim(), screen()),
                ("secondary", ink_faint(), screen()),
                ("secondary on surface", ink_faint(), surface()),
                ("heading", ink(), screen()),
                ("accent text", crimson(), screen()),
            ] {
                assert!(contrast(foreground, background) >= 4.5, "{name} contrast failed in {} mode", if dark { "dark" } else { "light" });
            }
        }
        DARK.with(|value| value.set(false));
    }
}
