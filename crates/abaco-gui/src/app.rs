//! AbacoApp — eframe::App implementation for the Abaco desktop GUI.

use abaco_ai::{CalculationHistory, NlParser, ParsedQuery};
use abaco_eval::Evaluator;
use abaco_units::UnitRegistry;

use crate::views;

/// Active view in the main panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Calculator,
    Converter,
    History,
}

/// AbacoApp owns all state; eframe::App::update() runs on the main thread.
pub struct AbacoApp {
    pub view: View,

    // Calculator state
    pub calc_input: String,
    pub calc_result: Option<String>,
    pub calc_error: Option<String>,
    pub evaluator: Evaluator,
    pub nl_parser: NlParser,

    // Converter state
    pub registry: UnitRegistry,
    pub conv_value_str: String,
    pub conv_from: String,
    pub conv_to: String,
    pub conv_result: Option<String>,
    pub conv_error: Option<String>,
    pub conv_selected_category: usize,

    // History
    pub history: CalculationHistory,

    // Plotter state
    pub plot_expr: String,
    pub plot_points: Vec<[f64; 2]>,
}

impl Default for AbacoApp {
    fn default() -> Self {
        Self::new()
    }
}

impl AbacoApp {
    pub fn new() -> Self {
        Self {
            view: View::Calculator,

            calc_input: String::new(),
            calc_result: None,
            calc_error: None,
            evaluator: Evaluator::new(),
            nl_parser: NlParser::new(),

            registry: UnitRegistry::new(),
            conv_value_str: String::new(),
            conv_from: String::new(),
            conv_to: String::new(),
            conv_result: None,
            conv_error: None,
            conv_selected_category: 0,

            history: CalculationHistory::new(200),

            plot_expr: String::new(),
            plot_points: Vec::new(),
        }
    }

    /// Evaluate the current calculator input.
    pub fn evaluate(&mut self) {
        let input = self.calc_input.trim();
        if input.is_empty() {
            self.calc_result = None;
            self.calc_error = None;
            return;
        }

        // Try variable assignment first
        if let Some(eq_pos) = input.find('=') {
            let var_name = input[..eq_pos].trim();
            let is_valid_ident = !var_name.is_empty()
                && var_name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && var_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_');
            if is_valid_ident {
                let expr = input[eq_pos + 1..].trim();
                match self.evaluator.eval(expr) {
                    Ok(value) => {
                        if let Some(f) = value.as_f64() {
                            self.evaluator.set_variable(var_name, f);
                        }
                        let result_str = value.to_string();
                        self.history.push(input, &result_str);
                        self.calc_result = Some(result_str);
                        self.calc_error = None;
                    }
                    Err(e) => {
                        self.calc_result = None;
                        self.calc_error = Some(e.to_string());
                    }
                }
                return;
            }
        }

        // Try NL parsing (handles "what is", "convert X to Y", "X% of Y", etc.)
        if let Ok(query) = self.nl_parser.parse_natural(input) {
            match query {
                ParsedQuery::Conversion { value, from, to } => {
                    match self.registry.convert(value, &from, &to) {
                        Ok(result) => {
                            let result_str = result.to_string();
                            self.history.push(input, &result_str);
                            self.calc_result = Some(result_str);
                            self.calc_error = None;
                        }
                        Err(e) => {
                            self.calc_result = None;
                            self.calc_error = Some(e.to_string());
                        }
                    }
                    return;
                }
                ParsedQuery::CurrencyConversion { value, from, to } => {
                    let msg = format!(
                        "{value} {from} -> {to} (live rates via hoosh — not yet connected)"
                    );
                    self.history.push(input, &msg);
                    self.calc_result = Some(msg);
                    self.calc_error = None;
                    return;
                }
                ParsedQuery::Calculation(expr) => {
                    match self.evaluator.eval(&expr) {
                        Ok(value) => {
                            let result_str = value.to_string();
                            self.history.push(input, &result_str);
                            self.calc_result = Some(result_str);
                            self.calc_error = None;
                        }
                        Err(e) => {
                            self.calc_result = None;
                            self.calc_error = Some(e.to_string());
                        }
                    }
                    return;
                }
            }
        }

        // Fall back to direct eval
        match self.evaluator.eval(input) {
            Ok(value) => {
                let result_str = value.to_string();
                self.history.push(input, &result_str);
                self.calc_result = Some(result_str);
                self.calc_error = None;
            }
            Err(e) => {
                self.calc_result = None;
                self.calc_error = Some(e.to_string());
            }
        }
    }

