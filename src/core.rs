//! Core types — numeric values, units, categories, and conversion results.
//!
//! These are the foundational data types shared across all abaco modules:
//! [`Value`] (polymorphic numeric result), [`Unit`] and [`UnitCategory`]
//! (physical measurement), [`ConversionResult`] (conversion output),
//! and [`Currency`] (monetary unit).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Core numeric value type for Abaco.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Fraction(i64, i64),
    Complex(f64, f64),
    Text(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Value::Fraction(num, den) => write!(f, "{num}/{den}"),
            Value::Complex(re, im) => {
                if *im >= 0.0 {
                    write!(f, "{re}+{im}i")
                } else {
                    write!(f, "{re}{im}i")
                }
            }
            Value::Text(s) => write!(f, "{s}"),
        }
    }
}

impl Value {
    /// Convert to f64 for arithmetic.
    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Integer(n) => Some(*n as f64),
            Value::Float(n) => Some(*n),
            Value::Fraction(num, den) => {
                if *den == 0 {
                    None
                } else {
                    Some(*num as f64 / *den as f64)
                }
            }
            _ => None,
        }
    }
}

/// Category of physical unit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitCategory {
    Length,
    Mass,
    Temperature,
    Time,
    DataSize,
    Speed,
    Area,
    Volume,
    Energy,
    Pressure,
    Angle,
    Frequency,
    Force,
    Power,
    FuelEconomy,
    Density,
    Luminosity,
    Viscosity,
}

impl UnitCategory {
    /// Returns a slice of all `UnitCategory` variants.
    #[must_use]
    pub fn all_categories() -> &'static [UnitCategory] {
        &[
            UnitCategory::Length,
            UnitCategory::Mass,
            UnitCategory::Temperature,
            UnitCategory::Time,
            UnitCategory::DataSize,
            UnitCategory::Speed,
            UnitCategory::Area,
            UnitCategory::Volume,
            UnitCategory::Energy,
            UnitCategory::Pressure,
            UnitCategory::Angle,
            UnitCategory::Frequency,
            UnitCategory::Force,
            UnitCategory::Power,
            UnitCategory::FuelEconomy,
            UnitCategory::Density,
            UnitCategory::Luminosity,
            UnitCategory::Viscosity,
        ]
    }
}

impl std::str::FromStr for UnitCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "length" => Ok(UnitCategory::Length),
            "mass" | "weight" => Ok(UnitCategory::Mass),
            "temperature" | "temp" => Ok(UnitCategory::Temperature),
            "time" => Ok(UnitCategory::Time),
            "datasize" | "data_size" | "data size" | "data" => Ok(UnitCategory::DataSize),
            "speed" | "velocity" => Ok(UnitCategory::Speed),
            "area" => Ok(UnitCategory::Area),
            "volume" => Ok(UnitCategory::Volume),
            "energy" => Ok(UnitCategory::Energy),
            "pressure" => Ok(UnitCategory::Pressure),
            "angle" => Ok(UnitCategory::Angle),
            "frequency" | "freq" => Ok(UnitCategory::Frequency),
            "force" => Ok(UnitCategory::Force),
            "power" | "wattage" => Ok(UnitCategory::Power),
            "fueleconomy" | "fuel_economy" | "fuel economy" | "fuel" | "mpg" => {
                Ok(UnitCategory::FuelEconomy)
            }
            "density" => Ok(UnitCategory::Density),
            "luminosity" | "light" | "illuminance" => Ok(UnitCategory::Luminosity),
            "viscosity" => Ok(UnitCategory::Viscosity),
            _ => Err(format!("unknown unit category: '{s}'")),
        }
    }
}

impl fmt::Display for UnitCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitCategory::Length => write!(f, "Length"),
            UnitCategory::Mass => write!(f, "Mass"),
            UnitCategory::Temperature => write!(f, "Temperature"),
            UnitCategory::Time => write!(f, "Time"),
            UnitCategory::DataSize => write!(f, "Data Size"),
            UnitCategory::Speed => write!(f, "Speed"),
            UnitCategory::Area => write!(f, "Area"),
            UnitCategory::Volume => write!(f, "Volume"),
            UnitCategory::Energy => write!(f, "Energy"),
            UnitCategory::Pressure => write!(f, "Pressure"),
            UnitCategory::Angle => write!(f, "Angle"),
            UnitCategory::Frequency => write!(f, "Frequency"),
            UnitCategory::Force => write!(f, "Force"),
            UnitCategory::Power => write!(f, "Power"),
            UnitCategory::FuelEconomy => write!(f, "Fuel Economy"),
            UnitCategory::Density => write!(f, "Density"),
            UnitCategory::Luminosity => write!(f, "Luminosity"),
            UnitCategory::Viscosity => write!(f, "Viscosity"),
        }
    }
}

/// A physical or digital unit of measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub symbol: String,
    pub category: UnitCategory,
    /// Multiply by this factor to convert to the base unit of the category.
    pub to_base_factor: f64,
    /// Add this offset after multiplication (used for temperature).
    pub to_base_offset: f64,
    /// If true, conversion is reciprocal: `base = factor / value` (used for fuel economy).
    pub to_base_inverse: bool,
}

impl Unit {
    #[must_use]
    pub fn new(
        name: &str,
        symbol: &str,
        category: UnitCategory,
        to_base_factor: f64,
        to_base_offset: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            category,
            to_base_factor,
            to_base_offset,
            to_base_inverse: false,
        }
    }

    /// Create a unit with reciprocal conversion (e.g., L/100km where base = factor / value).
    #[must_use]
    pub fn new_inverse(
        name: &str,
        symbol: &str,
        category: UnitCategory,
        to_base_factor: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            category,
            to_base_factor,
            to_base_offset: 0.0,
            to_base_inverse: true,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.symbol)
    }
}

