/// SenML code snippets extracted from the standard description examples
/// https://www.rfc-editor.org/rfc/rfc8428#section-5.1
struct SenMLSpecificationExamples;

impl SenMLSpecificationExamples {
    /// The following shows a temperature reading taken approximately "now"
    /// by a 1-wire sensor device that was assigned the unique 1-wire address
    /// of 10e2073a01080063:
    const SINGLE_DATAPOINT: &'static str = r#"
    [
     {"n":"urn:dev:ow:10e2073a01080063","u":"Cel","v":23.1}
    ]
    "#;

    /// The following example shows voltage and current "now", i.e., at an
    /// unspecified time.
    const MULTIPLE_DATAPOINT: &'static str = r#"
    [
        {"bn":"urn:dev:ow:10e2073a01080063:","n":"voltage","u":"V","v":120.1},
        {"n":"current","u":"A","v":1.2}
    ]
    "#;

    /// The next example is similar to the above one, but it shows current at
    /// Tue Jun 8 18:01:16.001 UTC 2010 and at each second for the previous 5
    /// seconds.
    const MULTIPLE_DATAPOINT_AND_TIME: &'static str = r#"
    [
        {"bn":"urn:dev:ow:10e2073a0108006:","bt":1.276020076001e+09,
        "bu":"A","bver":5,
        "n":"voltage","u":"V","v":120.1},
        {"n":"current","t":-5,"v":1.2},
        {"n":"current","t":-4,"v":1.3},
        {"n":"current","t":-3,"v":1.4},
        {"n":"current","t":-2,"v":1.5},
        {"n":"current","t":-1,"v":1.6},
        {"n":"current","v":1.7}
    ]
    "#;

