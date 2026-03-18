//! History view — searchable calculation history.

use crate::app::AbacoApp;
use crate::theme;

pub fn history_view(ui: &mut egui::Ui, app: &mut AbacoApp) {
    ui.heading("History");
    ui.add_space(8.0);

    let entry_count = app.history.len();

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("{entry_count} entries")).color(theme::TEXT_MUTED));
        if entry_count > 0 && ui.button("Clear").clicked() {
            app.history.clear();
        }
    });

    ui.add_space(8.0);

    if entry_count == 0 {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("No history yet").color(theme::TEXT_MUTED));
        });
        return;
    }

    // Collect entries to avoid borrow conflicts
    let display_entries: Vec<_> = app
        .history
        .entries()
        .iter()
        .rev()
        .map(|e| (e.input.clone(), e.result.clone(), e.timestamp.clone()))
        .collect();

    let mut clicked_input: Option<String> = None;

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("history_grid")
            .striped(true)
            .min_col_width(60.0)
            .show(ui, |ui| {
                ui.strong("Input");
                ui.strong("Result");
                ui.strong("Time");
                ui.end_row();

                for (input, result, timestamp) in &display_entries {
                    let time_str = timestamp
                        .split('T')
                        .nth(1)
                        .and_then(|t| t.split('.').next())
                        .unwrap_or(timestamp);

                    if ui
                        .label(
                            egui::RichText::new(input)
                                .color(theme::TEXT_SECONDARY)
                                .monospace(),
                        )
                        .on_hover_text("Click to copy to calculator")
                        .clicked()
                    {
                        clicked_input = Some(input.clone());
                    }

                    ui.label(egui::RichText::new(result).color(theme::ACCENT).monospace());
                    ui.label(egui::RichText::new(time_str).color(theme::TEXT_MUTED));
                    ui.end_row();
                }
            });
    });

    if let Some(input) = clicked_input {
        app.calc_input = input;
        app.view = crate::app::View::Calculator;
        app.evaluate();
    }
}
