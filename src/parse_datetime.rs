//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone};
use std::collections::HashMap;

/// Parses a vector of strings into `DateTime<FixedOffset>` objects.
///
/// This function iterates over each string in the input vector, attempts to parse it into a `DateTime<FixedOffset>` object,
/// and stores the successfully parsed DateTime objects in a new vector. 
///
/// See `parse_datetime` for formats that can be parsed
///
/// # Arguments
/// * `datetimes_strs` - A vector of strings where each string represents a datetime.
///
/// # Returns
/// * `Some(Vec<DateTime<FixedOffset>>)` if all strings are successfully parsed
/// * `None` if any of the strings cannot be parsed
///
/// # Examples
/// ```
/// use datetimescan::parse_datetime::parse_datetimes;
/// let datetimes = vec![
///     "2023-05-11T12:00:00+00:00".to_string(),
///     "2023-05-12T12:00:00+00:00".to_string(),
///     "2023-05-13T12:00:00+00:00".to_string(),
/// ];
/// let parsed = parse_datetimes(&datetimes);
/// assert!(parsed.is_some());
/// ```
pub fn parse_datetimes(datetimes_strs: &Vec<String>) -> Option<Vec<DateTime<FixedOffset>>>
{
    log::trace!("parse_datetimes(), datetimes_str=({:?})", datetimes_strs);
    let mut result = Vec::with_capacity(datetimes_strs.len());
    for datetime_str in datetimes_strs {
        let loop_result = parse_datetime(datetime_str);
        if loop_result.is_none() {
            log::warn!("parse_datetimes(), failed to parse datetime_str=({}), return None", datetime_str);
            return None
        }
        result.push(loop_result.unwrap());
    }
    log::trace!("parse_datetimes(), result=({:?})", result);
    Some(result)
}


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
        log::trace!("map_tzcode_to_tzoffset(), datetime_str=({})", datetime_str);
        let mut timezone_map: HashMap<String, &str> = HashMap::new();
        timezone_map.insert("UTC".to_string(), "+0000"); 
        timezone_map.insert("AEST".to_string(), "+1000"); 
        timezone_map.insert("AEDT".to_string(), "+1100"); 
        log::trace!("map_tzcode_to_tzoffset(), timezone_map=({:?})", timezone_map);
        let result: String = timezone_map
            .iter()
            .fold(datetime_str.to_string(), |acc, (abbr, offset)| {
                acc.replace(abbr, offset)
            });
        log::trace!("map_tzcode_to_tzoffset(), result=({})", result);
        result 
    }

    log::trace!("parse_datetime(), datetime_str=({})", datetime_str);
    let datetime_str = map_tzcode_to_tzoffset(datetime_str);
    let result = DateTime::parse_from_rfc3339(&datetime_str)
        .ok()
        .or_else(|| {
            DateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S%z")
                .ok()
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .and_then(|naive_datetime| {
                    let local_offset = *Local::now().offset();
                    Some(local_offset.from_local_datetime(&naive_datetime).unwrap())
                })
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
                .ok()
                .and_then(|naive_datetime| {
                    let local_offset = *Local::now().offset();
                    Some(local_offset.from_local_datetime(&naive_datetime).unwrap())
                })
        });
        log::trace!("parse_datetime(), result=({:?})", result);
        result
}

