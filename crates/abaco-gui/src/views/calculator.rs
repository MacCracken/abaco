//! Calculator view — expression input with live result display.

use crate::app::AbacoApp;
use crate::theme;

pub fn calculator_view(ui: &mut egui::Ui, app: &mut AbacoApp) {
    ui.heading("Calculator");
    ui.add_space(8.0);

    // Expression input
    let response = ui.add(
        egui::TextEdit::singleline(&mut app.calc_input)
            .hint_text("Type an expression... (e.g. sqrt(144) + 15%)")
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Monospace),
    );

    // Evaluate on Enter
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        app.evaluate();
        response.request_focus();
    }

    // Auto-focus on first frame
    if app.calc_result.is_none() && app.calc_error.is_none() && app.calc_input.is_empty() {
        response.request_focus();
    }

    ui.add_space(12.0);

    // Result display
    if let Some(ref result) = app.calc_result {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("=").color(theme::TEXT_MUTED).size(20.0));
                ui.label(
                    egui::RichText::new(result)
                        .color(theme::ACCENT)
                        .size(24.0)
                        .strong(),
                );
            });
        });
    }

    if let Some(ref error) = app.calc_error {
        ui.colored_label(theme::ERROR, error);
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // Recent history (last 10)
    ui.label(egui::RichText::new("Recent").color(theme::TEXT_MUTED));
    let entries: Vec<_> = app
        .history
        .entries()
        .iter()
        .rev()
        .take(10)
        .map(|e| (e.input.clone(), e.result.clone()))
        .collect();

    if entries.is_empty() {
        ui.label(egui::RichText::new("No calculations yet").color(theme::TEXT_MUTED));
    } else {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (input, result) in &entries {
                let response = ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(input)
                            .color(theme::TEXT_SECONDARY)
                            .monospace(),
                    );
                    ui.label(egui::RichText::new("=").color(theme::TEXT_MUTED));
                    ui.label(egui::RichText::new(result).color(theme::ACCENT).monospace());
                });
                // Click to reuse expression
                if response.response.interact(egui::Sense::click()).clicked() {
                    app.calc_input = input.clone();
                    app.evaluate();
                }
            }
        });
    }
}
