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
    let input = "This input has three datetimes: 2023-05-08T19:29:50AEST, 2023-05-08T19:29:50+1000, and 2023-05-08 19:29:50";
    let reader = Cursor::new(input);
    let results = search_datetimes(reader);
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("2023-05-08T19:29:50AEST".to_string(), 1, 32));
    assert_eq!(results[1], ("2023-05-08T19:29:50+1000".to_string(), 1, 57));
    assert_eq!(results[2], ("2023-05-08 19:29:50".to_string(), 1, 87));
}

