
#[cfg(test)]
mod test_date_range { 
    use chrono::NaiveDate;
    use datetimescan::date_range::{DateRange, parse_partial_date_str};

    #[test]
    fn test_parse_partial_date_str() {
        assert_eq!(parse_partial_date_str("2023"), Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()));
        assert_eq!(parse_partial_date_str("2023-03"), Some(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()));
        assert_eq!(parse_partial_date_str("2023-03-03"), Some(NaiveDate::from_ymd_opt(2023, 3, 3).unwrap()));
        assert_eq!(parse_partial_date_str("invalid"), None);
    
        // Test edge cases
        assert_eq!(parse_partial_date_str("0000"), Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()));
        assert_eq!(parse_partial_date_str("9999"), Some(NaiveDate::from_ymd_opt(9999, 1, 1).unwrap()));
        assert_eq!(parse_partial_date_str("0000-12"), Some(NaiveDate::from_ymd_opt(0, 12, 1).unwrap()));
        assert_eq!(parse_partial_date_str("9999-01"), Some(NaiveDate::from_ymd_opt(9999, 1, 1).unwrap()));
        assert_eq!(parse_partial_date_str("0000-01-01"), Some(NaiveDate::from_ymd_opt(0, 1, 1).unwrap()));
        assert_eq!(parse_partial_date_str("9999-12-31"), Some(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()));
    
        // Test invalid inputs
        assert_eq!(parse_partial_date_str(""), None);
        assert_eq!(parse_partial_date_str("2023-02-30"), None);  // Feb 30 does not exist
        assert_eq!(parse_partial_date_str("2023-00"), None);  // Month 00 does not exist
        assert_eq!(parse_partial_date_str("2023-13"), None);  // Month 13 does not exist
        assert_eq!(parse_partial_date_str("202300"), None);  // Not a valid format
    }

    #[test]
    fn test_date_range_new() {
        let dr = DateRange::new("2023", "2023-03-03");
        assert_eq!(dr.start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(dr.end, NaiveDate::from_ymd_opt(2023, 3, 3).unwrap());

        let dr = DateRange::new("2023-02", "2023-03");
        assert_eq!(dr.start, NaiveDate::from_ymd_opt(2023, 2, 1).unwrap());
        assert_eq!(dr.end, NaiveDate::from_ymd_opt(2023, 3, 1).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_date_range_new_invalid() {
        DateRange::new("2023-02-30", "2023-03-03");  // Invalid date
    }

    #[test]
    fn test_date_range_new_from_str_range() {
        let dr = DateRange::new_from_str_range(vec!["2023", "2023-03-03"]);
        assert_eq!(dr.start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(dr.end, NaiveDate::from_ymd_opt(2023, 3, 3).unwrap());

        let dr = DateRange::new_from_str_range(vec!["2023-02", "2023-03", "2023"]);
        assert_eq!(dr.start, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(dr.end, NaiveDate::from_ymd_opt(2023, 3, 1).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_date_range_new_from_str_range_invalid() {
        DateRange::new_from_str_range(vec!["2023-02-30", "2023-03-03"]);  // Invalid date
    }

}

