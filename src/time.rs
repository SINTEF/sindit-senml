//! # Convert time value between SenML time values and `chrono::DateTime<Utc>`.
//!
//! The time value can be either an absolute time or a relative time.
//! Absolute times are represented as a number of seconds since the Unix epoch.
//! Relative times are represented as a number of seconds relative to the current time.
//! Relative times can be negative.
//!
//! # Please note that SenML relies on floating point precision to represent subsecond precision.
//! This is not precise and will result in wrong values at nano second precision.
//! This is a limitation of the SenML specification.

use chrono::{DateTime, Duration, Utc};

// 2**28
const TIME_THRESHOLD: f64 = 268_435_456.0;

/// Convert a SenML time value to a `DateTime<Utc>`.
///
/// # Arguments
/// * `seconds` - The time value to parse.
/// * `now` - The current time.
/// # Returns
/// * `Some(DateTime<Utc>)` - The parsed time.
/// * `None` - The time value could not be parsed.
/// # Examples
/// ```
/// use chrono::{DateTime, Utc};
/// use sindit_senml::time::convert_senml_time;
/// let now = Utc::now();
/// let relative_time = -10.0;
/// let result = convert_senml_time(relative_time, now);
/// assert!(result.is_some());
/// ```
pub fn convert_senml_time(seconds: f64, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    // Check if seconds is a valid time value (not NaN or infinity)
    if !seconds.is_finite() {
        return None;
    }

    // Values greater than or equal to 2**28 represent an absolute time relative to the Unix epoch.
    // Values less than 2**28 represent time relative to the current time.

    // Split seconds into whole seconds and nanoseconds
    let whole_seconds = seconds.trunc() as i64;
    let frac_seconds = seconds.fract();

    let nanoseconds = if frac_seconds != 0.0 {
        (seconds.fract() * 1_000_000_000_f64).trunc() as i64
    } else {
        0 as i64
    };

    // Timestamp
    if seconds >= TIME_THRESHOLD {
        return DateTime::<Utc>::from_timestamp(whole_seconds, nanoseconds as u32);
    }

    // Relative time to now
    return Some(now + Duration::seconds(whole_seconds) + Duration::nanoseconds(nanoseconds));
}

/// Convert a `DateTime<Utc>` to a Unix timestamp.
///
/// The Unix timestamp is the number of seconds since the Unix epoch.
/// The Unix timestamp can be a floating point number to represent subsecond precision.
///
/// # Arguments
/// * `datetime` - The `DateTime<Utc>` to convert.
/// # Returns
/// * `i64` - The Unix timestamp.
/// * `Option<f64>` - The Unix timestamp with subsecond precision if necessary.
/// # Examples
/// ```
/// use chrono::{DateTime, Utc};
/// use sindit_senml::time::datetime_to_timestamp;
/// let datetime = DateTime::<Utc>::from_timestamp(1234567890, 123456789).unwrap();
/// let (timestamp, precise_timestamp) = datetime_to_timestamp(&datetime);
/// assert_eq!(timestamp, 1234567890);
/// assert_eq!(precise_timestamp, Some(1234567890.1234567890f64));
/// ```
pub fn datetime_to_timestamp(datetime: &DateTime<Utc>) -> (i64, Option<f64>) {
    let timestamp = datetime.timestamp();
    let nanos = datetime.timestamp_subsec_nanos();
    if nanos > 0 {
        let nanos = nanos as f64 / 1_000_000_000f64; // Convert nanoseconds to fraction of a second
        (timestamp, Some(timestamp as f64 + nanos))
    } else {
        (timestamp, None)
    }
}

#[cfg(test)]
mod tests {
    use super::convert_senml_time;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_absolute_time() {
        let time = 1320078429;
        let expected = DateTime::<Utc>::from_timestamp(time, 0);
        let result = convert_senml_time(time as f64, Utc::now());
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected.unwrap());
    }

    #[test]
    fn test_absolute_subseconds_time() {
        let time = 1234567890.1234567890f64;
        let expected = DateTime::<Utc>::from_timestamp(1234567890, 123456716);
        let result = convert_senml_time(time, Utc::now());
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected.unwrap());
    }

    #[test]
    fn test_relative_time() {
        let time = 10;
        let now = DateTime::<Utc>::from_timestamp(10_0000, 0).unwrap();
        let expected = now + chrono::Duration::seconds(time);
        let result = convert_senml_time(time as f64, now);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_negative_relative_time() {
        let time = -10;
        let now = Utc::now();
        let expected = now - chrono::Duration::seconds(-time);
        let result = convert_senml_time(time as f64, now);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_relative_subseconds_time() {
        let time = -10.1234567890f64;
        let now = DateTime::<Utc>::from_timestamp(10_0000, 0).unwrap();
        let expected =
            now - chrono::Duration::seconds(10) - chrono::Duration::nanoseconds(123456789);
        let result = convert_senml_time(time, now);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_datetime_to_timestamp() {
        let datetime = DateTime::<Utc>::from_timestamp(1234567890, 123456789).unwrap();

        let result = super::datetime_to_timestamp(&datetime);
        let (timestamp, precise_timestamp) = result;
        assert_eq!(timestamp, 1234567890);
        assert_eq!(precise_timestamp, Some(timestamp as f64 + 0.1234567890f64));

        // SenML rely on floating point precision to represent subsecond precision.
        // This is not very precise and 0.123456789 gets transformed to 0.123456716
        let result = convert_senml_time(1234567890.123456789, Utc::now());
        let unprecise_datetime = DateTime::<Utc>::from_timestamp(1234567890, 123456716).unwrap();
        assert_eq!(result.unwrap(), unprecise_datetime);
    }
}
