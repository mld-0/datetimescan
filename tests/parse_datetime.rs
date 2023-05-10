use datetimescan::parse_datetime::parse_datetime;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

#[test]
fn test_parse_datetime_rfc3339() {
    let datetime = parse_datetime("2023-05-08T18:30:00+02:00").unwrap();
    assert_eq!(
        datetime,
        DateTime::parse_from_rfc3339("2023-05-08T18:30:00+02:00").unwrap()
    );
}

#[test]
fn test_parse_datetime_without_t_separator() {
    let datetime = parse_datetime("2023-05-08 18:30:00").unwrap();
    let naive_datetime = NaiveDateTime::parse_from_str("2023-05-08 18:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let local_offset = *Local::now().offset();
    assert_eq!(
        datetime,
        local_offset.from_local_datetime(&naive_datetime).unwrap()
    );
}

#[test]
fn test_parse_datetime_without_timezone_offset() {
    let datetime = parse_datetime("2023-05-08T18:30:00").unwrap();
    let naive_datetime = NaiveDateTime::parse_from_str("2023-05-08T18:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let local_offset = *Local::now().offset();
    assert_eq!(
        datetime,
        local_offset.from_local_datetime(&naive_datetime).unwrap()
    );
}

#[test]
fn test_parse_datetime_with_tzcode_aedt() {
    let datetime = parse_datetime("2023-05-08T18:30:00AEDT").unwrap();
    assert_eq!(
        datetime,
        DateTime::parse_from_str("2023-05-08T18:30:00+1100", "%Y-%m-%dT%H:%M:%S%z").unwrap()
    );
}

#[test]
fn test_parse_datetime_with_tzcode_utc() {
    let datetime = parse_datetime("2023-05-08T18:30:00UTC").unwrap();
    assert_eq!(
        datetime,
        DateTime::parse_from_str("2023-05-08T18:30:00+0000", "%Y-%m-%dT%H:%M:%S%z").unwrap()
    );
}

#[test]
fn test_parse_datetime_invalid_input() {
    assert!(parse_datetime("invalid").is_none());
}

#[test]
fn test_parse_datetime_empty_input() {
    assert!(parse_datetime("").is_none());
}

