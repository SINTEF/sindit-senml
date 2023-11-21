//! # SINDIT SenML
//!
//! This library implements the [SenML RFC8428](https://www.rfc-editor.org/rfc/rfc8428.html) proposed standard.
//!
//! SenML (Sensor Markup Language) is a format for representing sensor data.
//!
//! Only the JSON representation is implemented.
//!
//! # Examples:
//!
//! ```
//! use sindit_senml::parse_json;
//!
//! let json_str = r#"[{"n": "temperature", "v": 42.0}]"#;
//! let records = parse_json(json_str, None).unwrap();
//! assert_eq!(records[0].name, "temperature");
//! assert_eq!(records[0].get_float_value(), Some(42.0));
//! ```
//!
//! ```
//! use sindit_senml::SenMLResolvedRecord;
//!
//! let record = SenMLResolvedRecord {
//!     name: "temperature".to_string(),
//!     unit: Some("Cel".to_string()),
//!     value: Some(sindit_senml::SenMLValueField::FloatingPoint(42f64)),
//!     sum: None,
//!     time: chrono::DateTime::<chrono::Utc>::from_timestamp(1234567890, 0).unwrap(),
//!     update_time: None,
//!     base_version: None,
//!     extra_fields: None,
//! };
//! let json = serde_json::to_string(&vec![record]).unwrap();
//! assert_eq!(
//!     json,
//!     r#"[{"n":"temperature","u":"Cel","v":42,"t":1234567890}]"#
//! );
//! ```
//!
use std::collections::HashMap;

use base64::Engine;
use chrono::{DateTime, Utc};
use serde::ser::{SerializeStruct, Serializer};
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use time::datetime_to_timestamp;
use validate_name::validate_name;

#[cfg(feature = "stream")]
mod stream;

pub mod time;
pub mod validate_name;

