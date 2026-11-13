//! Stick the Charge — a minimalist, high-contrast phone interface for browsing criminal
//! charges by number and name.
//!
//! Run it plainly to use the app. With `--screenshot <file.png>` it opens without taking
//! focus, draws about 120 frames, saves the window's picture and exits, so a check can
//! capture it without disturbing anyone.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Literal floats in the drawing code are all f32 by intent.
#![allow(float_literal_f32_fallback)]

mod app;
mod charges;
mod leaf;
mod statutes;
mod theme;

use std::path::PathBuf;

use eframe::egui;

/// How many frames to draw before taking the picture.
const DRAW_FRAMES: u32 = 120;
/// Give up on a picture after this many frames rather than hanging.
const MAX_FRAMES: u32 = 900;

struct Shot {
    path: PathBuf,
    frames: u32,
    asked_at: Option<u32>,
    saved: bool,
}

fn main() -> eframe::Result<()> {
    let (shot, mode) = parse_cli();

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Stick the Charge")
        .with_inner_size([468.0, 900.0])
        .with_min_inner_size([380.0, 620.0]);

    if shot.is_some() {
        // Open without stealing focus, and on Windows keep it behind other windows: a covered
        // window still draws there, so the picture still arrives.
        viewport = viewport.with_active(false);
        #[cfg(target_os = "windows")]
        {
            viewport = viewport.with_window_level(egui::viewport::WindowLevel::AlwaysOnBottom);
        }
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Stick the Charge",
        options,
        Box::new(move |cc| {
            theme::initialize(&cc.egui_ctx);
            Ok(Box::new(StickTheCharge::new(shot, mode)))
        }),
    )
}

fn parse_cli() -> (Option<Shot>, Option<String>) {
    let mut args = std::env::args().skip(1);
    let mut shot_path = None;
    let mut mode = None;
    while let Some(a) = args.next() {
        if a == "--screenshot" {
            shot_path = args.next().map(PathBuf::from);
        } else if let Some(rest) = a.strip_prefix("--screenshot=") {
            shot_path = Some(PathBuf::from(rest));
        } else if a == "--mode" || a == "--open" {
            mode = args.next();
        } else if let Some(rest) = a.strip_prefix("--mode=").or_else(|| a.strip_prefix("--open=")) {
            mode = Some(rest.to_string());
        }
    }
    let shot = shot_path.map(|path| Shot { path, frames: 0, asked_at: None, saved: false });
    (shot, mode)
}

struct StickTheCharge {
    app: app::App,
    shot: Option<Shot>,
}

impl StickTheCharge {
    fn new(shot: Option<Shot>, mode: Option<String>) -> Self {
        let mut app = app::App::default();
        if let Some(m) = mode {
            app.open_mode(&m);
        }
        Self { app, shot }
    }
}

impl eframe::App for StickTheCharge {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.ui(ctx);

        let Some(shot) = self.shot.as_mut() else {
            return;
        };
        shot.frames += 1;
        // Keep drawing even while unfocused, or no picture ever arrives.
        ctx.request_repaint();

        if shot.asked_at.is_none() && shot.frames >= DRAW_FRAMES {
            shot.asked_at = Some(shot.frames);
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }

        if !shot.saved {
            let image = ctx.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });
            if let Some(image) = image {
                match save_png(&shot.path, &image) {
                    Ok(()) => {
                        eprintln!("saved {} ({}x{})", shot.path.display(), image.size[0], image.size[1]);
                        shot.saved = true;
                    }
                    Err(err) => eprintln!("could not save {}: {err}", shot.path.display()),
                }
            }
        }

        if shot.saved || shot.frames > MAX_FRAMES {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn save_png(path: &std::path::Path, image: &egui::ColorImage) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
    }
    let (w, h) = (image.size[0] as u32, image.size[1] as u32);
    let mut bytes = Vec::with_capacity((w * h * 4) as usize);
    for px in &image.pixels {
        bytes.extend_from_slice(&px.to_array());
    }
    image::save_buffer(path, &bytes, w, h, image::ExtendedColorType::Rgba8)
        .map_err(|e| e.to_string())
}