    /// As an example of SenSML, the following stream of measurements may be
    /// sent via a long-lived HTTP POST from the producer of the stream to
    /// its consumer, and each measurement object may be reported at the time
    /// it was measured:
    #[cfg(feature = "stream")]
    const STREAM: &'static str = r#"
    [
     {"bn":"urn:dev:ow:10e2073a01080063","bt":1.320067464e+09,
      "bu":"%RH","v":21.2},
     {"t":10,"v":21.3},
     {"t":20,"v":21.4},
     {"t":30,"v":21.4},
     {"t":40,"v":21.5},
     {"t":50,"v":21.5},
     {"t":60,"v":21.5},
     {"t":70,"v":21.6},
     {"t":80,"v":21.7},
    "#;

    /// The following example shows humidity measurements from a mobile
    /// device with a 1-wire address 10e2073a01080063, starting at Mon Oct 31
    /// 13:24:24 UTC 2011.  The device also provides position data, which is
    /// provided in the same measurement or parameter array as separate
    /// entries.  Note that time is used to correlate data that belongs
    /// together, e.g., a measurement and a parameter associated with it.
    /// Finally, the device also reports extra data about its battery status
    /// at a separate time.
    const MULTIPLE_MEASUREMENTS: &'static str = r#"
    [
        {"bn":"urn:dev:ow:10e2073a01080063","bt":1.320067464e+09,
        "bu":"%RH","v":20},
        {"u":"lon","v":24.30621},
        {"u":"lat","v":60.07965},
        {"t":60,"v":20.3},
        {"u":"lon","t":60,"v":24.30622},
        {"u":"lat","t":60,"v":60.07965},
        {"t":120,"v":20.7},
        {"u":"lon","t":120,"v":24.30623},
        {"u":"lat","t":120,"v":60.07966},
        {"u":"%EL","t":150,"v":98},
        {"t":180,"v":21.2},
        {"u":"lon","t":180,"v":24.30628},
        {"u":"lat","t":180,"v":60.07967}
    ]
    "#;

    /// The following shows the example from the previous section in resolved
    /// format.
    const RESOLVED_DATA: &'static str = r#"
    [
        {"n":"urn:dev:ow:10e2073a01080063","u":"%RH","t":1.320067464e+09,
        "v":20},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lon","t":1.320067464e+09,
        "v":24.30621},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lat","t":1.320067464e+09,
        "v":60.07965},
        {"n":"urn:dev:ow:10e2073a01080063","u":"%RH","t":1.320067524e+09,
        "v":20.3},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lon","t":1.320067524e+09,
        "v":24.30622},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lat","t":1.320067524e+09,
        "v":60.07965},
        {"n":"urn:dev:ow:10e2073a01080063","u":"%RH","t":1.320067584e+09,
        "v":20.7},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lon","t":1.320067584e+09,
        "v":24.30623},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lat","t":1.320067584e+09,
        "v":60.07966},
        {"n":"urn:dev:ow:10e2073a01080063","u":"%EL","t":1.320067614e+09,
        "v":98},
        {"n":"urn:dev:ow:10e2073a01080063","u":"%RH","t":1.320067644e+09,
        "v":21.2},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lon","t":1.320067644e+09,
        "v":24.30628},
        {"n":"urn:dev:ow:10e2073a01080063","u":"lat","t":1.320067644e+09,
        "v":60.07967}
    ]
    "#;

    /// The following example shows a sensor that returns different data
    /// types.
    const MULTIPLE_DATATYPES: &'static str = r#"
    [
        {"bn":"urn:dev:ow:10e2073a01080063:","n":"temp","u":"Cel","v":23.1},
        {"n":"label","vs":"Machine Room"},
        {"n":"open","vb":false},
        {"n":"nfc-reader","vd":"aGkgCg"}
    ]
    "#;

    /// The following example shows the results from a query to one device
    /// that aggregates multiple measurements from other devices.  The
    /// example assumes that a client has fetched information from a device
    /// at 2001:db8::2 by performing a GET operation on http://[2001:db8::2]
    /// at Mon Oct 31 16:27:09 UTC 2011 and has gotten two separate values as
    /// a result: a temperature and humidity measurement as well as the
    /// results from another device at http://[2001:db8::1] that also had a
    /// temperature and humidity measurement.  Note that the last record
    /// would use the Base Name from the 3rd record but the Base Time from
    /// the first record.
    const COLLECTION_OF_RESOURCES: &'static str = r#"
    [
        {"bn":"2001:db8::2/","bt":1.320078429e+09,
        "n":"temperature","u":"Cel","v":25.2},
        {"n":"humidity","u":"%RH","v":30},
        {"bn":"2001:db8::1/","n":"temperature","u":"Cel","v":12.3},
        {"n":"humidity","u":"%RH","v":67}
    ]
    "#;

    /// The following example shows the SenML that could be used to set the
    /// current set point of a typical residential thermostat that has a
    /// temperature set point, a switch to turn on and off the heat, and a
    /// switch to turn on the fan override.
    const SETTING_ACTUATOR: &'static str = r#"
    [
        {"bn":"urn:dev:ow:10e2073a01080063:"},
        {"n":"temp","u":"Cel","v":23.1},
        {"n":"heat","u":"/","v":1},
        {"n":"fan","u":"/","v":0}
    ]
    "#;

    /// In the following example, two different lights are turned on.  It is
    /// assumed that the lights are on a network that can guarantee delivery
    /// of the messages to the two lights within 15 ms (e.g., a network using
    /// 802.1BA [IEEE802.1BA] and 802.1AS [IEEE802.1AS] for time
    /// synchronization).  The controller has set the time of the lights to
    /// come on at 20 ms in the future from the current time.  This allows
    /// both lights to receive the message, wait till that time, then apply
    /// the switch command so that both lights come on at the same time.
    const LIGHTS_ON: &'static str = r#"
    [
        {"bt":1.320078429e+09,"bu":"/","n":"2001:db8::3","v":1},
        {"n":"2001:db8::4","v":1}
    ]
    "#;

    /// The following shows two lights being turned off using a
    /// non-deterministic network that has high odds of delivering a message
    /// in less than 100 ms and uses NTP for time synchronization.  The
    /// current time is 1320078429.  The user has just turned off a light
    /// switch that is turning off two lights.  Both lights are immediately
    /// dimmed to 50% brightness to give the user instant feedback that
    /// something is changing.  However, given the network, the lights will
    /// probably dim at somewhat different times.  Then 100 ms in the future,
    /// both lights will go off at the same time.  The instant, but not
    /// synchronized, dimming gives the user the sensation of quick
    /// responses, and the timed-off 100 ms in the future gives the
    /// perception of both lights going off at the same time.
    const SYNCHRONIZED_LIGHTS_OFF: &'static str = r#"
    [
        {"bt":1.320078429e+09,"bu":"/","n":"2001:db8::3","v":0.5},
        {"n":"2001:db8::4","v":0.5},
        {"n":"2001:db8::3","t":0.1,"v":0},
        {"n":"2001:db8::4","t":0.1,"v":0}
    ]
    "#;
}

