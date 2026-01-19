// MpFree: Lightweight youtube-to-mp3 converter built with Rust and egui
// Copyright (C) 2026  Petros Baloglou

use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use crate::paths::AppPaths;

pub struct MpFreeApp {
    pub url: String,
    pub paths: AppPaths,
    pub status: String,
    pub tx: mpsc::Sender<String>,
    pub rx: mpsc::Receiver<String>,
    pub download_path: Option<PathBuf>,
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

    pub fn run_download(&mut self) {
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
                cmd.creation_flags(0x08000000);
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