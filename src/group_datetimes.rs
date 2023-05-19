//  vim modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

//  Notes:
//  {{{
//  2023-05-14T22:56:21AEST we don't need empty vectors to indicate days/months/years between the first and last datetime chronologically with no matching datetimes (do we?)
//  2023-05-14T22:58:32AEST should be using an enum not a string for 'interval'?
//  2023-05-14T23:01:54AEST we could return a HashMap<String, Vec<usize>> with the indices of the matches instead of the actual DateTimes?
//  }}}

/// Groups `DateTime<FixedOffset>` objects from the given vector by day, month, or year
///
/// # Arguments
/// * `datetimes` - A vector of `DateTime<FixedOffset>` objects.
/// * `interval` - A string that determines the grouping interval ("d", "m", or "y")
///
/// # Returns
/// A `HashMap` where the keys are strings representing the date (YYYY-MM-DD/YYYY-MM/YYYY) 
/// and values are a vector of the `DateTime<FixedOffset>` objects that fall within that interval
///
/// # Panics
/// The function will panic if `interval` is not "d", "m", or "y".
///
/// # Examples
/// ```
/// use datetimescan::group_datetimes::group_datetimes;
/// use chrono::DateTime;
/// let dt1 = DateTime::parse_from_rfc3339("2023-05-14T12:00:00+00:00").unwrap();
/// let dt2 = DateTime::parse_from_rfc3339("2023-05-15T12:00:00+00:00").unwrap();
/// let datetimes = vec![dt1, dt2];
/// let result = group_datetimes(&datetimes, "d");
/// assert_eq!(result.get(&"2023-05-14".to_string()).unwrap().len(), 1);
/// ```
pub fn group_datetimes(datetimes: &Vec<DateTime<FixedOffset>>, interval: &str) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    log::trace!("group_datetimes(), interval=({}), datetimes=({:?})", interval, datetimes);
    let result = if interval.eq_ignore_ascii_case("d") {
        group_datetimes_by_format(datetimes, "%Y-%m-%d")
    } else if interval.eq_ignore_ascii_case("m") {
        group_datetimes_by_format(datetimes, "%Y-%m")
    } else if interval.eq_ignore_ascii_case("y") {
        group_datetimes_by_format(datetimes, "%Y")
    } else {
        panic!("unsupported interval=({}) (must be d/m/y)", interval);
    };
    log::trace!("group_datetimes(), result=({:?})", result);
    result
}

fn group_datetimes_by_format(datetimes: &Vec<DateTime<FixedOffset>>, format: &str) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    let mut result = HashMap::new();
    for datetime in datetimes {
        let date_string = datetime.format(format).to_string();
        result.entry(date_string)
            .or_insert_with(Vec::new)
            .push(datetime.clone());
    }
    result
}