/// SINDIT SenML Error.
///
/// This represent the common errors that can happen when using this library.
/// The library is not supposed to panic, but instead return an error.
#[derive(Error, Debug)]
pub enum SinditSenMLError {
    #[error("Invalid JSON")]
    InvalidJSON(#[from] serde_json::Error),
    #[error("Invalid name")]
    InvalidName,
    #[error("Invalid time")]
    InvalidTime,
    #[error("Missing name in record at index {0}")]
    MissingName(usize),
    #[error("Invalid name in record named at index {0}")]
    InvalidNameInRecord(usize),
    #[error("Invalid time in record at index {0}")]
    InvalidTimeInRecord(usize),
    #[error("All records must have the same version number")]
    DifferentBaseVersion,
    #[error("Only one kind of value per record at index {0}")]
    OnlyOneValuePerRecord(usize),
    #[error("Invalid base64 value in record at index {0}")]
    InvalidBase64Value(#[from] base64::DecodeError),
    #[error("Positive version number required")]
    InvalidVersionNumber,
}

#[derive(Deserialize, Debug, Clone)]
struct SenMLRecord {
    #[serde(rename = "bn")]
    base_name: Option<String>,

    #[serde(rename = "bt")]
    base_time: Option<f64>,

    #[serde(rename = "bu")]
    base_unit: Option<String>,

    #[serde(rename = "bv")]
    base_value: Option<f64>,

    #[serde(rename = "bs")]
    base_sum: Option<f64>,

    #[serde(rename = "bver")]
    base_version: Option<u64>,

    #[serde(rename = "n")]
    name: Option<String>,

    #[serde(rename = "u")]
    unit: Option<String>,

    #[serde(rename = "v")]
    value: Option<f64>,

    #[serde(rename = "vs")]
    string_value: Option<String>,

    #[serde(rename = "vb")]
    bool_value: Option<bool>,

    #[serde(rename = "vd")]
    data_value: Option<String>,

    #[serde(rename = "s")]
    sum: Option<f64>,

    #[serde(rename = "t")]
    time: Option<f64>,

    #[serde(rename = "ut")]
    update_time: Option<f64>,

    #[serde(flatten, default)]
    extra_fields: Option<HashMap<String, serde_json::Value>>,
}

/// A SenML Value Field.
///
/// SenML can contain multiple types of values:
/// - Floating point
/// - Boolean
/// - String
/// - Data (binary)
///
/// This enum represents all the possible values.
#[derive(Debug, PartialEq, Clone)]
pub enum SenMLValueField {
    BooleanValue(bool),
    StringValue(String),
    DataValue(Vec<u8>),
    FloatingPoint(f64),
}

impl SenMLValueField {
    pub fn as_bool(&self) -> Option<&bool> {
        if let SenMLValueField::BooleanValue(ref value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let SenMLValueField::StringValue(ref value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_data(&self) -> Option<&Vec<u8>> {
        if let SenMLValueField::DataValue(ref value) = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<&f64> {
        if let SenMLValueField::FloatingPoint(ref value) = *self {
            Some(value)
        } else {
            None
        }
    }
}

impl serde::ser::Serialize for SenMLValueField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SenMLValueField", 1)?;
        match *self {
            SenMLValueField::BooleanValue(ref value) => state.serialize_field("vb", value)?,
            SenMLValueField::StringValue(ref value) => state.serialize_field("vs", value)?,
            SenMLValueField::FloatingPoint(ref value) => {
                if value.fract() == 0.0 {
                    state.serialize_field("v", &(*value as i64))?
                } else {
                    state.serialize_field("v", value)?
                }
            }
            SenMLValueField::DataValue(ref value) => state.serialize_field(
                "vd",
                &base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value),
            )?,
        }
        state.end()
    }
}

fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let (timestamp, precise_timestamp) = datetime_to_timestamp(date);
    if let Some(precise_timestamp) = precise_timestamp {
        return serializer.serialize_f64(precise_timestamp);
    }
    serializer.serialize_i64(timestamp)
}

/// SenML Resolved Record.
///
/// A SenML Record that is extracted from a SenML Pack and has all the
/// fields resolved, meaning that all the base fields are applied to the
/// record.
///
/// This can be serialised to JSON using serde.
///
/// Please note that this is not the most compact SenML representation,
/// but it is a compatible one.
/// <https://www.rfc-editor.org/rfc/rfc8428#section-4.6>
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct SenMLResolvedRecord {
    /// The name of the record.
    ///
    /// This is the concatenation of the base name and the name.
    /// The name is always present and cannot be an empty string.
    #[serde(rename = "n")]
    pub name: String,

    /// The unit of the record.
    ///
    /// The unit is optional and is preferably present.
    /// It should be a SI unit if possible and use the units
    /// defined in the SenML unit registries.
    /// <https://www.rfc-editor.org/rfc/rfc8428.html#section-12.1>
    /// <https://www.rfc-editor.org/rfc/rfc8798.html>
    #[serde(rename = "u", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// The value of the record.
    ///
    /// The value is optional as the record can also contain a `sum`.
    /// The value defaults to 0.0 if both the sum and the value are missing.
    #[serde(flatten)]
    pub value: Option<SenMLValueField>,

    /// Integrated sum of the values over time.
    ///
    /// This field should have been named "integral" according to the RFC
    /// but is named "sum" for historical reasons.
    /// Optional.
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,

    /// Time when the value was recorded.
    ///
    /// This is a UTC DateTime that is always present.
    /// It defaults to the current time of the system.
    #[serde(rename = "t", serialize_with = "serialize_datetime")]
    pub time: DateTime<Utc>,

    /// Period of time in seconds that represents the maximum time
    /// before the sensor will provided and updated reading for a measurement.
    ///
    /// Optional. This can be used to detect the failure of sensors or
    /// the communications path from the sensor.
    #[serde(rename = "ut", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,

    /// Version number of the media type format.
    ///
    /// This field is an optional positive integer and defaults to 10 if not present.
    #[serde(rename = "bver", skip_serializing_if = "Option::is_none")]
    pub base_version: Option<u64>,

    /// Extra fields that are not part of the SenML specification but
    /// are allowed to be present and were in the JSON records.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<HashMap<String, serde_json::Value>>,
}

impl SenMLResolvedRecord {
    pub fn get_bool_value(&self) -> Option<bool> {
        self.value.as_ref().and_then(|v| v.as_bool().copied())
    }

    pub fn get_string_value(&self) -> Option<&String> {
        self.value.as_ref().and_then(|v| v.as_string())
    }

    pub fn get_data_value(&self) -> Option<&Vec<u8>> {
        self.value.as_ref().and_then(|v| v.as_data())
    }

    pub fn get_float_value(&self) -> Option<f64> {
        self.value.as_ref().and_then(|v| v.as_float().copied())
    }
}

fn resolve_value(
    record: &SenMLRecord,
    base_value: &Option<f64>,
    index: usize,
) -> Result<Option<SenMLValueField>, SinditSenMLError> {
    match record.value {
        Some(value) => {
            if record.string_value.is_some()
                || record.bool_value.is_some()
                || record.data_value.is_some()
            {
                return Err(SinditSenMLError::OnlyOneValuePerRecord(index));
            }
            match base_value {
                Some(base_value) => Ok(Some(SenMLValueField::FloatingPoint(base_value + value))),
                None => Ok(Some(SenMLValueField::FloatingPoint(value))),
            }
        }
        None => match record.string_value {
            Some(ref value) => {
                if record.bool_value.is_some() || record.data_value.is_some() {
                    return Err(SinditSenMLError::OnlyOneValuePerRecord(index));
                }
                Ok(Some(SenMLValueField::StringValue(value.to_string())))
            }
            None => match record.bool_value {
                Some(ref value) => {
                    if record.data_value.is_some() {
                        return Err(SinditSenMLError::OnlyOneValuePerRecord(index));
                    }
                    Ok(Some(SenMLValueField::BooleanValue(*value)))
                }
                None => match record.data_value {
                    Some(ref value) => {
                        match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(value) {
                            Ok(value) => Ok(Some(SenMLValueField::DataValue(value))),
                            Err(base64_error) => {
                                Err(SinditSenMLError::InvalidBase64Value(base64_error))
                            }
                        }
                    }
                    None => match base_value {
                        Some(base_value) => Ok(Some(SenMLValueField::FloatingPoint(*base_value))),
                        None => Ok(None),
                    },
                },
            },
        },
    }
}

fn resolve_records(
    input_records: &Vec<SenMLRecord>,
    now: DateTime<Utc>,
) -> Result<Vec<SenMLResolvedRecord>, SinditSenMLError> {
    let mut base_name: Option<String> = None;
    let mut base_time: Option<f64> = None;
    let mut base_unit: Option<String> = None;
    let mut base_value: Option<f64> = None;
    let mut base_sum: Option<f64> = None;
    let mut base_version: Option<u64> = None;

    input_records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            if let Some(ref record_base_name) = record.base_name {
                base_name = Some(record_base_name.to_string());
            }

            if let Some(record_base_time) = record.base_time {
                base_time = Some(record_base_time);
            }

            if let Some(ref record_base_unit) = record.base_unit {
                base_unit = Some(record_base_unit.to_string());
            }

            if let Some(record_base_value) = record.base_value {
                base_value = Some(record_base_value);
            }

            if let Some(record_base_sum) = record.base_sum {
                base_sum = Some(record_base_sum);
            }

            match record.base_version {
                Some(record_base_version) => match base_version {
                    Some(base_version) => {
                        if base_version != record_base_version {
                            return Err(SinditSenMLError::DifferentBaseVersion);
                        }
                    }
                    None => {
                        if record_base_version == 0 {
                            return Err(SinditSenMLError::InvalidVersionNumber);
                        }
                        base_version = Some(record_base_version);
                    }
                },
                None => {
                    // We default to 10 if no base version is present.
                    // This is the default in the RFC.
                    if base_version.is_none() {
                        base_version = Some(10);
                    }
                }
            };

            let name = match record.name {
                Some(ref name) => match base_name {
                    Some(ref base_name) => base_name.to_string() + name,
                    None => name.to_string(),
                },
                None => match base_name {
                    Some(ref base_name) => base_name.to_string(),
                    None => return Err(SinditSenMLError::MissingName(index)),
                },
            };

            if !validate_name(&name) {
                return Err(SinditSenMLError::InvalidNameInRecord(index));
            }

            let unit: Option<String> = match record.unit {
                Some(ref unit) => Some(unit.to_string()),
                None => base_unit.clone(),
            };

            let mut value = resolve_value(record, &base_value, index)?;

            let time = match record.time {
                Some(time) => match base_time {
                    Some(base_time) => base_time + time,
                    None => time,
                },
                None => match base_time {
                    Some(base_time) => base_time,
                    None => 0.0,
                },
            };
            let datetime = match time::convert_senml_time(time, now) {
                Some(datetime) => datetime,
                None => return Err(SinditSenMLError::InvalidTimeInRecord(index)),
            };

            let sum = match record.sum {
                Some(sum) => match base_sum {
                    Some(base_sum) => Some(base_sum + sum),
                    None => Some(sum),
                },
                None => match base_sum {
                    Some(base_sum) => Some(base_sum),
                    None => None,
                },
            };

            if value.is_none() && sum.is_none() {
                // return Err(SinditSenMLError::MissingValueOrSum(index));
                // My understanding of the RFC:
                // A sum or a value must be present and never at the same time.
                // Both defaults to 0, but if no base sum or sum are present,
                // then it has to be a value because it is accepted to not have
                // a sum value in the RFC.
                // the default value is 0.
                value = Some(SenMLValueField::FloatingPoint(0.0));
            }

            // Version 10 is the default in SenML.
            // However the RFC says:
            //   The Base Version field MUST NOT be present in resolved Records if the
            //   SenML version defined in this document is used; otherwise, it MUST be
            //   present in all the resolved SenML Records.
            //
            // We interpret this as it must be skipped.
            // let record_base_version = base_version.unwrap_or(10); //
            let record_base_version = match base_version {
                Some(base_version) => match base_version {
                    10 => None,
                    _ => Some(base_version),
                },
                None => None,
            };

            let update_time = record.update_time;

            // skip extra_fields if the record has empty hashmap or None
            let extra_fields = match &record.extra_fields {
                Some(extra_fields) => {
                    if extra_fields.is_empty() {
                        None
                    } else {
                        Some(extra_fields.clone())
                    }
                }
                None => None,
            };

            Ok(SenMLResolvedRecord {
                name,
                unit,
                value,
                sum,
                time: datetime,
                update_time,
                base_version: record_base_version,
                extra_fields,
            })
        })
        .collect()
}

