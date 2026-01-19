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
}

impl MpFreeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel();
        
        // Αρχικοποίηση των binaries κατά το startup
        let paths = AppPaths::init();

        Self {
            url: String::new(),
            paths,
            status: "Ready".to_string(),
            tx,
            rx,
        }
    }

    fn run_download(&mut self) {
        let url = self.url.clone();
        let tx = self.tx.clone();
        let yt_path = self.paths.ytdlp.clone();
        let ff_path = self.paths.ffmpeg.clone();

        std::thread::spawn(move || {
            let mut cmd = Command::new(yt_path);

            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
            }

            let result = cmd
                .args([
                    "-x", 
                    "--audio-format", "mp3",
                    "--ffmpeg-location", ff_path.to_str().unwrap(),
                    "-o", "%(title)s.%(ext)s"
                ])
                .arg(url)
                .output();

            let msg = match result {
                Ok(output) if output.status.success() => {
                    "Download and conversion completed.".to_string()
                }
                Ok(output) => {
                    let err_msg = String::from_utf8_lossy(&output.stderr);
                    format!("yt-dlp error: {}", err_msg)
                }
                Err(e) => format!("Failed to start process: {}", e),
            };
            let _ = tx.send(msg);
        });
    }
}

// ===== UI IMPLEMENTATION =====

impl eframe::App for MpFreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Ok(msg) = self.rx.try_recv() {
            self.status = msg;
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("MpFree");
                ui.add_space(20.0);

                let edit = ui.add(egui::TextEdit::singleline(&mut self.url)
                    .hint_text("Paste YouTube URL here...")
                    .desired_width(350.0));

                if edit.changed() { self.status = "Ready".to_string(); }

                ui.add_space(20.0);

                if ui.button("Download MP3").clicked() {
                    if self.url.is_empty() {
                        self.status = "Please enter a URL first.".to_string();
                    } else {
                        self.status = "Downloading...".to_string();
                        self.run_download();
                    }
                }

                ui.add_space(20.0);
                ui.label(egui::RichText::new(&self.status).strong());
            });
        });
    }
}

// ===== MAIN ENTRY POINT =====

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 220.0])
            .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "MpFree v1.0",
        options,
        Box::new(|cc| Ok(Box::new(MpFreeApp::new(cc)))),
    )
}