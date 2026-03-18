//! Sidebar — left nav panel for view switching.

use crate::app::{AbacoApp, View};
use crate::theme;

pub fn sidebar(ui: &mut egui::Ui, app: &mut AbacoApp) {
    ui.add_space(8.0);

    let entries = [
        (View::Calculator, "Calculator"),
        (View::Converter, "Converter"),
        (View::History, "History"),
    ];

    for (view, label) in entries {
        let is_active = app.view == view;
        let color = if is_active {
            theme::ACCENT
        } else {
            theme::TEXT_SECONDARY
        };

        let response = ui.selectable_label(is_active, egui::RichText::new(label).color(color));
        if response.clicked() {
            app.view = view;
        }
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // Plotter toggle — always accessible from sidebar
    let plot_active = !app.plot_expr.is_empty();
    let plot_color = if plot_active {
        theme::ACCENT
    } else {
        theme::TEXT_SECONDARY
    };
    ui.label(egui::RichText::new("Plot f(x)").color(plot_color).small());
    let response = ui.add(
        egui::TextEdit::singleline(&mut app.plot_expr)
            .hint_text("e.g. sin(x)")
            .desired_width(80.0),
    );
    if response.changed() || response.lost_focus() {
        app.update_plot();
    }
}
