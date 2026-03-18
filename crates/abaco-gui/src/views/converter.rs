//! Converter view — unit conversion with category browser.

use abaco_core::UnitCategory;

use crate::app::AbacoApp;
use crate::theme;

pub fn converter_view(ui: &mut egui::Ui, app: &mut AbacoApp) {
    ui.heading("Unit Converter");
    ui.add_space(8.0);

    let categories = UnitCategory::all_categories();

    // Category selector
    ui.horizontal(|ui| {
        ui.label("Category:");
        for (i, cat) in categories.iter().enumerate() {
            let is_selected = app.conv_selected_category == i;
            let color = if is_selected {
                theme::ACCENT
            } else {
                theme::TEXT_SECONDARY
            };
            if ui
                .selectable_label(
                    is_selected,
                    egui::RichText::new(cat.to_string()).color(color),
                )
                .clicked()
            {
                app.conv_selected_category = i;
                app.conv_from.clear();
                app.conv_to.clear();
                app.conv_result = None;
                app.conv_error = None;
            }
        }
    });

    ui.add_space(12.0);

    let category = categories[app.conv_selected_category];

    // Collect unit data upfront to avoid borrow conflicts
    let unit_data: Vec<(String, String)> = app
        .registry
        .list_units(category)
        .iter()
        .map(|u| (u.name.clone(), u.symbol.clone()))
        .collect();

    // Value input
    ui.horizontal(|ui| {
        ui.label("Value:");
        let val_response = ui.add(
            egui::TextEdit::singleline(&mut app.conv_value_str)
                .hint_text("Enter number")
                .desired_width(120.0)
                .font(egui::TextStyle::Monospace),
        );
        if val_response.changed() {
            app.convert();
        }
    });

    ui.add_space(8.0);

    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("From: ");
        egui::ComboBox::from_id_salt("from_unit")
            .selected_text(if app.conv_from.is_empty() {
                "Select unit".to_string()
            } else {
                app.conv_from.clone()
            })
            .width(180.0)
            .show_ui(ui, |ui| {
                for (name, symbol) in &unit_data {
                    let label = format!("{name} ({symbol})");
                    if ui
                        .selectable_label(app.conv_from == *name, &label)
                        .clicked()
                    {
                        app.conv_from = name.clone();
                        changed = true;
                    }
                }
            });

        if ui.button("\u{21C4}").on_hover_text("Swap units").clicked() {
            std::mem::swap(&mut app.conv_from, &mut app.conv_to);
            changed = true;
        }

        ui.label("To:");
        egui::ComboBox::from_id_salt("to_unit")
            .selected_text(if app.conv_to.is_empty() {
                "Select unit".to_string()
            } else {
                app.conv_to.clone()
            })
            .width(180.0)
            .show_ui(ui, |ui| {
                for (name, symbol) in &unit_data {
                    let label = format!("{name} ({symbol})");
                    if ui.selectable_label(app.conv_to == *name, &label).clicked() {
                        app.conv_to = name.clone();
                        changed = true;
                    }
                }
            });
    });

    if changed {
        app.convert();
    }

    ui.add_space(12.0);

    // Result display
    if let Some(ref result) = app.conv_result {
        ui.group(|ui| {
            ui.label(
                egui::RichText::new(result)
                    .color(theme::ACCENT)
                    .size(20.0)
                    .strong(),
            );
        });
    }

    if let Some(ref error) = app.conv_error {
        ui.colored_label(theme::ERROR, error);
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // Unit reference for selected category
    ui.label(egui::RichText::new(format!("{category} units")).color(theme::TEXT_MUTED));
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("unit_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.strong("Name");
                ui.strong("Symbol");
                ui.end_row();
                for (name, symbol) in &unit_data {
                    ui.label(name);
                    ui.label(egui::RichText::new(symbol).color(theme::ACCENT).monospace());
                    ui.end_row();
                }
            });
    });
}
