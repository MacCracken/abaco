use abaco_core::{ConversionResult, Unit, UnitCategory};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum UnitError {
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),
    #[error("Incompatible units: {0} and {1} are different categories")]
    IncompatibleUnits(String, String),
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

pub type Result<T> = std::result::Result<T, UnitError>;

/// Registry of known units and conversion logic.
pub struct UnitRegistry {
    units: HashMap<UnitCategory, Vec<Unit>>,
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitRegistry {
    /// Create a new registry populated with built-in units.
    pub fn new() -> Self {
        let mut reg = Self {
            units: HashMap::new(),
        };
        reg.populate_defaults();
        reg
    }

    fn add(&mut self, unit: Unit) {
        self.units
            .entry(unit.category)
            .or_default()
            .push(unit);
    }

    fn populate_defaults(&mut self) {
        // Length (base: meter)
        self.add(Unit::new("meter", "m", UnitCategory::Length, 1.0, 0.0));
        self.add(Unit::new("kilometer", "km", UnitCategory::Length, 1000.0, 0.0));
        self.add(Unit::new("centimeter", "cm", UnitCategory::Length, 0.01, 0.0));
        self.add(Unit::new("millimeter", "mm", UnitCategory::Length, 0.001, 0.0));
        self.add(Unit::new("mile", "mi", UnitCategory::Length, 1609.344, 0.0));
        self.add(Unit::new("yard", "yd", UnitCategory::Length, 0.9144, 0.0));
        self.add(Unit::new("foot", "ft", UnitCategory::Length, 0.3048, 0.0));
        self.add(Unit::new("inch", "in", UnitCategory::Length, 0.0254, 0.0));
        self.add(Unit::new("nautical_mile", "nmi", UnitCategory::Length, 1852.0, 0.0));

        // Mass (base: kilogram)
        self.add(Unit::new("kilogram", "kg", UnitCategory::Mass, 1.0, 0.0));
        self.add(Unit::new("gram", "g", UnitCategory::Mass, 0.001, 0.0));
        self.add(Unit::new("milligram", "mg", UnitCategory::Mass, 0.000001, 0.0));
        self.add(Unit::new("pound", "lb", UnitCategory::Mass, 0.453592, 0.0));
        self.add(Unit::new("ounce", "oz", UnitCategory::Mass, 0.0283495, 0.0));
        self.add(Unit::new("ton", "t", UnitCategory::Mass, 1000.0, 0.0));
        self.add(Unit::new("stone", "st", UnitCategory::Mass, 6.35029, 0.0));

        // Temperature (base: celsius, using offset conversion)
        // For temperature: base_value = (value + to_base_offset) * to_base_factor
        // Celsius is the base: factor=1, offset=0
        // Fahrenheit: C = (F - 32) * 5/9 => offset=-32, factor=5/9
        // Kelvin: C = K - 273.15 => offset=-273.15, factor=1
        self.add(Unit::new("celsius", "C", UnitCategory::Temperature, 1.0, 0.0));
        self.add(Unit::new(
            "fahrenheit",
            "F",
            UnitCategory::Temperature,
            5.0 / 9.0,
            -32.0,
        ));
        self.add(Unit::new(
            "kelvin",
            "K",
            UnitCategory::Temperature,
            1.0,
            -273.15,
        ));

        // Time (base: second)
        self.add(Unit::new("second", "s", UnitCategory::Time, 1.0, 0.0));
        self.add(Unit::new("minute", "min", UnitCategory::Time, 60.0, 0.0));
        self.add(Unit::new("hour", "hr", UnitCategory::Time, 3600.0, 0.0));
        self.add(Unit::new("day", "d", UnitCategory::Time, 86400.0, 0.0));
        self.add(Unit::new("week", "wk", UnitCategory::Time, 604800.0, 0.0));
        self.add(Unit::new("year", "yr", UnitCategory::Time, 31557600.0, 0.0));

        // Data size (base: byte)
        self.add(Unit::new("byte", "B", UnitCategory::DataSize, 1.0, 0.0));
        self.add(Unit::new("kilobyte", "KB", UnitCategory::DataSize, 1024.0, 0.0));
        self.add(Unit::new("megabyte", "MB", UnitCategory::DataSize, 1_048_576.0, 0.0));
        self.add(Unit::new(
            "gigabyte",
            "GB",
            UnitCategory::DataSize,
            1_073_741_824.0,
            0.0,
        ));
        self.add(Unit::new(
            "terabyte",
            "TB",
            UnitCategory::DataSize,
            1_099_511_627_776.0,
            0.0,
        ));
        self.add(Unit::new(
            "petabyte",
            "PB",
            UnitCategory::DataSize,
            1_125_899_906_842_624.0,
            0.0,
        ));

        // Speed (base: m/s)
        self.add(Unit::new("meters_per_second", "m/s", UnitCategory::Speed, 1.0, 0.0));
        self.add(Unit::new(
            "kilometers_per_hour",
            "km/h",
            UnitCategory::Speed,
            1.0 / 3.6,
            0.0,
        ));
        self.add(Unit::new(
            "miles_per_hour",
            "mph",
            UnitCategory::Speed,
            0.44704,
            0.0,
        ));
        self.add(Unit::new("knot", "kn", UnitCategory::Speed, 0.514444, 0.0));

        // Area (base: sq meter)
        self.add(Unit::new("square_meter", "m2", UnitCategory::Area, 1.0, 0.0));
        self.add(Unit::new("square_kilometer", "km2", UnitCategory::Area, 1_000_000.0, 0.0));
        self.add(Unit::new("hectare", "ha", UnitCategory::Area, 10_000.0, 0.0));
        self.add(Unit::new("acre", "ac", UnitCategory::Area, 4046.8564224, 0.0));
        self.add(Unit::new("square_foot", "ft2", UnitCategory::Area, 0.092903, 0.0));

        // Volume (base: liter)
        self.add(Unit::new("liter", "L", UnitCategory::Volume, 1.0, 0.0));
        self.add(Unit::new("milliliter", "mL", UnitCategory::Volume, 0.001, 0.0));
        self.add(Unit::new("gallon", "gal", UnitCategory::Volume, 3.78541, 0.0));
        self.add(Unit::new("quart", "qt", UnitCategory::Volume, 0.946353, 0.0));
        self.add(Unit::new("pint", "pt", UnitCategory::Volume, 0.473176, 0.0));
        self.add(Unit::new("cup", "cup", UnitCategory::Volume, 0.236588, 0.0));
        self.add(Unit::new("tablespoon", "tbsp", UnitCategory::Volume, 0.0147868, 0.0));
        self.add(Unit::new("teaspoon", "tsp", UnitCategory::Volume, 0.00492892, 0.0));
    }

    /// Find a unit by name or symbol (case-insensitive).
    pub fn find_unit(&self, name_or_symbol: &str) -> Option<&Unit> {
        let query = name_or_symbol.to_lowercase();
        for units in self.units.values() {
            for unit in units {
                if unit.name.to_lowercase() == query
                    || unit.symbol.to_lowercase() == query
                {
                    return Some(unit);
                }
            }
        }
        // Also try plural forms (strip trailing 's')
        if query.ends_with('s') {
            let singular = &query[..query.len() - 1];
            for units in self.units.values() {
                for unit in units {
                    if unit.name.to_lowercase() == singular {
                        return Some(unit);
                    }
                }
            }
        }
        None
    }

    /// List all units in a category.
    pub fn list_units(&self, category: UnitCategory) -> Vec<&Unit> {
        self.units
            .get(&category)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Convert a value between two units.
    pub fn convert(&self, value: f64, from: &str, to: &str) -> Result<ConversionResult> {
        let from_unit = self
            .find_unit(from)
            .ok_or_else(|| UnitError::UnknownUnit(from.to_string()))?;
        let to_unit = self
            .find_unit(to)
            .ok_or_else(|| UnitError::UnknownUnit(to.to_string()))?;

        if from_unit.category != to_unit.category {
            return Err(UnitError::IncompatibleUnits(
                from_unit.name.clone(),
                to_unit.name.clone(),
            ));
        }

        // Convert to base unit, then from base to target
        let base_value = (value + from_unit.to_base_offset) * from_unit.to_base_factor;
        let result = base_value / to_unit.to_base_factor - to_unit.to_base_offset;

        Ok(ConversionResult {
            from_value: value,
            from_unit: from_unit.symbol.clone(),
            to_value: result,
            to_unit: to_unit.symbol.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg() -> UnitRegistry {
        UnitRegistry::new()
    }

    #[test]
    fn test_km_to_miles() {
        let r = reg().convert(1.0, "km", "mi").unwrap();
        assert!((r.to_value - 0.621371).abs() < 0.001);
    }

    #[test]
    fn test_miles_to_km() {
        let r = reg().convert(1.0, "mi", "km").unwrap();
        assert!((r.to_value - 1.60934).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        let r = reg().convert(100.0, "celsius", "fahrenheit").unwrap();
        assert!((r.to_value - 212.0).abs() < 0.1);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let r = reg().convert(32.0, "fahrenheit", "celsius").unwrap();
        assert!(r.to_value.abs() < 0.1);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let r = reg().convert(0.0, "celsius", "kelvin").unwrap();
        assert!((r.to_value - 273.15).abs() < 0.1);
    }

    #[test]
    fn test_kg_to_pounds() {
        let r = reg().convert(1.0, "kg", "lb").unwrap();
        assert!((r.to_value - 2.20462).abs() < 0.001);
    }

    #[test]
    fn test_bytes_to_gb() {
        let r = reg().convert(1_073_741_824.0, "byte", "GB").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_gallons_to_liters() {
        let r = reg().convert(1.0, "gallon", "liter").unwrap();
        assert!((r.to_value - 3.78541).abs() < 0.001);
    }

    #[test]
    fn test_liters_to_gallons() {
        let r = reg().convert(3.78541, "liter", "gallon").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_find_unit_case_insensitive() {
        let r = reg();
        assert!(r.find_unit("Kilometer").is_some());
        assert!(r.find_unit("KM").is_some());
    }

    #[test]
    fn test_find_unit_plural() {
        let r = reg();
        assert!(r.find_unit("meters").is_some());
    }

    #[test]
    fn test_unknown_unit() {
        let result = reg().convert(1.0, "foo", "bar");
        assert!(result.is_err());
    }

    #[test]
    fn test_incompatible_units() {
        let result = reg().convert(1.0, "km", "kg");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_units_length() {
        let r = reg();
        let units = r.list_units(UnitCategory::Length);
        assert!(units.len() >= 9);
    }

    #[test]
    fn test_hours_to_minutes() {
        let r = reg().convert(2.0, "hour", "minute").unwrap();
        assert!((r.to_value - 120.0).abs() < 0.1);
    }
}