/// Parse SenML JSON and return SenMLResolvedRecords.
///
/// # Arguments
/// * `json_str` - The SenML JSON string to parse.
/// * `now` - The current time. Defaults to current UTC time.
/// # Returns
/// * `Result<Vec<SenMLResolvedRecord>, SinditSenMLError>` - The parsed SenML records.
/// # Examples
/// ```
/// use sindit_senml::parse_json;
///
/// let json_str = r#"[{"n": "temperature", "v": 42.0}]"#;
/// let records = parse_json(json_str, None).unwrap();
/// assert_eq!(records[0].name, "temperature");
/// assert_eq!(records[0].get_float_value(), Some(42.0));
/// ```
///
pub fn parse_json(
    json_str: &str,
    now: Option<DateTime<Utc>>,
) -> Result<Vec<SenMLResolvedRecord>, SinditSenMLError> {
    let records: Vec<SenMLRecord> = match serde_json::from_str(json_str) {
        Ok(records) => records,
        Err(error) => return Err(SinditSenMLError::InvalidJSON(error)),
    };

    resolve_records(&records, now.unwrap_or(Utc::now()))
}

#[cfg(test)]
mod tests {

    use crate::*;

    static EMPTY_RECORD: SenMLRecord = SenMLRecord {
        base_name: None,
        base_time: None,
        base_unit: None,
        base_value: None,
        base_sum: None,
        base_version: None,
        name: None,
        unit: None,
        value: None,
        string_value: None,
        bool_value: None,
        data_value: None,
        sum: None,
        time: None,
        update_time: None,
        extra_fields: None,
    };

