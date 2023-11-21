use crate::SenMLRecord;

// See
// https://github.com/Marcono1234/struson/discussions/19
mod tests {
    use crate::SenMLRecord;
    use struson::reader::*;

    #[test]
    fn test_stream() {
        let json = r#"{"a": [1, true]}"#;
        let reader = std::io::BufReader::new(json.as_bytes());
        let mut json_reader = struson::reader::JsonStreamReader::new(reader);

        json_reader.begin_array().expect("Begin array error");

        while json_reader.has_next()? {
            // let user: User = json_reader.deserialize_next()?;
            let record: SenMLRecord = json_reader.deserialize_next()?;
            // ... use deserialized value in some way
            println!("deserialized: {record:?}")
        }

        // Optionally consume the remainder of the JSON document
        json_reader.end_array()?;
        json_reader.consume_trailing_whitespace()?;
    }
}
