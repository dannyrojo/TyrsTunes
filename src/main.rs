mod gui;
mod rodio;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "TyrsTunes",
        options,
        Box::new(|cc| Ok(Box::new(gui::MyApp::new(cc)))));
}