use eframe::egui;
use crate::app::MpFreeApp;

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

                ui.add(egui::TextEdit::singleline(&mut self.url)
                    .hint_text("Paste YouTube URL here...")
                    .desired_width(350.0));

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let button_width = 115.0;
                    let spacing = 8.0;
                    let max_label_width = 180.0;
                    let font_id = egui::FontId::proportional(13.0);

                    let display_path = match &self.download_path {
                        Some(p) => p.to_string_lossy().to_string(),
                        None => "Default folder".to_string(),
                    };

                    let galley = ui.painter().layout_no_wrap(
                        display_path.clone(), 
                        font_id.clone(), 
                        ui.visuals().text_color()
                    );
                    let actual_label_width = galley.rect.width().min(max_label_width);

                    let total_row_width = button_width + spacing + actual_label_width;
                    let left_padding = (ui.available_width() - total_row_width) / 2.0;
                    if left_padding > 0.0 { ui.add_space(left_padding); }
                    
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