    #[test]
    fn test_resolve_value_simple() {
        // None value

        assert!(resolve_value(&EMPTY_RECORD, &None, 0).unwrap().is_none());

        // Floating point value
        let mut record = EMPTY_RECORD.clone();
        record.value = Some(42.0);
        assert_eq!(
            resolve_value(&record, &None, 0,).unwrap().unwrap(),
            SenMLValueField::FloatingPoint(42.0)
        );

        // String value
        let mut record = EMPTY_RECORD.clone();
        record.string_value = Some("Hello world!".to_string());
        assert_eq!(
            resolve_value(&record, &None, 0,).unwrap().unwrap(),
            SenMLValueField::StringValue("Hello world!".to_string())
        );

        // Boolean value true
        let mut record = EMPTY_RECORD.clone();
        record.bool_value = Some(true);
        assert_eq!(
            resolve_value(&record, &None, 0,).unwrap().unwrap(),
            SenMLValueField::BooleanValue(true)
        );

        // Boolean value false
        let mut record = EMPTY_RECORD.clone();
        record.bool_value = Some(false);
        assert_eq!(
            resolve_value(&record, &None, 0,).unwrap().unwrap(),
            SenMLValueField::BooleanValue(false)
        );

        // Base64 string
        let mut record = EMPTY_RECORD.clone();
        record.data_value = Some("SGVsbG8gd29ybGQh".to_string());
        assert_eq!(
            resolve_value(&record, &None, 0,).unwrap().unwrap(),
            SenMLValueField::DataValue("Hello world!".as_bytes().to_vec())
        );
    }

    #[test]
    fn test_resolve_value_base_value() {
        // None value
        let mut record = EMPTY_RECORD.clone();
        record.base_value = Some(10.0); // Ignored
        assert!(resolve_value(&record, &None, 0,).unwrap().is_none());

        assert_eq!(
            resolve_value(
                &record,
                &Some(10.0), // This is the one that matters
                0,
            )
            .unwrap()
            .unwrap(),
            SenMLValueField::FloatingPoint(10.0)
        );

        // Floating point value
        let mut record = EMPTY_RECORD.clone();
        record.base_value = Some(10.0); // Ignored
        record.value = Some(42.0);
        assert_eq!(
            resolve_value(
                &record,
                &Some(32.0), // This is the one that matters
                0,
            )
            .unwrap()
            .unwrap(),
            SenMLValueField::FloatingPoint(74.0)
        );

        // Base value
    }

