# SINDIT SenMl

Rust implementation of the [SenML RFC8428](https://www.rfc-editor.org/rfc/rfc8428) proposed standard.

SenML (Sensor Markup Language) is a format for representing sensor data.

*Only the JSON representation is implemented.*

## Examples:

```rust
use sindit_senml::parse_json;

let json_str = r#"[{"n": "temperature", "v": 42.0}]"#;
let records = parse_json(json_str, None).unwrap();
assert_eq!(records[0].name, "temperature");
assert_eq!(records[0].get_float_value(), Some(42.0));
```

```rust
use sindit_senml::SenMLResolvedRecord;

let record = SenMLResolvedRecord {
    name: "temperature".to_string(),
    unit: Some("Cel".to_string()),
    value: Some(sindit_senml::SenMLValueField::FloatingPoint(42f64)),
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
```