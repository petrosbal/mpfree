// MpFree: Lightweight youtube-to-mp3 converter built with Rust and egui
// Copyright (C) 2026  Petros Baloglou

use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
const YTDLP_BINARY: &[u8] = include_bytes!("../bin/yt-dlp.exe");
#[cfg(target_os = "windows")]
const FFMPEG_BINARY: &[u8] = include_bytes!("../bin/ffmpeg.exe");

#[cfg(not(target_os = "windows"))]
const YTDLP_BINARY: &[u8] = include_bytes!("../bin/yt-dlp");
#[cfg(not(target_os = "windows"))]
const FFMPEG_BINARY: &[u8] = include_bytes!("../bin/ffmpeg");

pub struct AppPaths {
    pub ytdlp: PathBuf,
    pub ffmpeg: PathBuf,
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