    #[test]
    fn test_resolve_value_failures() {
        // float and string
        let mut record = EMPTY_RECORD.clone();
        record.value = Some(42.0);
        record.string_value = Some("Hello world!".to_string());
        assert!(resolve_value(&record, &None, 0).is_err());

        // float and bool
        let mut record = EMPTY_RECORD.clone();
        record.value = Some(42.0);
        record.bool_value = Some(true);
        assert!(resolve_value(&record, &None, 0,).is_err());

        // string and bool
        let mut record = EMPTY_RECORD.clone();
        record.string_value = Some("Hello world!".to_string());
        record.bool_value = Some(true);
        assert!(resolve_value(&record, &None, 0,).is_err());

        // float and base64
        let mut record = EMPTY_RECORD.clone();
        record.value = Some(42.0);
        record.data_value = Some("SGVsbG8gd29ybGQh".to_string());
        assert!(resolve_value(&record, &None, 0,).is_err());

        // bool and base64
        let mut record = EMPTY_RECORD.clone();
        record.bool_value = Some(true);
        record.data_value = Some("SGVsbG8gd29ybGQh".to_string());
        assert!(resolve_value(&record, &None, 0,).is_err());

        // Invalid base64
        let mut record = EMPTY_RECORD.clone();
        record.data_value = Some("    ".to_string());
        assert!(resolve_value(&record, &None, 0,).is_err());
    }

    mod test_resolve_records {
        use std::ops::Add;

        use crate::{tests::EMPTY_RECORD, *};
        use lazy_static::lazy_static;

        lazy_static! {
            static ref BASE_RECORD: SenMLRecord = SenMLRecord {
                base_name: Some(String::from("abcd-")),
                base_time: Some(1234567890.0),
                base_unit: Some(String::from("Cel")),
                base_value: Some(10.0),
                base_sum: Some(20.0),
                base_version: Some(10),
                name: None,
                unit: None,
                value: None,
                string_value: None,
                bool_value: None,
                data_value: None,
                sum: None,
                time: None,
                update_time: None,
                extra_fields: None,
            };
            static ref NOW: DateTime<Utc> = Utc::now();
        }

        #[test]
        fn test_empty() {
            assert_eq!(
                Vec::new() as Vec<SenMLResolvedRecord>,
                resolve_records(&Vec::new(), *NOW).unwrap()
            );
        }

        #[test]
        fn test_single_base_recodr() {
            let data = vec![BASE_RECORD.clone()];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(resolved_data.is_ok());
        }

        // Two identical base records
        #[test]
        fn test_two_identical_base_records() {
            let data = vec![BASE_RECORD.clone(), BASE_RECORD.clone()];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(resolved_data.is_ok());
        }

        // Second record uses a different version
        #[test]
        fn test_second_record_uses_different_version() {
            let mut second_record = BASE_RECORD.clone();
            second_record.base_version = Some(12);
            let data = vec![BASE_RECORD.clone(), second_record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::DifferentBaseVersion
            ));
        }

        // Name concatenation
        #[test]
        fn test_name_concatenation() {
            let mut second_record = BASE_RECORD.clone();
            second_record.name = Some("efgh".to_string());
            let data = vec![BASE_RECORD.clone(), second_record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(resolved_data[0].name, "abcd-");
            assert_eq!(resolved_data[1].name, "abcd-efgh");
        }

        // Missing name
        #[test]
        fn test_missing_name() {
            let mut first_record = EMPTY_RECORD.clone();
            first_record.name = Some("efgh".to_string());
            first_record.value = Some(10.0);
            let mut second_record = EMPTY_RECORD.clone();
            second_record.value = Some(10.0);
            let data = vec![first_record, second_record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::MissingName(1)
            ));
        }

