
#[cfg(test)]
mod test_search_datetimes {
    use datetimescan::search_datetimes::search_datetimes;
    use std::io::Cursor;

    #[test]
    fn test_search_datetimes_no_match() {
        let input = "This is a test input without any datetime";
        let reader = Cursor::new(input);
        let results = search_datetimes(reader);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_datetimes_single_match() {
        let input = "This is a test input with a datetime: 2023-05-08T19:29:50AEST";
        let reader = Cursor::new(input);
        let results = search_datetimes(reader);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], ("2023-05-08T19:29:50AEST".to_string(), 1, 38));
    }

    #[test]
    fn test_search_datetimes_multiple_matches() {
        let input = "Supported formats: '2023-05-08T19:29:50AEST', '2023-05-08T19:29:50UTC', '2023-05-08T19:29:50+1000', '2023-05-08T19:29:50+10:00', '2023-05-08 19:29:50', '2023-05-08T19:29:50'";
        let reader = Cursor::new(input);
        let results = search_datetimes(reader);
        assert_eq!(results.len(), 6);
        assert_eq!(results[0], ("2023-05-08T19:29:50AEST".to_string(), 1, 20));
        assert_eq!(results[1], ("2023-05-08T19:29:50UTC".to_string(), 1, 47));
        assert_eq!(results[2], ("2023-05-08T19:29:50+1000".to_string(), 1, 73));
        assert_eq!(results[3], ("2023-05-08T19:29:50+10:00".to_string(), 1, 101));
        assert_eq!(results[4], ("2023-05-08 19:29:50".to_string(), 1, 130));
        assert_eq!(results[5], ("2023-05-08T19:29:50".to_string(), 1, 153));
    }
}

