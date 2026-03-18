//! abaco-gui — Desktop GUI for the Abaco calculator and unit converter

mod app;
mod theme;
mod views;

pub use app::AbacoApp;

/// Launch the GUI window. Blocks until closed.
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Abaco — Calculator & Unit Converter")
            .with_inner_size([800.0, 560.0])
            .with_min_inner_size([520.0, 380.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Abaco",
        options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(AbacoApp::new()))
        }),
    )
}