        #[test]
        fn test_invalid_name() {
            let mut first_record = EMPTY_RECORD.clone();
            first_record.name = Some("   ".to_string());
            first_record.value = Some(10.0);
            let data = vec![first_record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::InvalidNameInRecord(0)
            ));
        }

        #[test]
        fn test_units() {
            let mut second_record = BASE_RECORD.clone();
            second_record.unit = Some("F".to_string());
            let data = vec![BASE_RECORD.clone(), second_record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(resolved_data[0].unit, Some("Cel".to_string()));
            assert_eq!(resolved_data[1].unit, Some("F".to_string()));
        }

        #[test]
        fn test_basetime() {
            let mut first_record = EMPTY_RECORD.clone();
            first_record.time = Some(1111111111.1);
            first_record.name = Some("efgh".to_string());
            first_record.value = Some(10.0);
            let mut second_record = BASE_RECORD.clone();
            second_record.base_time = Some(2222222222.2);
            let mut third_record = EMPTY_RECORD.clone();
            third_record.time = Some(3333333333.3);
            let data = vec![first_record, second_record, third_record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(resolved_data[0].time.timestamp(), 1111111111);
            assert_eq!(resolved_data[1].time.timestamp(), 2222222222);
            assert_eq!(resolved_data[2].time.timestamp(), 5555555555);
        }

        #[test]
        fn test_relative_time() {
            let mut first_record = BASE_RECORD.clone();
            first_record.base_time = None;
            let mut second_record = EMPTY_RECORD.clone();
            second_record.time = Some(12.0);
            let data = vec![first_record, second_record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            let now_in_12_seconds = NOW.add(chrono::Duration::seconds(12)).timestamp();
            assert_eq!(resolved_data[0].time.timestamp(), NOW.timestamp());
            assert_eq!(resolved_data[1].time.timestamp(), now_in_12_seconds);
        }

        #[test]
        fn test_invalid_time() {
            let mut first_record = EMPTY_RECORD.clone();
            // NaN time ?
            first_record.time = Some(0.0 / 0.0);
            first_record.name = Some("efgh".to_string());
            first_record.value = Some(10.0);
            let data = vec![first_record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::InvalidTimeInRecord(0)
            ));
        }

        #[test]
        fn test_sum() {
            let mut first_record = EMPTY_RECORD.clone();
            first_record.name = Some("efgh".to_string());
            first_record.sum = Some(5.0);
            let mut second_record = BASE_RECORD.clone();
            second_record.base_sum = Some(10.0);
            let mut third_record = EMPTY_RECORD.clone();
            third_record.sum = Some(20.0);
            let data = vec![first_record, second_record, third_record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(resolved_data[0].sum, Some(5.0));
            assert_eq!(resolved_data[1].sum, Some(10.0));
            assert_eq!(resolved_data[2].sum, Some(30.0));
        }

        #[test]
        fn test_missing_value_or_sum() {
            let mut record = EMPTY_RECORD.clone();
            record.name = Some("efgh".to_string());
            let data = vec![record];
            let resolved_data = resolve_records(&data, *NOW);
            assert_eq!(
                resolved_data.unwrap()[0].value,
                Some(SenMLValueField::FloatingPoint(0.0))
            );
        }

        #[test]
        fn test_two_value_fields() {
            let mut record = EMPTY_RECORD.clone();
            record.name = Some("efgh".to_string());
            record.value = Some(10.0);
            record.string_value = Some("Hello world!".to_string());
            let data = vec![record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::OnlyOneValuePerRecord(0)
            ));
        }

        #[test]
        fn test_no_units_is_fine() {
            let mut record = EMPTY_RECORD.clone();
            record.name = Some("efgh".to_string());
            record.value = Some(10.0);
            record.unit = None;
            let data = vec![record];
            let resolved_data = resolve_records(&data, *NOW);
            assert!(resolved_data.is_ok());
        }

        #[test]
        fn test_extra_fields_are_preserved() {
            let mut record = BASE_RECORD.clone();
            record.extra_fields =
                Some(serde_json::from_str(r#"{"extra_field": "extra_value"}"#).unwrap());
            let data = vec![record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(
                resolved_data[0].extra_fields,
                Some(serde_json::from_str(r#"{"extra_field": "extra_value"}"#).unwrap())
            );
        }

        #[test]
        fn test_empty_extra_fields_are_skipped() {
            let mut record = BASE_RECORD.clone();
            record.extra_fields = Some(serde_json::from_str(r#"{}"#).unwrap());
            let data = vec![record];
            let resolved_data = resolve_records(&data, *NOW).unwrap();
            assert_eq!(resolved_data[0].extra_fields, None);
        }

        #[test]
        fn test_resolver_helpers() {
            let mut records = resolve_records(&vec![BASE_RECORD.clone()], *NOW).unwrap();
            let mut record = records.pop().unwrap();
            // None, defaults to the float value
            assert_eq!(record.get_bool_value(), None);
            assert_eq!(record.get_string_value(), None);
            assert_eq!(record.get_data_value(), None);
            assert_eq!(record.get_float_value(), Some(10.0));
            // Boolean
            record.value = Some(SenMLValueField::BooleanValue(true));
            assert_eq!(record.get_bool_value(), Some(true));
            assert_eq!(record.get_string_value(), None);
            assert_eq!(record.get_data_value(), None);
            assert_eq!(record.get_float_value(), None);
            // String
            record.value = Some(SenMLValueField::StringValue("Hello world!".to_string()));
            assert_eq!(record.get_bool_value(), None);
            assert_eq!(record.get_string_value(), Some(&"Hello world!".to_string()));
            assert_eq!(record.get_data_value(), None);
            assert_eq!(record.get_float_value(), None);
            // Data
            record.value = Some(SenMLValueField::DataValue(Vec::from(
                "Hello world!".as_bytes(),
            )));
            assert_eq!(record.get_bool_value(), None);
            assert_eq!(record.get_string_value(), None);
            assert_eq!(
                record.get_data_value(),
                Some(&Vec::from("Hello world!".as_bytes()))
            );
            assert_eq!(record.get_float_value(), None);
            // Float
            record.value = Some(SenMLValueField::FloatingPoint(10.0));
            assert_eq!(record.get_bool_value(), None);
            assert_eq!(record.get_string_value(), None);
            assert_eq!(record.get_data_value(), None);
            assert_eq!(record.get_float_value(), Some(10.0));
        }

        #[test]
        fn test_zero_version_number() {
            let mut record = BASE_RECORD.clone();
            record.base_version = Some(0);
            let data = vec![record];
            assert!(matches!(
                resolve_records(&data, *NOW).unwrap_err(),
                SinditSenMLError::InvalidVersionNumber
            ));
        }
    }

    mod test_parse_json {
        use crate::*;
        use chrono::Utc;

        #[test]
        fn test_empty() {
            let data = "[]";
            let resolved_data = parse_json(data, None);
            assert!(resolved_data.is_ok());
            assert_eq!(resolved_data.unwrap(), Vec::new());
        }

        #[test]
        fn test_single_record() {
            let data = r#"[{"n": "abcd", "v": 10.0}]"#;
            let now = Utc::now();
            let resolved_data = parse_json(data, Some(now));
            assert!(resolved_data.is_ok());
            assert_eq!(
                resolved_data.unwrap(),
                vec![SenMLResolvedRecord {
                    name: "abcd".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::FloatingPoint(10.0)),
                    sum: None,
                    time: now,
                    update_time: None,
                    base_version: None,
                    extra_fields: None,
                }]
            );
        }

        #[test]
        fn test_multiple_records() {
            let data = r#"[{"n": "abcd", "v": 10.0}, {"n": "efgh", "v": 20.0, "t": 1.5}]"#;
            let now = Utc::now();
            let now_in_1_5_seconds =
                now + chrono::Duration::seconds(1) + chrono::Duration::milliseconds(500);
            let resolved_data = parse_json(data, Some(now));
            assert!(resolved_data.is_ok());
            assert_eq!(
                resolved_data.unwrap(),
                vec![
                    SenMLResolvedRecord {
                        name: "abcd".to_string(),
                        unit: None,
                        value: Some(SenMLValueField::FloatingPoint(10.0)),
                        sum: None,
                        time: now,
                        update_time: None,
                        base_version: None,
                        extra_fields: None,
                    },
                    SenMLResolvedRecord {
                        name: "efgh".to_string(),
                        unit: None,
                        value: Some(SenMLValueField::FloatingPoint(20.0)),
                        sum: None,
                        time: now_in_1_5_seconds,
                        update_time: None,
                        base_version: None,
                        extra_fields: None,
                    }
                ]
            );
        }

        #[test]
        fn test_record_with_extra_fields() {
            let data = r#"[{"n": "abcd", "v": 10.0, "extra_field": "extra_value"}]"#;
            let now = Utc::now();
            let resolved_data = parse_json(data, Some(now));
            assert!(resolved_data.is_ok());
            assert_eq!(
                resolved_data.unwrap(),
                vec![SenMLResolvedRecord {
                    name: "abcd".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::FloatingPoint(10.0)),
                    sum: None,
                    time: now,
                    update_time: None,
                    base_version: None,
                    extra_fields: Some(
                        serde_json::from_str(r#"{"extra_field": "extra_value"}"#).unwrap()
                    ),
                }]
            );
        }

        #[test]
        fn test_invalid_json() {
            let data = r#"[{"n": "abcd", "v": 10.0"#;
            let resolved_data = parse_json(data, None);
            assert!(matches!(
                resolved_data.unwrap_err(),
                SinditSenMLError::InvalidJSON(_)
            ));
        }
    }

    mod test_serialisation {
        use crate::*;
        use chrono::Utc;

        #[test]
        fn test_serialise_empty() {
            let data: Vec<SenMLResolvedRecord> = Vec::new();
            let serialised_data = serde_json::to_string(&data).unwrap();
            assert_eq!(serialised_data, "[]");
        }

        #[test]
        fn test_serialise_single_record() {
            let time = DateTime::<Utc>::from_timestamp(1234567890, 1234 * 100_000_u32).unwrap();
            let data = vec![SenMLResolvedRecord {
                name: "abcd".to_string(),
                unit: None,
                value: Some(SenMLValueField::FloatingPoint(10.3)),
                sum: None,
                time: time,
                update_time: None,
                base_version: Some(12),
                extra_fields: None,
            }];
            let serialised_data = serde_json::to_string(&data).unwrap();
            assert_eq!(
                serialised_data,
                r#"[{"n":"abcd","v":10.3,"t":1234567890.1234,"bver":12}]"#
            );
        }

        #[test]
        fn test_serialise_multiple_records() {
            let time = DateTime::<Utc>::from_timestamp(1234567890, 1234 * 100_000_u32).unwrap();
            let data = vec![
                SenMLResolvedRecord {
                    name: "abcd".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::FloatingPoint(10f64)),
                    sum: None,
                    time: time,
                    update_time: None,
                    base_version: None,
                    extra_fields: Some(
                        serde_json::from_str(r#"{"extra_field": "extra_value"}"#).unwrap(),
                    ),
                },
                SenMLResolvedRecord {
                    name: "efgh".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::DataValue(Vec::from(
                        "Hello world!".as_bytes(),
                    ))),
                    sum: None,
                    time: time,
                    update_time: None,
                    base_version: None,
                    extra_fields: Some(serde_json::from_str(r#"{"no":false}"#).unwrap()),
                },
                SenMLResolvedRecord {
                    name: "ijkl".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::BooleanValue(true)),
                    sum: None,
                    time: time,
                    update_time: None,
                    base_version: None,
                    extra_fields: None,
                },
                SenMLResolvedRecord {
                    name: "mnop".to_string(),
                    unit: None,
                    value: Some(SenMLValueField::StringValue("Hello world!".to_string())),
                    sum: None,
                    time: time,
                    update_time: None,
                    base_version: None,
                    extra_fields: None,
                },
            ];
            let serialised_data = serde_json::to_string(&data).unwrap();
            assert_eq!(
                serialised_data,
                r#"[{"n":"abcd","v":10,"t":1234567890.1234,"extra_field":"extra_value"},{"n":"efgh","vd":"SGVsbG8gd29ybGQh","t":1234567890.1234,"no":false},{"n":"ijkl","vb":true,"t":1234567890.1234},{"n":"mnop","vs":"Hello world!","t":1234567890.1234}]"#
            );
        }
        #[test]
        fn test_base64_urlsafe() {
            let data = vec![SenMLResolvedRecord {
                name: "abcd".to_string(),
                unit: None,
                value: Some(SenMLValueField::DataValue(Vec::from(
                    "light work".as_bytes(),
                ))),
                sum: None,
                time: DateTime::<Utc>::from_timestamp(1234567890, 1234 * 100_000_u32).unwrap(),
                update_time: None,
                base_version: Some(11),
                extra_fields: None,
            }];
            let serialised_data = serde_json::to_string(&data).unwrap();
            let parsed_data: serde_json::Value = serde_json::from_str(&serialised_data).unwrap();
            assert_eq!(
                parsed_data[0]["vd"],
                serde_json::Value::String("bGlnaHQgd29yaw".to_string())
            );
            let parsed_records = parse_json(&serialised_data, None).unwrap();
            assert_eq!(parsed_records[0].get_data_value().unwrap(), b"light work");

            let data = vec![SenMLResolvedRecord {
                name: "abcd".to_string(),
                unit: None,
                value: Some(SenMLValueField::DataValue(b"//\xC2\xBB".to_vec())),
                sum: None,
                time: DateTime::<Utc>::from_timestamp(1234567890, 1234 * 100_000_u32).unwrap(),
                update_time: None,
                base_version: Some(11),
                extra_fields: None,
            }];
            let serialised_data = serde_json::to_string(&data).unwrap();
            let parsed_data: serde_json::Value = serde_json::from_str(&serialised_data).unwrap();
            assert_eq!(
                parsed_data[0]["vd"],
                serde_json::Value::String("Ly_Cuw".to_string())
            );
            let parsed_records = parse_json(&serialised_data, None).unwrap();
            assert_eq!(parsed_records[0].get_data_value().unwrap(), b"//\xC2\xBB");
        }
    }

    mod test_crate_documentation_examples {
        #[test]
        fn test_example_parsing() {
            use crate::parse_json;

            let json_str = r#"[{"n": "temperature", "v": 42.0}]"#;
            let records = parse_json(json_str, None).unwrap();
            assert_eq!(records[0].name, "temperature");
            assert_eq!(records[0].get_float_value(), Some(42.0));
        }

        #[test]
        fn test_example_serialisation() {
            use crate::SenMLResolvedRecord;

            let record = SenMLResolvedRecord {
                name: "temperature".to_string(),
                unit: Some("Cel".to_string()),
                value: Some(crate::SenMLValueField::FloatingPoint(42f64)),
                sum: None,
                time: chrono::DateTime::<chrono::Utc>::from_timestamp(1234567890, 0).unwrap(),
                update_time: None,
                base_version: None,
                extra_fields: None,
            };
            let json = serde_json::to_string(&vec![record]).unwrap();
            assert_eq!(
                json,
                r#"[{"n":"temperature","u":"Cel","v":42,"t":1234567890}]"#
            );
        }
    }
}
