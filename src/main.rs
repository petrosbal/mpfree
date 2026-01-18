const YTDLP_BINARY: &[u8] = include_bytes!("../yt-dlp");

use eframe::egui;
use std::process::Command;
use std::sync::mpsc;

struct MpFreeApp {
    url: String,
    status: String,
    tx: mpsc::Sender<String>,
    rx: mpsc::Receiver<String>,

}

impl Default for MpFreeApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            url: String::new(),
            status: "Ready".to_string(),
            tx,
            rx,
        }
    }
}

impl MpFreeApp {
    fn run_download(&mut self) {
        let url_to_download = self.url.clone();
        let tx = self.tx.clone();

        std::thread::spawn(move || {
            let mut cmd = Command::new("yt-dlp");

            #[cfg(windows)]
            use std::os::windows::process::CommandExt;
            #[cfg(windows)]
            cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

            let status = cmd
                .args(["-x", "--audio-format", "mp3", "-o", "%(title)s.%(ext)s"])
                .arg(url_to_download)
                .status();

            let msg = match status {
                Ok(s) if s.success() => "Download completed!".to_string(),
                _ => "Download failed. Check your URL or yt-dlp.".to_string(),
            };
            let _ = tx.send(msg);
        });
    }
}



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
                        self.status = "You should probably enter a URL first.".to_string();
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


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([400.0, 200.0])
        .with_resizable(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "MpFree v1.0",
        options,
        Box::new(|_cc| Ok(Box::new(MpFreeApp::default()))),
    )
}
