use crate::core::{ConversionResult, Unit, UnitCategory};
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
        self.units.entry(unit.category).or_default().push(unit);
    }

    fn populate_defaults(&mut self) {
        // Length (base: meter)
        self.add(Unit::new("meter", "m", UnitCategory::Length, 1.0, 0.0));
        self.add(Unit::new(
            "kilometer",
            "km",
            UnitCategory::Length,
            1000.0,
            0.0,
        ));
        self.add(Unit::new(
            "centimeter",
            "cm",
            UnitCategory::Length,
            0.01,
            0.0,
        ));
        self.add(Unit::new(
            "millimeter",
            "mm",
            UnitCategory::Length,
            0.001,
            0.0,
        ));
        self.add(Unit::new("mile", "mi", UnitCategory::Length, 1609.344, 0.0));
        self.add(Unit::new("yard", "yd", UnitCategory::Length, 0.9144, 0.0));
        self.add(Unit::new("foot", "ft", UnitCategory::Length, 0.3048, 0.0));
        self.add(Unit::new("inch", "in", UnitCategory::Length, 0.0254, 0.0));
        self.add(Unit::new(
            "nautical_mile",
            "nmi",
            UnitCategory::Length,
            1852.0,
            0.0,
        ));

        // Mass (base: kilogram)
        self.add(Unit::new("kilogram", "kg", UnitCategory::Mass, 1.0, 0.0));
        self.add(Unit::new("gram", "g", UnitCategory::Mass, 0.001, 0.0));
        self.add(Unit::new(
            "milligram",
            "mg",
            UnitCategory::Mass,
            0.000001,
            0.0,
        ));
        self.add(Unit::new("pound", "lb", UnitCategory::Mass, 0.453592, 0.0));
        self.add(Unit::new("ounce", "oz", UnitCategory::Mass, 0.0283495, 0.0));
        self.add(Unit::new("ton", "t", UnitCategory::Mass, 1000.0, 0.0));
        self.add(Unit::new("stone", "st", UnitCategory::Mass, 6.35029, 0.0));

        // Temperature (base: celsius, using offset conversion)
        // For temperature: base_value = (value + to_base_offset) * to_base_factor
        // Celsius is the base: factor=1, offset=0
        // Fahrenheit: C = (F - 32) * 5/9 => offset=-32, factor=5/9
        // Kelvin: C = K - 273.15 => offset=-273.15, factor=1
        self.add(Unit::new(
            "celsius",
            "C",
            UnitCategory::Temperature,
            1.0,
            0.0,
        ));
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
        self.add(Unit::new(
            "kilobyte",
            "KB",
            UnitCategory::DataSize,
            1024.0,
            0.0,
        ));
        self.add(Unit::new(
            "megabyte",
            "MB",
            UnitCategory::DataSize,
            1_048_576.0,
            0.0,
        ));
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
        self.add(Unit::new(
            "meters_per_second",
            "m/s",
            UnitCategory::Speed,
            1.0,
            0.0,
        ));
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
        self.add(Unit::new(
            "square_meter",
            "m2",
            UnitCategory::Area,
            1.0,
            0.0,
        ));
        self.add(Unit::new(
            "square_kilometer",
            "km2",
            UnitCategory::Area,
            1_000_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "hectare",
            "ha",
            UnitCategory::Area,
            10_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "acre",
            "ac",
            UnitCategory::Area,
            4046.8564224,
            0.0,
        ));
        self.add(Unit::new(
            "square_foot",
            "ft2",
            UnitCategory::Area,
            0.092903,
            0.0,
        ));

        // Volume (base: liter)
        self.add(Unit::new("liter", "L", UnitCategory::Volume, 1.0, 0.0));
        self.add(Unit::new(
            "milliliter",
            "mL",
            UnitCategory::Volume,
            0.001,
            0.0,
        ));
        self.add(Unit::new(
            "gallon",
            "gal",
            UnitCategory::Volume,
            3.78541,
            0.0,
        ));
        self.add(Unit::new(
            "quart",
            "qt",
            UnitCategory::Volume,
            0.946353,
            0.0,
        ));
        self.add(Unit::new("pint", "pt", UnitCategory::Volume, 0.473176, 0.0));
        self.add(Unit::new("cup", "cup", UnitCategory::Volume, 0.236588, 0.0));
        self.add(Unit::new(
            "tablespoon",
            "tbsp",
            UnitCategory::Volume,
            0.0147868,
            0.0,
        ));
        self.add(Unit::new(
            "teaspoon",
            "tsp",
            UnitCategory::Volume,
            0.00492892,
            0.0,
        ));

        // Energy (base: joule)
        self.add(Unit::new("joule", "J", UnitCategory::Energy, 1.0, 0.0));
        self.add(Unit::new(
            "kilojoule",
            "kJ",
            UnitCategory::Energy,
            1000.0,
            0.0,
        ));
        self.add(Unit::new(
            "calorie",
            "cal",
            UnitCategory::Energy,
            4.184,
            0.0,
        ));
        self.add(Unit::new(
            "kilocalorie",
            "kcal",
            UnitCategory::Energy,
            4184.0,
            0.0,
        ));
        self.add(Unit::new(
            "watt_hour",
            "Wh",
            UnitCategory::Energy,
            3600.0,
            0.0,
        ));
        self.add(Unit::new(
            "kilowatt_hour",
            "kWh",
            UnitCategory::Energy,
            3_600_000.0,
            0.0,
        ));
        self.add(Unit::new("btu", "BTU", UnitCategory::Energy, 1055.06, 0.0));
        self.add(Unit::new(
            "electronvolt",
            "eV",
            UnitCategory::Energy,
            1.602176634e-19,
            0.0,
        ));

        // Pressure (base: pascal)
        self.add(Unit::new("pascal", "Pa", UnitCategory::Pressure, 1.0, 0.0));
        self.add(Unit::new(
            "kilopascal",
            "kPa",
            UnitCategory::Pressure,
            1000.0,
            0.0,
        ));
        self.add(Unit::new(
            "bar",
            "bar",
            UnitCategory::Pressure,
            100_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "atmosphere",
            "atm",
            UnitCategory::Pressure,
            101_325.0,
            0.0,
        ));
        self.add(Unit::new(
            "psi",
            "psi",
            UnitCategory::Pressure,
            6894.76,
            0.0,
        ));
        self.add(Unit::new(
            "mmhg",
            "mmHg",
            UnitCategory::Pressure,
            133.322,
            0.0,
        ));
        self.add(Unit::new(
            "torr",
            "torr",
            UnitCategory::Pressure,
            133.322,
            0.0,
        ));

        // Angle (base: radian)
        self.add(Unit::new("radian", "rad", UnitCategory::Angle, 1.0, 0.0));
        self.add(Unit::new(
            "degree",
            "deg",
            UnitCategory::Angle,
            std::f64::consts::PI / 180.0,
            0.0,
        ));
        self.add(Unit::new(
            "gradian",
            "grad",
            UnitCategory::Angle,
            std::f64::consts::PI / 200.0,
            0.0,
        ));
        self.add(Unit::new(
            "arcminute",
            "arcmin",
            UnitCategory::Angle,
            std::f64::consts::PI / 10800.0,
            0.0,
        ));
        self.add(Unit::new(
            "arcsecond",
            "arcsec",
            UnitCategory::Angle,
            std::f64::consts::PI / 648000.0,
            0.0,
        ));

        // Frequency (base: hertz)
        self.add(Unit::new("hertz", "Hz", UnitCategory::Frequency, 1.0, 0.0));
        self.add(Unit::new(
            "kilohertz",
            "kHz",
            UnitCategory::Frequency,
            1_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "megahertz",
            "MHz",
            UnitCategory::Frequency,
            1_000_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "gigahertz",
            "GHz",
            UnitCategory::Frequency,
            1_000_000_000.0,
            0.0,
        ));

        // Force (base: newton)
        self.add(Unit::new("newton", "N", UnitCategory::Force, 1.0, 0.0));
        self.add(Unit::new(
            "kilonewton",
            "kN",
            UnitCategory::Force,
            1_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "pound_force",
            "lbf",
            UnitCategory::Force,
            4.44822,
            0.0,
        ));
        self.add(Unit::new("dyne", "dyn", UnitCategory::Force, 0.00001, 0.0));
        self.add(Unit::new(
            "kilogram_force",
            "kgf",
            UnitCategory::Force,
            9.80665,
            0.0,
        ));

        // Power (base: watt)
        self.add(Unit::new("watt", "W", UnitCategory::Power, 1.0, 0.0));
        self.add(Unit::new(
            "kilowatt",
            "kW",
            UnitCategory::Power,
            1_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "megawatt",
            "MW",
            UnitCategory::Power,
            1_000_000.0,
            0.0,
        ));
        self.add(Unit::new(
            "horsepower",
            "hp",
            UnitCategory::Power,
            745.7,
            0.0,
        ));
        self.add(Unit::new(
            "milliwatt",
            "mW",
            UnitCategory::Power,
            0.001,
            0.0,
        ));
    }

    /// Find a unit by name or symbol.
    /// Tries exact symbol match first, then case-insensitive name/symbol, then plurals.
    pub fn find_unit(&self, name_or_symbol: &str) -> Option<&Unit> {
        // Exact symbol match first (important for case-sensitive symbols like mW vs MW)
        for units in self.units.values() {
            for unit in units {
                if unit.symbol == name_or_symbol {
                    return Some(unit);
                }
            }
        }
        // Case-insensitive name or symbol
        let query = name_or_symbol.to_lowercase();
        for units in self.units.values() {
            for unit in units {
                if unit.name.to_lowercase() == query || unit.symbol.to_lowercase() == query {
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

    #[test]
    fn test_joule_to_calorie() {
        let r = reg().convert(1.0, "joule", "calorie").unwrap();
        assert!((r.to_value - 0.239).abs() < 0.001);
    }

    #[test]
    fn test_kwh_to_btu() {
        let r = reg().convert(1.0, "kWh", "BTU").unwrap();
        assert!((r.to_value - 3412.14).abs() < 0.1);
    }

    #[test]
    fn test_atm_to_psi() {
        let r = reg().convert(1.0, "atm", "psi").unwrap();
        assert!((r.to_value - 14.696).abs() < 0.001);
    }

    #[test]
    fn test_bar_to_pascal() {
        let r = reg().convert(1.0, "bar", "Pa").unwrap();
        assert!((r.to_value - 100_000.0).abs() < 0.1);
    }

    #[test]
    fn test_degrees_to_radians() {
        let r = reg().convert(180.0, "degree", "radian").unwrap();
        assert!((r.to_value - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_radians_to_degrees() {
        let r = reg().convert(std::f64::consts::PI, "rad", "deg").unwrap();
        assert!((r.to_value - 180.0).abs() < 0.001);
    }

    #[test]
    fn test_mhz_to_ghz() {
        let r = reg().convert(1000.0, "MHz", "GHz").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_hz_to_khz() {
        let r = reg().convert(44100.0, "Hz", "kHz").unwrap();
        assert!((r.to_value - 44.1).abs() < 0.001);
    }

    #[test]
    fn test_newton_to_lbf() {
        let r = reg().convert(1.0, "newton", "lbf").unwrap();
        assert!((r.to_value - 0.22481).abs() < 0.001);
    }

    #[test]
    fn test_kw_to_hp() {
        let r = reg().convert(1.0, "kW", "hp").unwrap();
        assert!((r.to_value - 1.341).abs() < 0.01);
    }

    #[test]
    fn test_hp_to_watt() {
        let r = reg().convert(1.0, "hp", "W").unwrap();
        assert!((r.to_value - 745.7).abs() < 0.1);
    }

    // --- Length additional ---
    #[test]
    fn test_inches_to_cm() {
        let r = reg().convert(1.0, "inch", "cm").unwrap();
        assert!((r.to_value - 2.54).abs() < 0.01);
    }

    #[test]
    fn test_yards_to_meters() {
        let r = reg().convert(1.0, "yard", "meter").unwrap();
        assert!((r.to_value - 0.9144).abs() < 0.001);
    }

    #[test]
    fn test_nautical_mile_to_km() {
        let r = reg().convert(1.0, "nautical_mile", "km").unwrap();
        assert!((r.to_value - 1.852).abs() < 0.001);
    }

    // --- Mass additional ---
    #[test]
    fn test_ounces_to_grams() {
        let r = reg().convert(1.0, "ounce", "gram").unwrap();
        assert!((r.to_value - 28.3495).abs() < 0.01);
    }

    #[test]
    fn test_stone_to_kg() {
        let r = reg().convert(1.0, "stone", "kg").unwrap();
        assert!((r.to_value - 6.35029).abs() < 0.01);
    }

    #[test]
    fn test_ton_to_kg() {
        let r = reg().convert(1.0, "ton", "kg").unwrap();
        assert!((r.to_value - 1000.0).abs() < 0.1);
    }

    // --- Temperature additional ---
    #[test]
    fn test_kelvin_to_fahrenheit() {
        let r = reg().convert(373.15, "kelvin", "fahrenheit").unwrap();
        assert!((r.to_value - 212.0).abs() < 0.1);
    }

    #[test]
    fn test_absolute_zero_kelvin() {
        let r = reg().convert(0.0, "kelvin", "celsius").unwrap();
        assert!((r.to_value - (-273.15)).abs() < 0.1);
    }

    // --- Time additional ---
    #[test]
    fn test_days_to_hours() {
        let r = reg().convert(1.0, "day", "hour").unwrap();
        assert!((r.to_value - 24.0).abs() < 0.1);
    }

    #[test]
    fn test_weeks_to_days() {
        let r = reg().convert(1.0, "week", "day").unwrap();
        assert!((r.to_value - 7.0).abs() < 0.1);
    }

    #[test]
    fn test_year_to_days() {
        let r = reg().convert(1.0, "year", "day").unwrap();
        assert!((r.to_value - 365.25).abs() < 0.1);
    }

    // --- Data size additional ---
    #[test]
    fn test_tb_to_gb() {
        let r = reg().convert(1.0, "TB", "GB").unwrap();
        assert!((r.to_value - 1024.0).abs() < 0.1);
    }

    #[test]
    fn test_mb_to_kb() {
        let r = reg().convert(1.0, "MB", "KB").unwrap();
        assert!((r.to_value - 1024.0).abs() < 0.1);
    }

    // --- Speed additional ---
    #[test]
    fn test_kmh_to_mph() {
        let r = reg().convert(100.0, "km/h", "mph").unwrap();
        assert!((r.to_value - 62.137).abs() < 0.01);
    }

    #[test]
    fn test_knot_to_kmh() {
        let r = reg().convert(1.0, "knot", "km/h").unwrap();
        assert!((r.to_value - 1.852).abs() < 0.01);
    }

    // --- Area additional ---
    #[test]
    fn test_hectare_to_acres() {
        let r = reg().convert(1.0, "hectare", "acre").unwrap();
        assert!((r.to_value - 2.471).abs() < 0.01);
    }

    #[test]
    fn test_sqkm_to_hectare() {
        let r = reg().convert(1.0, "km2", "ha").unwrap();
        assert!((r.to_value - 100.0).abs() < 0.1);
    }

    // --- Volume additional ---
    #[test]
    fn test_cups_to_ml() {
        let r = reg().convert(1.0, "cup", "mL").unwrap();
        assert!((r.to_value - 236.588).abs() < 1.0);
    }

    #[test]
    fn test_tablespoon_to_teaspoon() {
        let r = reg().convert(1.0, "tablespoon", "teaspoon").unwrap();
        assert!((r.to_value - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_pint_to_cups() {
        let r = reg().convert(1.0, "pint", "cup").unwrap();
        assert!((r.to_value - 2.0).abs() < 0.1);
    }

    // --- Energy additional ---
    #[test]
    fn test_kcal_to_kj() {
        let r = reg().convert(1.0, "kcal", "kJ").unwrap();
        assert!((r.to_value - 4.184).abs() < 0.01);
    }

    #[test]
    fn test_wh_to_joule() {
        let r = reg().convert(1.0, "Wh", "J").unwrap();
        assert!((r.to_value - 3600.0).abs() < 0.1);
    }

    // --- Pressure additional ---
    #[test]
    fn test_kpa_to_atm() {
        let r = reg().convert(101.325, "kPa", "atm").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_mmhg_to_torr() {
        // mmHg and torr are essentially the same
        let r = reg().convert(1.0, "mmhg", "torr").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.01);
    }

    // --- Angle additional ---
    #[test]
    fn test_full_circle_degrees_to_rad() {
        let r = reg().convert(360.0, "degree", "radian").unwrap();
        assert!((r.to_value - std::f64::consts::TAU).abs() < 0.001);
    }

    #[test]
    fn test_gradian_to_degree() {
        let r = reg().convert(100.0, "gradian", "degree").unwrap();
        assert!((r.to_value - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_arcminute_to_degree() {
        let r = reg().convert(60.0, "arcminute", "degree").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_arcsecond_to_arcminute() {
        let r = reg().convert(60.0, "arcsecond", "arcminute").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    // --- Frequency additional ---
    #[test]
    fn test_ghz_to_hz() {
        let r = reg().convert(1.0, "GHz", "Hz").unwrap();
        assert!((r.to_value - 1e9).abs() < 1.0);
    }

    #[test]
    fn test_khz_to_mhz() {
        let r = reg().convert(1000.0, "kHz", "MHz").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    // --- Force additional ---
    #[test]
    fn test_kgf_to_newton() {
        let r = reg().convert(1.0, "kgf", "N").unwrap();
        assert!((r.to_value - 9.80665).abs() < 0.001);
    }

    #[test]
    fn test_dyne_to_newton() {
        let r = reg().convert(100000.0, "dyne", "N").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_kn_to_newton() {
        let r = reg().convert(1.0, "kN", "N").unwrap();
        assert!((r.to_value - 1000.0).abs() < 0.1);
    }

    // --- Power additional ---
    #[test]
    fn test_mw_to_watt() {
        let r = reg().convert(1.0, "MW", "W").unwrap();
        assert!((r.to_value - 1_000_000.0).abs() < 1.0);
    }

    #[test]
    fn test_milliwatt_to_watt() {
        let r = reg().convert(1000.0, "mW", "W").unwrap();
        assert!((r.to_value - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_hp_to_kw() {
        let r = reg().convert(1.0, "hp", "kW").unwrap();
        assert!((r.to_value - 0.7457).abs() < 0.001);
    }

    // --- Cross-category error ---
    #[test]
    fn test_angle_vs_frequency_incompatible() {
        let result = reg().convert(1.0, "degree", "Hz");
        assert!(result.is_err());
    }

    #[test]
    fn test_force_vs_power_incompatible() {
        let result = reg().convert(1.0, "newton", "watt");
        assert!(result.is_err());
    }
}
