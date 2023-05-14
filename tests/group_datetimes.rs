
#[cfg(test)]
mod test_group_datetimes {
    use datetimescan::group_datetimes::group_datetimes;
    use chrono::DateTime;

    #[test]
    fn test_by_day() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-14T12:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-14T13:00:00+00:00").unwrap();
        let dt3 = DateTime::parse_from_rfc3339("2023-05-15T12:00:00+00:00").unwrap();
        let dt4 = DateTime::parse_from_rfc3339("2023-05-16T12:00:00+00:00").unwrap();
        let dt5 = DateTime::parse_from_rfc3339("2023-05-16T13:00:00+00:00").unwrap();
        let datetimes = vec![dt1, dt2, dt3, dt4, dt5];

        let result = group_datetimes(&datetimes, "d");

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&"2023-05-14".to_string()).unwrap().len(), 2);
        assert_eq!(result.get(&"2023-05-15".to_string()).unwrap().len(), 1);
        assert_eq!(result.get(&"2023-05-16".to_string()).unwrap().len(), 2);
    }

    #[test]
    fn test_by_month() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-14T12:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-15T13:00:00+00:00").unwrap();
        let dt3 = DateTime::parse_from_rfc3339("2023-06-15T12:00:00+00:00").unwrap();
        let dt4 = DateTime::parse_from_rfc3339("2023-07-16T12:00:00+00:00").unwrap();
        let dt5 = DateTime::parse_from_rfc3339("2023-07-16T13:00:00+00:00").unwrap();
        let datetimes = vec![dt1, dt2, dt3, dt4, dt5];

        let result = group_datetimes(&datetimes, "m");

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&"2023-05".to_string()).unwrap().len(), 2);
        assert_eq!(result.get(&"2023-06".to_string()).unwrap().len(), 1);
        assert_eq!(result.get(&"2023-07".to_string()).unwrap().len(), 2);
    }

    #[test]
    fn test_by_year() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-14T12:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-06-15T13:00:00+00:00").unwrap();
        let dt3 = DateTime::parse_from_rfc3339("2024-06-15T12:00:00+00:00").unwrap();
        let dt4 = DateTime::parse_from_rfc3339("2025-07-16T12:00:00+00:00").unwrap();
        let dt5 = DateTime::parse_from_rfc3339("2025-07-16T13:00:00+00:00").unwrap();
        let datetimes = vec![dt1, dt2, dt3, dt4, dt5];

        let result = group_datetimes(&datetimes, "y");

        assert_eq!(result.len(), 3);
        assert_eq!(result.get(&"2023".to_string()).unwrap().len(), 2);
        assert_eq!(result.get(&"2024".to_string()).unwrap().len(), 1);
        assert_eq!(result.get(&"2025".to_string()).unwrap().len(), 2);
    }

    #[test]
    #[should_panic(expected = "unsupported interval=(invalid) (must be d/m/y)")]
    fn test_invalid_date_interval() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-14T12:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-15T12:00:00+00:00").unwrap();
        let datetimes = vec![dt1, dt2];

        group_datetimes(&datetimes, "invalid");
    }
}

