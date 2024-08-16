use eframe::egui;
use crate::rodio;

pub struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Play Audio").clicked() {
                if let Err(e) = rodio::play_audio("/home/eggbert/songs/Divine1.mp3") {
                    eprintln!("Error playing audio: {}", e);
                }
            }
        });
    }
}
