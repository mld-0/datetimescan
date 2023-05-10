//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone};
use std::collections::HashMap;

/// Parses a datetime string and returns a `DateTime<FixedOffset>` object.
///
/// The input datetime string can be in various formats, such as RFC3339, with or without
/// timezone offset, and with or without 'T' separator between date and time.
/// If no timezone offset is provided, the local timezone offset will be used.
///
/// (Only currently supported timezones-as-string are: UTC, AEST, AEDT)
///
/// # Arguments
/// * `datetime_str` - A string representing a datetime
///
/// # Examples
/// ```
/// use datetimescan::parse_datetime::parse_datetime;
/// use chrono::{DateTime, FixedOffset};
///
/// let datetime = parse_datetime("2023-05-08T18:30:00+02:00").unwrap();
/// assert_eq!(datetime, DateTime::parse_from_rfc3339("2023-05-08T18:30:00+02:00").unwrap());
/// ```
///
/// # Returns
/// * `Some(DateTime<FixedOffset>)` if the input string can be parsed successfully
/// * `None` if the input string cannot be parsed
pub fn parse_datetime(datetime_str: &str) -> Option<DateTime<FixedOffset>> 
{
    fn map_tzcode_to_tzoffset(datetime_str: &str) -> String
    {
        let mut timezone_map: HashMap<String, &str> = HashMap::new();
        timezone_map.insert("UTC".to_string(), "+0000"); 
        timezone_map.insert("AEST".to_string(), "+1000"); 
        timezone_map.insert("AEDT".to_string(), "+1100"); 

        let datetime_str_with_offset: String = timezone_map
            .iter()
            .fold(datetime_str.to_string(), |acc, (abbr, offset)| {
                acc.replace(abbr, offset)
            });
        datetime_str_with_offset
    }

    let datetime_str_with_offset = map_tzcode_to_tzoffset(datetime_str);

    DateTime::parse_from_rfc3339(&datetime_str_with_offset)
        .ok()
        .or_else(|| {
            DateTime::parse_from_str(&datetime_str_with_offset, "%Y-%m-%dT%H:%M:%S%z")
                .ok()
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(&datetime_str_with_offset, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .and_then(|naive_datetime| {
                    let local_offset = *Local::now().offset();
                    Some(local_offset.from_local_datetime(&naive_datetime).unwrap())
                })
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(&datetime_str_with_offset, "%Y-%m-%d %H:%M:%S")
                .ok()
                .and_then(|naive_datetime| {
                    let local_offset = *Local::now().offset();
                    Some(local_offset.from_local_datetime(&naive_datetime).unwrap())
                })
        })
}

