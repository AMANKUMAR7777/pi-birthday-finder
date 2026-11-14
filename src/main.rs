mod pi_search;
mod pi_streamer;
mod app;

use app::PiApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("π Birthday Quest - Find Your Birthday in Pi")
            .with_inner_size([780.0, 680.0])
            .with_min_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pi Birthday Quest",
        options,
        Box::new(|_cc| Ok(Box::new(PiApp::default()))),
    )
}
