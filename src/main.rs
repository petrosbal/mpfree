// MpFree: Lightweight youtube-to-mp3 converter built with Rust and egui
// Copyright (C) 2026  Petros Baloglou
// This program comes with ABSOLUTELY NO WARRANTY.
// This is free software, and you are welcome to redistribute it
// under certain conditions; see LICENSE for details.

#![windows_subsystem = "windows"]

use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;

// ===== EMBEDDED BINARIES =====

#[cfg(target_os = "windows")]
const YTDLP_BINARY: &[u8] = include_bytes!("../bin/yt-dlp.exe");
#[cfg(target_os = "windows")]
const FFMPEG_BINARY: &[u8] = include_bytes!("../bin/ffmpeg.exe");

#[cfg(not(target_os = "windows"))]
const YTDLP_BINARY: &[u8] = include_bytes!("../bin/yt-dlp");
#[cfg(not(target_os = "windows"))]
const FFMPEG_BINARY: &[u8] = include_bytes!("../bin/ffmpeg");

// ===== APP PATHS MANAGEMENT =====

struct AppPaths {
    ytdlp: PathBuf,
    ffmpeg: PathBuf,
}

impl AppPaths {
    pub fn init() -> Self {
        let temp_dir = std::env::temp_dir();

        let yt_name = if cfg!(target_os = "windows") { "yt-dlp.exe" } else { "yt-dlp" };
        let ff_name = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };

        let ytdlp = temp_dir.join(yt_name);
        let ffmpeg = temp_dir.join(ff_name);

        Self::prepare_file(&ytdlp, YTDLP_BINARY);
        Self::prepare_file(&ffmpeg, FFMPEG_BINARY);

        Self { ytdlp, ffmpeg }
    }

    fn prepare_file(path: &PathBuf, bytes: &[u8]) {
        if !path.exists() {
            fs::write(path, bytes).expect("Failed to write binary to temp");
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
            }
        }
    }
}

// ===== MAIN APPLICATION STRUCTURE =====

struct MpFreeApp {
    url: String,
    paths: AppPaths,
    status: String,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,
    download_path: Option<PathBuf>,
}

impl MpFreeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&_cc.egui_ctx);

        let (tx, rx) = mpsc::channel();
        let paths = AppPaths::init();

        Self {
            url: String::new(),
            paths,
            status: "Ready".to_string(),
            tx,
            rx,
            download_path: None,
        }
    }

    fn run_download(&mut self) {
        let url = self.url.clone();
        let tx = self.tx.clone();
        let yt_path = self.paths.ytdlp.clone();
        let ff_path = self.paths.ffmpeg.clone();
        let download_path = self.download_path.clone();

        std::thread::spawn(move || {
            let mut cmd = Command::new(yt_path);

            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
            }

            let output_template = match download_path {
                Some(p) => p.join("%(title)s.%(ext)s").to_string_lossy().to_string(),
                None => "%(title)s.%(ext)s".to_string(),
            };
            let result = cmd
                .args([
                    "-x", 
                    "--audio-format", "mp3",
                    "--ffmpeg-location", ff_path.to_str().unwrap(),
                    "-o", &output_template,
                ])
                .arg(url)
                .output();

            let msg = match result {
                Ok(output) if output.status.success() => "Download and conversion completed.".to_string(),
                Ok(output) => format!("yt-dlp error: {}", String::from_utf8_lossy(&output.stderr)),
                Err(e) => format!("Failed to start: {}", e),
            };
            let _ = tx.send(msg);
        });
    }
}

// ===== UI IMPLEMENTATION =====

impl eframe::App for MpFreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        ctx.set_cursor_icon(egui::CursorIcon::Default);
        if let Ok(msg) = self.rx.try_recv() {
            self.status = msg;
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(15.0);
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/mpfree_logo.png"))
                        .max_width(200.0)
                );
                ui.add_space(15.0);

                // 1. URL input
                ui.add(egui::TextEdit::singleline(&mut self.url)
                    .hint_text("Paste YouTube URL here...")
                    .desired_width(350.0));

                ui.add_space(10.0);

                // 2. folder selection
                ui.horizontal(|ui| {
                    let button_width = 115.0; // width of the "Select Folder" button
                    let spacing = 8.0; // space between button and label
                    let max_label_width = 180.0; // maximum width for the path label
                    let font_id = egui::FontId::proportional(13.0);

                    // determine display path
                    let display_path = match &self.download_path {
                        Some(p) => p.to_string_lossy().to_string(),
                        None => "Default folder".to_string(),
                    };

                    // measure label width
                    let galley = ui.painter().layout_no_wrap(
                        display_path.clone(), 
                        font_id.clone(), 
                        ui.visuals().text_color()
                    );
                    let actual_label_width = galley.rect.width().min(max_label_width);

                    // calculate left padding for centering
                    let total_row_width = button_width + spacing + actual_label_width;
                    let left_padding = (ui.available_width() - total_row_width) / 2.0;
                    if left_padding > 0.0 {
                        ui.add_space(left_padding);
                    }
                    
                    // select folder button
                    if ui.add_sized([button_width, 22.0], egui::Button::new("📁 Select Folder")).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.download_path = Some(path);
                        }
                        ctx.request_repaint();
                    }

                    ui.add_space(spacing);

                    ui.add_sized(
                        [actual_label_width, 22.0],
                        egui::Label::new(egui::RichText::new(display_path).font(font_id))
                            .truncate()
                    );
                });

                ui.add_space(10.0);

                // 3. download button
                if ui.button(egui::RichText::new("Download MP3").heading()).clicked() {
                    if self.url.is_empty() {
                        self.status = "Please enter a URL first.".to_string();
                    } else {
                        self.status = "Downloading...".to_string();
                        self.run_download();
                    }
                }

                ui.add_space(15.0);
                ui.label(egui::RichText::new(&self.status).strong());
            });
        });
    }
}

// ===== MAIN ENTRY POINT =====

fn main() -> eframe::Result {
    let size = [800.0, 400.0];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size),
        ..Default::default()
    };
    
    eframe::run_native(
        "MpFree",
        options,
        Box::new(|cc| Ok(Box::new(MpFreeApp::new(cc)))),
    )
}