
#[cfg(test)]
mod test_date_range_ctor { 
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

#[cfg(test)]
mod test_date_range_get_dates { 
    use chrono::NaiveDate;
    use datetimescan::date_range::DateRange;

    #[test]
    fn test_days_range() {
        let inputs = vec![("2020-01-01", "2022-01-01"), 
                          ("2020-01-01", "2020-01-02"), 
                          ("2021-01-01", "2021-12-31"), 
                          ("2021-01-01", "2021-01-01"),];
        let checks_len = vec![ 732, 2, 365, 1, ];
        assert_eq!(inputs.len(), checks_len.len());
        for ((start, end), check_len) in inputs.iter().zip(checks_len.iter()) {
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("d");
            assert_eq!(result.len(), *check_len);
            assert_eq!(result[0].format("%F").to_string(), *start);
            assert_eq!(result[result.len()-1].format("%F").to_string(), *end);
        }
        let inputs = vec![("2020-01-01", "2020-01-05"),];
        let checks_strs = vec![vec!["2020-01-01", "2020-01-02", "2020-01-03", "2020-01-04", "2020-01-05"],];
        assert_eq!(inputs.len(), checks_strs.len());
        for ((start, end), check_strs) in inputs.iter().zip(checks_strs.iter()) {
            let check_dates: Vec<NaiveDate> = check_strs.iter().map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap()).collect();
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("d");
            assert_eq!(result, check_dates);
        }
    }

    #[test]
    fn test_months_range() {
        let inputs = vec![("2020-01-01", "2020-12-31"), 
                          ("2020-01-01", "2020-01-01"), 
                          ("2020-03", "2020-09"), 
                          ("2020-03", "2020-09-30"),
                          ("2020-03-01", "2020-05-15"),];
        let checks_len = vec![ 12, 1, 7, 7, 3, ];
        assert_eq!(inputs.len(), checks_len.len());
        for ((start, end), check_len) in inputs.iter().zip(checks_len.iter()) {
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("m");
            assert_eq!(result.len(), *check_len);
            assert_eq!(result[0].format("%Y-%m").to_string(), start[0..7]);
            assert_eq!(result[result.len()-1].format("%Y-%m").to_string(), end[0..7]);
        }
        let inputs = vec![("2020-01-01", "2020-05-01"),];
        let checks_strs = vec![vec!["2020-01-01", "2020-02-01", "2020-03-01", "2020-04-01", "2020-05-01"],];
        assert_eq!(inputs.len(), checks_strs.len());
        for ((start, end), check_strs) in inputs.iter().zip(checks_strs.iter()) {
            let check_dates: Vec<NaiveDate> = check_strs.iter().map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap()).collect();
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("m");
            assert_eq!(result, check_dates);
        }
    }

    #[test]
    fn test_years_range() {
        let inputs = vec![("2020-01-01", "2020-12-31"), 
                          ("2020-01-01", "2020-01-01"), 
                          ("2020-01-01", "2023-04-05"), 
                          ("2020", "2022-01-01"), 
                          ("1982", "2043"),
                          ("2019-08", "2023-04"),];
        let checks_len = vec![ 1, 1, 4, 3, 62, 5, ];
        assert_eq!(inputs.len(), checks_len.len());
        for ((start, end), check_len) in inputs.iter().zip(checks_len.iter()) {
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("y");
            assert_eq!(result.len(), *check_len);
            assert_eq!(result[0].format("%Y").to_string(), start[0..4]);
            assert_eq!(result[result.len()-1].format("%Y").to_string(), end[0..4]);
        }
        let inputs = vec![("2020-01-01", "2023-01-01"),];
        let checks_strs = vec![vec!["2020-01-01", "2021-01-01", "2022-01-01", "2023-01-01"],];
        assert_eq!(inputs.len(), checks_strs.len());
        for ((start, end), check_strs) in inputs.iter().zip(checks_strs.iter()) {
            let check_dates: Vec<NaiveDate> = check_strs.iter().map(|x| NaiveDate::parse_from_str(x, "%Y-%m-%d").unwrap()).collect();
            let date_range = DateRange::new(start, end);
            let result = date_range.get_dates("y");
            assert_eq!(result, check_dates);
        }
    }
}

#[cfg(test)]
mod test_date_range_is_date_in_range {
    use datetimescan::date_range::DateRange;

    #[test]
    fn test_is_date_in_range() {
        let cases = vec![("2020-01-01", "2021-01-01", "2020-01-01", true), 
          ("2020-01-01", "2021-01-01", "2020-02-01", true),
          ("2020-01-01", "2021-01-01", "2020-12-31", true),
          ("2020-01-01", "2021-01-01", "2021-01-01", true),
          ("2020-01-01", "2021-01-01", "2019-12-31", false),
          ("2020-01-01", "2021-01-01", "2021-01-02", false),
          ("2020-01", "2020-03", "2020-02", true), 
          ("2020-01", "2020-02", "2020-01", true),
          ("2020-01", "2020-02", "2020-02", true),
          ("2020-01", "2020-02", "2020-03", false),
          ("2020", "2022", "2021", true),
          ("2020", "2020", "2020", true),];
        for (start, end, mid, check) in cases.iter() {
            let date_range = DateRange::new(start, end);
            let result = date_range.is_date_in_range(mid);
            assert_eq!(result, *check);
        }
    }
}

#[cfg(test)]
mod test_date_range_get_missing {

}

