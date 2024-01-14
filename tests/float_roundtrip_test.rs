mod tests {
    use sindit_senml::parse_json;
    #[test]
    fn test_float_roundtrip() {
        // If feature float_roundtrip
        #[cfg(feature = "float_roundtrip")]
        let number = "97.45365320034685";
        // else
        #[cfg(not(feature = "float_roundtrip"))]
        let number = "97.45365320034684";

        let json_str = format!(r#"[{{"n":"temperature","v":{},"t":1234567891.2}}]"#, number);
        let records = parse_json(&json_str, None).unwrap();
        let json_str_again = serde_json::to_string(&records).unwrap();
        assert_eq!(json_str, json_str_again);
    }
}
