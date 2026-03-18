//! Plotter — inline function graph using egui_plot.

use crate::app::AbacoApp;
use crate::theme;

/// Draw the plot panel at the bottom of the central area (if expression is set).
pub fn plot_panel(ui: &mut egui::Ui, app: &AbacoApp) {
    if app.plot_expr.is_empty() || app.plot_points.is_empty() {
        return;
    }

    ui.separator();
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(format!("f(x) = {}", app.plot_expr))
            .color(theme::TEXT_MUTED)
            .small(),
    );

    let line_points: egui_plot::PlotPoints = app.plot_points.iter().map(|p| [p[0], p[1]]).collect();
    let line = egui_plot::Line::new("f(x)", line_points).color(theme::ACCENT);

    egui_plot::Plot::new("fn_plot")
        .height(180.0)
        .allow_drag(true)
        .allow_zoom(true)
        .allow_scroll(true)
        .show(ui, |plot_ui| {
            plot_ui.line(line);
        });
}
