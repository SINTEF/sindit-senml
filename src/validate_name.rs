//! # Validate SenML Name
//!
//! The Name value is concatenated to the Base Name value to yield the
//! name of the sensor.  The resulting concatenated name needs to
//! uniquely identify and differentiate the sensor from all others.  The
//! concatenated name MUST consist only of characters out of the set "A"
//! to "Z", "a" to "z", and "0" to "9", as well as "-", ":", ".", "/",
//! and "_"; furthermore, it MUST start with a character out of the set
//! "A" to "Z", "a" to "z", or "0" to "9".
use once_cell::sync::OnceCell;
use regex::Regex;

// Put the Regex in an OnceCell so it is only compiled once
static PATTERN: OnceCell<Regex> = OnceCell::new();

/// Validate a name according to the SenML specifications.
///
///
/// # Arguments
/// * `name` - The name to validate
/// # Output
/// * `bool` - True if the name is valid, false otherwise
/// # Example
/// ```
/// use sindit_senml::validate_name::validate_name;
/// validate_name("Sensor1"); // true
/// validate_name("sensor-name"); // true
/// validate_name("123Sensor"); // true
/// validate_name(""); // false
/// validate_name("-sensor"); // false
/// ```
pub fn validate_name(name: &str) -> bool {
    // Check if the name matches the pattern using the static regex
    PATTERN
        .get_or_init(|| Regex::new(r"^[A-Za-z0-9][A-Za-z0-9\-\:\.\/_]*$").unwrap())
        .is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(validate_name("Sensor1"));
        assert!(validate_name("sensor-name"));
        assert!(validate_name("123Sensor"));
        assert!(validate_name("sensor_123"));
        assert!(validate_name("sensor.name/1"));
    }

    #[test]
    fn test_invalid_names() {
        assert!(!validate_name("")); // Empty string
        assert!(!validate_name("-sensor")); // Starts with a non-alphanumeric character
        assert!(!validate_name(".name")); // Starts with a non-alphanumeric character
        assert!(!validate_name("sensor name")); // Contains a space
        assert!(!validate_name("sensor@name")); // Contains an invalid character
        assert!(!validate_name("センサー")); // Contains non-Latin characters
    }
}