mod tests {
    use super::*;
    use chrono::{DateTime, Duration, Utc};
    use sindit_senml::parse_json;

    fn dates_similar(date1: DateTime<Utc>, date2: DateTime<Utc>) -> bool {
        date1.signed_duration_since(date2).num_milliseconds().abs() <= 10
    }

    #[test]
    fn test_single_datapoint() {
        let now = Utc::now();
        let result = parse_json(SenMLSpecificationExamples::SINGLE_DATAPOINT, Some(now)).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[0].unit, Some(String::from("Cel")));
        assert_eq!(result[0].get_float_value(), Some(23.1));
        assert_eq!(result[0].time, now);
    }

    #[test]
    fn test_multiple_datapoints() {
        let now = Utc::now();
        let result = parse_json(SenMLSpecificationExamples::MULTIPLE_DATAPOINT, Some(now)).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063:voltage");
        assert_eq!(result[0].unit, Some(String::from("V")));
        assert_eq!(result[0].get_float_value(), Some(120.1));
        assert_eq!(result[0].time, now);

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a01080063:current");
        assert_eq!(result[1].unit, Some(String::from("A")));
        assert_eq!(result[1].get_float_value(), Some(1.2));
        assert_eq!(result[1].time, now);
    }

    #[test]
    fn test_multiple_datapoints_and_time() {
        let now = Utc::now();
        let basetime = DateTime::<Utc>::from_timestamp(1.276020076001e9 as i64, 0).unwrap();
        let result = parse_json(
            SenMLSpecificationExamples::MULTIPLE_DATAPOINT_AND_TIME,
            Some(now),
        )
        .unwrap();
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a0108006:voltage");
        assert_eq!(result[0].unit, Some(String::from("V")));
        assert_eq!(result[0].get_float_value(), Some(120.1));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a0108006:current");
        assert_eq!(result[1].unit, Some(String::from("A")));
        assert_eq!(result[1].get_float_value(), Some(1.2));
        assert!(dates_similar(
            result[1].time,
            basetime - Duration::seconds(5),
        ));

        assert_eq!(result[2].name, "urn:dev:ow:10e2073a0108006:current");
        assert!(dates_similar(
            result[2].time,
            basetime - Duration::seconds(4),
        ));

        assert!(dates_similar(
            result[3].time,
            basetime - Duration::seconds(3),
        ));

        assert!(dates_similar(
            result[4].time,
            basetime - Duration::seconds(2),
        ));

        assert!(dates_similar(
            result[5].time,
            basetime - Duration::seconds(1),
        ));

        assert!(dates_similar(
            result[6].time,
            basetime - Duration::seconds(0),
        ));
    }

    #[test]
    fn test_multiple_measurements() {
        let basetime = DateTime::<Utc>::from_timestamp(1.320067464e9 as i64, 0).unwrap();
        let result = parse_json(SenMLSpecificationExamples::MULTIPLE_MEASUREMENTS, None).unwrap();
        assert_eq!(result.len(), 13);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[0].unit, Some(String::from("%RH")));
        assert_eq!(result[0].get_float_value(), Some(20.0));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[1].unit, Some(String::from("lon")));
        assert_eq!(result[1].get_float_value(), Some(24.30621));
        assert!(dates_similar(result[1].time, basetime));

        assert_eq!(result[2].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[2].unit, Some(String::from("lat")));
        assert_eq!(result[2].get_float_value(), Some(60.07965));
        assert!(dates_similar(result[2].time, basetime));

        assert_eq!(result[3].get_float_value(), Some(20.3));
        assert!(dates_similar(
            result[3].time,
            basetime + Duration::seconds(60)
        ));

        assert_eq!(result[12].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[12].unit, Some(String::from("lat")));
        assert_eq!(result[12].get_float_value(), Some(60.07967));
        assert!(dates_similar(
            result[12].time,
            basetime + Duration::seconds(180)
        ));
    }

    #[test]
    fn test_resolved_data() {
        let basetime = DateTime::<Utc>::from_timestamp(1.320067464e9 as i64, 0).unwrap();
        let result = parse_json(SenMLSpecificationExamples::RESOLVED_DATA, None).unwrap();
        assert_eq!(result.len(), 13);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[0].unit, Some(String::from("%RH")));
        assert_eq!(result[0].get_float_value(), Some(20.0));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[1].unit, Some(String::from("lon")));
        assert_eq!(result[1].get_float_value(), Some(24.30621));
        assert!(dates_similar(result[1].time, basetime));

        assert_eq!(result[2].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[2].unit, Some(String::from("lat")));
        assert_eq!(result[2].get_float_value(), Some(60.07965));
        assert!(dates_similar(result[2].time, basetime));

        assert_eq!(result[3].get_float_value(), Some(20.3));
        assert!(dates_similar(
            result[3].time,
            basetime + Duration::seconds(60)
        ));

        assert_eq!(result[12].name, "urn:dev:ow:10e2073a01080063");
        assert_eq!(result[12].unit, Some(String::from("lat")));
        assert_eq!(result[12].get_float_value(), Some(60.07967));
        assert!(dates_similar(
            result[12].time,
            basetime + Duration::seconds(180)
        ));
    }

    #[test]
    fn test_serialisation_resolved_data() {
        let result = parse_json(SenMLSpecificationExamples::RESOLVED_DATA, None).unwrap();
        let serialised = serde_json::to_string(&result).unwrap();
        // Replace the scientific notation datetime format with a more common one
        let fixed_example = SenMLSpecificationExamples::RESOLVED_DATA
            .replace("1.320067", "1320067")
            .replace("4e+09", "4");

        // remove the bver fields in the serialised json as the example
        // does not contain them.
        let adjusted_serialised = serialised.replace(r#","bver":10"#, "");

        let parsed_example: serde_json::Value = serde_json::from_str(&fixed_example).unwrap();
        let parsed_serialisation: serde_json::Value =
            serde_json::from_str(&adjusted_serialised).unwrap();

        assert_eq!(parsed_example, parsed_serialisation);
    }

    #[test]
    fn test_serialisation_multiple_measurements() {
        let result = parse_json(SenMLSpecificationExamples::MULTIPLE_MEASUREMENTS, None).unwrap();
        let serialised = serde_json::to_string(&result).unwrap();
        // Replace the scientific notation datetime format with a more common one
        let fixed_example = SenMLSpecificationExamples::RESOLVED_DATA
            .replace("1.320067", "1320067")
            .replace("4e+09", "4");
        // remove the bver fields in the serialised json as the example
        // does not contain them.
        let adjusted_serialised = serialised.replace(r#","bver":10"#, "");

        let parsed_example: serde_json::Value = serde_json::from_str(&fixed_example).unwrap();
        let parsed_serialisation: serde_json::Value =
            serde_json::from_str(&adjusted_serialised).unwrap();

        assert_eq!(parsed_example, parsed_serialisation);
    }

    #[test]
    fn test_multiple_datatypes() {
        let now = Utc::now();
        let result = parse_json(SenMLSpecificationExamples::MULTIPLE_DATATYPES, Some(now)).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063:temp");
        assert_eq!(result[0].unit, Some(String::from("Cel")));
        assert_eq!(result[0].get_float_value(), Some(23.1));
        assert_eq!(result[0].time, now);

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a01080063:label");
        assert_eq!(
            result[1].get_string_value(),
            Some(&String::from("Machine Room"))
        );
        assert_eq!(result[1].time, now);

        assert_eq!(result[2].name, "urn:dev:ow:10e2073a01080063:open");
        assert_eq!(result[2].get_bool_value(), Some(false));
        assert_eq!(result[2].time, now);

        assert_eq!(result[3].name, "urn:dev:ow:10e2073a01080063:nfc-reader");
        assert_eq!(
            result[3].get_data_value(),
            Some(&vec![0x68, 0x69, 0x20, 0x0a])
        );
        assert_eq!(result[3].time, now);
    }

    #[test]
    fn test_collection_of_resources() {
        let basetime = DateTime::<Utc>::from_timestamp(1.320078429e9 as i64, 0).unwrap();
        let result = parse_json(SenMLSpecificationExamples::COLLECTION_OF_RESOURCES, None).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].name, "2001:db8::2/temperature");
        assert_eq!(result[0].unit, Some(String::from("Cel")));
        assert_eq!(result[0].get_float_value(), Some(25.2));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "2001:db8::2/humidity");
        assert_eq!(result[1].unit, Some(String::from("%RH")));
        assert_eq!(result[1].get_float_value(), Some(30.0));
        assert!(dates_similar(result[1].time, basetime));

        assert_eq!(result[2].name, "2001:db8::1/temperature");
        assert_eq!(result[2].unit, Some(String::from("Cel")));
        assert_eq!(result[2].get_float_value(), Some(12.3));
        assert!(dates_similar(result[2].time, basetime));

        assert_eq!(result[3].name, "2001:db8::1/humidity");
        assert_eq!(result[3].unit, Some(String::from("%RH")));
        assert_eq!(result[3].get_float_value(), Some(67.0));
        assert!(dates_similar(result[3].time, basetime));
    }

    #[test]
    fn test_setting_actuator() {
        let now = Utc::now();
        let result = parse_json(SenMLSpecificationExamples::SETTING_ACTUATOR, Some(now)).unwrap();
        assert_eq!(result.len(), 4);

        // A bit weird example with a first datapoint
        // without a value nor a unit. so it defaults to 0 and has
        // an invalid name.
        assert_eq!(result[0].name, "urn:dev:ow:10e2073a01080063:");
        assert_eq!(result[0].unit, None);
        assert_eq!(result[0].get_float_value(), Some(0.0));

        assert_eq!(result[1].name, "urn:dev:ow:10e2073a01080063:temp");
        assert_eq!(result[1].unit, Some(String::from("Cel")));
        assert_eq!(result[1].get_float_value(), Some(23.1));
        assert_eq!(result[1].time, now);

        assert_eq!(result[2].name, "urn:dev:ow:10e2073a01080063:heat");
        assert_eq!(result[2].unit, Some(String::from("/")));
        assert_eq!(result[2].get_float_value(), Some(1.0));
        assert_eq!(result[2].time, now);

        assert_eq!(result[3].name, "urn:dev:ow:10e2073a01080063:fan");
        assert_eq!(result[3].unit, Some(String::from("/")));
        assert_eq!(result[3].get_float_value(), Some(0.0));
        assert_eq!(result[3].time, now);
    }

    #[test]
    fn test_lights_on() {
        let basetime = DateTime::<Utc>::from_timestamp(1.320078429e9 as i64, 0).unwrap();
        let result = parse_json(SenMLSpecificationExamples::LIGHTS_ON, None).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "2001:db8::3");
        assert_eq!(result[0].unit, Some(String::from("/")));
        assert_eq!(result[0].get_float_value(), Some(1.0));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "2001:db8::4");
        assert_eq!(result[1].unit, Some(String::from("/")));
        assert_eq!(result[1].get_float_value(), Some(1.0));
        assert!(dates_similar(result[1].time, basetime));
    }

    #[test]
    fn test_synchronized_lights_off() {
        let basetime = DateTime::<Utc>::from_timestamp(1.320078429e9 as i64, 0).unwrap();
        let result = parse_json(SenMLSpecificationExamples::SYNCHRONIZED_LIGHTS_OFF, None).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].name, "2001:db8::3");
        assert_eq!(result[0].unit, Some(String::from("/")));
        assert_eq!(result[0].get_float_value(), Some(0.5));
        assert!(dates_similar(result[0].time, basetime));

        assert_eq!(result[1].name, "2001:db8::4");
        assert_eq!(result[1].unit, Some(String::from("/")));
        assert_eq!(result[1].get_float_value(), Some(0.5));
        assert!(dates_similar(result[1].time, basetime));

        assert_eq!(result[2].name, "2001:db8::3");
        assert_eq!(result[2].unit, Some(String::from("/")));
        assert_eq!(result[2].get_float_value(), Some(0.0));
        assert!(dates_similar(
            result[2].time,
            basetime + Duration::milliseconds(100)
        ));

        assert_eq!(result[3].name, "2001:db8::4");
        assert_eq!(result[3].unit, Some(String::from("/")));
        assert_eq!(result[3].get_float_value(), Some(0.0));
        assert!(dates_similar(
            result[3].time,
            basetime + Duration::milliseconds(100)
        ));
    }
}
