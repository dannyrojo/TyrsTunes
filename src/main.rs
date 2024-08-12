mod audio_backend;
mod gui;
mod database;

use anyhow::Result;
use gui::MyApp;

fn main() -> Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TyrsTunes",
        options,
        Box::new(|cc| {
            let app = MyApp::new(cc).expect("Failed to create MyApp");
            Ok(Box::new(app) as Box<dyn eframe::App>)
        }),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {}", e))?;
    Ok(())
}