    /// Run a unit conversion.
    pub fn convert(&mut self) {
        let value: f64 = match self.conv_value_str.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                self.conv_result = None;
                self.conv_error = if self.conv_value_str.trim().is_empty() {
                    None
                } else {
                    Some("Invalid number".to_string())
                };
                return;
            }
        };

        if self.conv_from.is_empty() || self.conv_to.is_empty() {
            self.conv_result = None;
            self.conv_error = None;
            return;
        }

        match self.registry.convert(value, &self.conv_from, &self.conv_to) {
            Ok(result) => {
                let result_str = result.to_string();
                self.history.push(
                    &format!("{value} {} to {}", self.conv_from, self.conv_to),
                    &result_str,
                );
                self.conv_result = Some(result_str);
                self.conv_error = None;
            }
            Err(e) => {
                self.conv_result = None;
                self.conv_error = Some(e.to_string());
            }
        }
    }

    /// Generate plot points for an expression over a range.
    pub fn update_plot(&mut self) {
        self.plot_points.clear();
        let expr = self.plot_expr.trim();
        if expr.is_empty() {
            return;
        }

        let steps = 400;
        let x_min = -10.0_f64;
        let x_max = 10.0_f64;
        let dx = (x_max - x_min) / steps as f64;

        for i in 0..=steps {
            let x = x_min + i as f64 * dx;
            let mut eval = Evaluator::new();
            eval.set_variable("x", x);
            // Copy current variables
            if let Ok(val) = eval.eval(expr)
                && let Some(y) = val.as_f64()
                && y.is_finite()
            {
                self.plot_points.push([x, y]);
            }
        }
    }
}

impl eframe::App for AbacoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(110.0)
            .show(ctx, |ui| {
                views::sidebar::sidebar(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view {
                View::Calculator => views::calculator::calculator_view(ui, self),
                View::Converter => views::converter::converter_view(ui, self),
                View::History => views::history::history_view(ui, self),
            }

            // Plot panel at bottom if expression is set
            views::plotter::plot_panel(ui, self);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_creates() {
        let app = AbacoApp::new();
        assert_eq!(app.view, View::Calculator);
        assert!(app.calc_input.is_empty());
    }

    #[test]
    fn app_evaluates() {
        let mut app = AbacoApp::new();
        app.calc_input = "2 + 3".to_string();
        app.evaluate();
        assert_eq!(app.calc_result, Some("5".to_string()));
        assert!(app.calc_error.is_none());
    }

    #[test]
    fn app_converts() {
        let mut app = AbacoApp::new();
        app.conv_value_str = "100".to_string();
        app.conv_from = "celsius".to_string();
        app.conv_to = "fahrenheit".to_string();
        app.convert();
        assert!(app.conv_result.is_some());
        assert!(app.conv_error.is_none());
    }

    #[test]
    fn app_variable_assignment() {
        let mut app = AbacoApp::new();
        app.calc_input = "x = 42".to_string();
        app.evaluate();
        assert_eq!(app.calc_result, Some("42".to_string()));
        assert_eq!(app.evaluator.get_variable("x"), Some(42.0));
    }

    #[test]
    fn app_plot_generates_points() {
        let mut app = AbacoApp::new();
        app.plot_expr = "x * 2".to_string();
        app.update_plot();
        assert!(!app.plot_points.is_empty());
    }
}
