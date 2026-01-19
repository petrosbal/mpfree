// MpFree: Lightweight youtube-to-mp3 converter built with Rust and egui
// Copyright (C) 2026  Petros Baloglou
// This program comes with ABSOLUTELY NO WARRANTY.
// This is free software, and you are welcome to redistribute it
// under certain conditions; see LICENSE for details.

#![windows_subsystem = "windows"]

mod paths;
mod app;
mod ui;

use app::MpFreeApp;
use eframe::egui;

fn main() -> eframe::Result {
    let icon_bytes = include_bytes!("../assets/mpfree_icon.png");
    let icon = load_icon(icon_bytes);

    let size = [600.0, 400.0];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_icon(icon),
        ..Default::default()
    };
    
    eframe::run_native(
        "MpFree",
        options,
        Box::new(|cc| Ok(Box::new(MpFreeApp::new(cc)))),
    )
}

fn load_icon(bytes: &[u8]) -> egui::IconData {
    let img = image::load_from_memory(bytes).expect("Failed to load icon");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    }
}