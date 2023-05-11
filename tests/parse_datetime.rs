
mod test_parse_datetimes {
    use datetimescan::parse_datetime::parse_datetimes;
    use chrono::{DateTime, FixedOffset};

    #[test]
    fn test_parses_valid_dates() {
        let valid_dates = vec![
            "2023-05-11T12:00:00+00:00".to_string(),
            "2023-05-12T12:00:00+00:00".to_string(),
            "2023-05-13T12:00:00+00:00".to_string(),
        ];
        let expected: Vec<DateTime<FixedOffset>> = valid_dates
            .iter()
            .map(|s| DateTime::parse_from_rfc3339(s).unwrap())
            .collect();
        let parsed = parse_datetimes(&valid_dates).unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_returns_none_for_invalid_dates() {
        let invalid_dates = vec![
            "2023-05-11T12:00:00+00:00".to_string(),
            "invalid".to_string(),
        ];
        let parsed = parse_datetimes(&invalid_dates);
        assert!(parsed.is_none());
    }

    #[test]
    fn test_empty_vector() {
        let empty_vec: Vec<String> = Vec::new();
        let parsed = parse_datetimes(&empty_vec);
        assert_eq!(parsed, Some(Vec::new()));
    }
}


mod test_parse_datetime {
    use datetimescan::parse_datetime::parse_datetime;
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    #[test]
    fn test_rfc3339() {
        let datetime = parse_datetime("2023-05-08T18:30:00+02:00").unwrap();
        assert_eq!(
            datetime,
            DateTime::parse_from_rfc3339("2023-05-08T18:30:00+02:00").unwrap()
        );
    }

    #[test]
    fn test_without_t_separator() {
        let datetime = parse_datetime("2023-05-08 18:30:00").unwrap();
        let naive_datetime = NaiveDateTime::parse_from_str("2023-05-08 18:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let local_offset = *Local::now().offset();
        assert_eq!(
            datetime,
            local_offset.from_local_datetime(&naive_datetime).unwrap()
        );
    }

    #[test]
    fn test_without_timezone_offset() {
        let datetime = parse_datetime("2023-05-08T18:30:00").unwrap();
        let naive_datetime = NaiveDateTime::parse_from_str("2023-05-08T18:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let local_offset = *Local::now().offset();
        assert_eq!(
            datetime,
            local_offset.from_local_datetime(&naive_datetime).unwrap()
        );
    }

    #[test]
    fn test_with_tzcode_aedt() {
        let datetime = parse_datetime("2023-05-08T18:30:00AEDT").unwrap();
        assert_eq!(
            datetime,
            DateTime::parse_from_str("2023-05-08T18:30:00+1100", "%Y-%m-%dT%H:%M:%S%z").unwrap()
        );
    }

    #[test]
    fn test_with_tzcode_utc() {
        let datetime = parse_datetime("2023-05-08T18:30:00UTC").unwrap();
        assert_eq!(
            datetime,
            DateTime::parse_from_str("2023-05-08T18:30:00+0000", "%Y-%m-%dT%H:%M:%S%z").unwrap()
        );
    }

    #[test]
    fn test_invalid_input() {
        assert!(parse_datetime("invalid").is_none());
    }

    #[test]
    fn test_empty_input() {
        assert!(parse_datetime("").is_none());
    }
}

