//! Draws the app headlessly and prints every piece of text it painted with its position, so
//! the layout can be checked without opening the window or looking at a picture.
//!
//! Run: cargo run --example layout_dump

#![allow(float_literal_f32_fallback)]

#[path = "../src/app.rs"]
mod app;
#[path = "../src/charges.rs"]
mod charges;
#[path = "../src/leaf.rs"]
mod leaf;
#[path = "../src/statutes.rs"]
mod statutes;
#[path = "../src/theme.rs"]
mod theme;

use eframe::egui::{self, pos2, Rect, Vec2};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "list".to_string());
    let size = match std::env::args().nth(2) {
        Some(s) => {
            let mut parts = s.split('x');
            let w = parts.next().and_then(|v| v.parse().ok()).unwrap_or(468.0);
            let h = parts.next().and_then(|v| v.parse().ok()).unwrap_or(900.0);
            Vec2::new(w, h)
        }
        None => Vec2::new(468.0, 900.0),
    };

    let ctx = egui::Context::default();
    theme::apply(&ctx);
    let mut application = app::App::default();

    if mode != "list" {
        application.open_mode(&mode);
    }

    let mut input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), size)),
        ..Default::default()
    };
    input.time = Some(0.0);

    let mut texts: Vec<(String, egui::Pos2)> = Vec::new();

    // Two passes: the first settles sizes, the second paints for real.
    for pass in 0..2 {
        let output = ctx.run(input.clone(), |ctx| {
            application.ui(ctx);
        });
        if pass == 1 {
            for clipped in output.shapes {
                collect(&clipped.shape, &mut texts);
            }
        }
        // Small frame step, like real frames.
        input.time = Some(0.016 * (pass as f64 + 1.0));
    }

    println!("--- {mode} ({}) ---", texts.len());
    let mut rows: Vec<(i32, i32, String)> = texts
        .into_iter()
        .map(|(t, p)| (p.y.round() as i32, p.x.round() as i32, t))
        .collect();
    rows.sort();
    for (y, x, t) in rows {
        println!("{y:>4},{x:>4}  {}", t.replace('\n', " / "));
    }
}

fn collect(shape: &egui::epaint::Shape, out: &mut Vec<(String, egui::Pos2)>) {
    use egui::epaint::Shape;
    match shape {
        Shape::Text(text) => out.push((text.galley.text().to_string(), text.pos)),
        Shape::Vec(shapes) => {
            for s in shapes {
                collect(s, out);
            }
        }
        _ => {}
    }
}
