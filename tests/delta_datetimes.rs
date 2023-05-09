use chrono::DateTime;
use datetimescan::delta_datetimes::datetime_difference_seconds; 

#[test]
fn test_same_datetimes() {
    let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
    let dt2 = dt1.clone();

    assert_eq!(datetime_difference_seconds(dt1, dt2), 0);
}

#[test]
fn test_positive_difference() {
    let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
    let dt2 = DateTime::parse_from_rfc3339("2023-05-01T01:00:00+00:00").unwrap();

    assert_eq!(datetime_difference_seconds(dt1, dt2), 3600);
}

#[test]
fn test_negative_difference() {
    let dt1 = DateTime::parse_from_rfc3339("2023-05-01T01:00:00+00:00").unwrap();
    let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();

    assert_eq!(datetime_difference_seconds(dt1, dt2), -3600);
}

#[test]
fn test_different_offsets() {
    let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+03:00").unwrap();
    let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00-02:00").unwrap();

    assert_eq!(datetime_difference_seconds(dt1, dt2), 5 * 3600);
}

#[test]
fn test_years_appart() {
    let dt1 = DateTime::parse_from_rfc3339("2019-05-01T00:00:00+00:00").unwrap();
    let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();

    assert_eq!(datetime_difference_seconds(dt1, dt2), 1461 * 24 * 60 * 60);
}

