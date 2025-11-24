mod operation;
mod state;
mod calculator;
mod app;

use app::CalculatorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "GUI Calculator",
        options,
        Box::new(|cc| Box::new(CalculatorApp::new(cc))),
    )
}