/// Result of a unit conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub from_value: f64,
    pub from_unit: String,
    pub to_value: f64,
    pub to_unit: String,
}

impl fmt::Display for ConversionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} = {} {}",
            self.from_value, self.from_unit, self.to_value, self.to_unit
        )
    }
}

/// A currency for conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

impl Currency {
    #[must_use]
    pub fn new(code: &str, name: &str, symbol: &str) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) {}", self.code, self.symbol, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display_integer() {
        assert_eq!(Value::Integer(42).to_string(), "42");
    }

    #[test]
    fn test_value_display_float() {
        assert_eq!(
            Value::Float(std::f64::consts::PI).to_string(),
            "3.141592653589793"
        );
    }

    #[test]
    fn test_value_display_float_whole() {
        assert_eq!(Value::Float(7.0).to_string(), "7.0");
    }

    #[test]
    fn test_value_display_fraction() {
        assert_eq!(Value::Fraction(1, 3).to_string(), "1/3");
    }

    #[test]
    fn test_value_display_complex_positive_im() {
        assert_eq!(Value::Complex(3.0, 4.0).to_string(), "3+4i");
    }

    #[test]
    fn test_value_display_complex_negative_im() {
        assert_eq!(Value::Complex(3.0, -4.0).to_string(), "3-4i");
    }

    #[test]
    fn test_value_display_text() {
        assert_eq!(Value::Text("hello".to_string()).to_string(), "hello");
    }

    #[test]
    fn test_value_as_f64_integer() {
        assert_eq!(Value::Integer(5).as_f64(), Some(5.0));
    }

    #[test]
    fn test_value_as_f64_fraction() {
        assert_eq!(Value::Fraction(1, 2).as_f64(), Some(0.5));
    }

    #[test]
    fn test_value_as_f64_fraction_zero_den() {
        assert_eq!(Value::Fraction(1, 0).as_f64(), None);
    }

    #[test]
    fn test_unit_creation() {
        let u = Unit::new("meter", "m", UnitCategory::Length, 1.0, 0.0);
        assert_eq!(u.name, "meter");
        assert_eq!(u.symbol, "m");
        assert_eq!(u.category, UnitCategory::Length);
    }

    #[test]
    fn test_unit_display() {
        let u = Unit::new("kilogram", "kg", UnitCategory::Mass, 1.0, 0.0);
        assert_eq!(u.to_string(), "kilogram (kg)");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(UnitCategory::Temperature.to_string(), "Temperature");
        assert_eq!(UnitCategory::DataSize.to_string(), "Data Size");
    }

    #[test]
    fn test_currency_creation() {
        let c = Currency::new("USD", "US Dollar", "$");
        assert_eq!(c.code, "USD");
        assert_eq!(c.symbol, "$");
    }

    #[test]
    fn test_conversion_result_display() {
        let r = ConversionResult {
            from_value: 5.0,
            from_unit: "km".to_string(),
            to_value: 3.10686,
            to_unit: "mi".to_string(),
        };
        assert!(r.to_string().contains("5 km"));
    }

    #[test]
    fn test_value_float_large_uses_scientific() {
        // Values >= 1e15 should not use ".0" formatting
        let s = Value::Float(1e16).to_string();
        assert!(!s.contains(".0") || s.contains("e"));
    }

    #[test]
    fn test_value_complex_as_f64_none() {
        assert_eq!(Value::Complex(1.0, 2.0).as_f64(), None);
    }

    #[test]
    fn test_value_text_as_f64_none() {
        assert_eq!(Value::Text("hello".into()).as_f64(), None);
    }

    #[test]
    fn test_unit_category_from_str_aliases() {
        use std::str::FromStr;
        assert_eq!(
            UnitCategory::from_str("weight").unwrap(),
            UnitCategory::Mass
        );
        assert_eq!(
            UnitCategory::from_str("temp").unwrap(),
            UnitCategory::Temperature
        );
        assert_eq!(
            UnitCategory::from_str("velocity").unwrap(),
            UnitCategory::Speed
        );
        assert_eq!(
            UnitCategory::from_str("wattage").unwrap(),
            UnitCategory::Power
        );
        assert_eq!(
            UnitCategory::from_str("freq").unwrap(),
            UnitCategory::Frequency
        );
        assert_eq!(
            UnitCategory::from_str("data").unwrap(),
            UnitCategory::DataSize
        );
        assert_eq!(
            UnitCategory::from_str("data size").unwrap(),
            UnitCategory::DataSize
        );
        assert_eq!(
            UnitCategory::from_str("data_size").unwrap(),
            UnitCategory::DataSize
        );
    }

    #[test]
    fn test_unit_category_from_str_unknown() {
        use std::str::FromStr;
        assert!(UnitCategory::from_str("unicorn").is_err());
    }

    #[test]
    fn test_unit_category_all_categories_count() {
        assert_eq!(UnitCategory::all_categories().len(), 18);
    }

    #[test]
    fn test_currency_display() {
        let c = Currency::new("EUR", "Euro", "€");
        assert_eq!(c.to_string(), "EUR (€) Euro");
    }

    #[test]
    fn test_value_float_as_f64() {
        assert_eq!(
            Value::Float(std::f64::consts::PI).as_f64(),
            Some(std::f64::consts::PI)
        );
    }
}
