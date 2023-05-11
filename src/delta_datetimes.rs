//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset};

/// Calculates the difference in seconds between consecutive `DateTime` objects.
///
/// # Arguments
/// * `datetimes` - A vector of `DateTime` objects in `FixedOffset` timezone. 
///     The `DateTime` objects should be in ascending order for meaningful results.
/// * `allow_negatives` - A boolean value indicating whether negative differences 
///     should be preserved. If `false`, any negative difference is replaced with `0`.
///
/// # Returns
/// A vector of integers representing the difference in seconds between each consecutive 
/// pair of `DateTime` objects. The difference is calculated as `datetimes[i] - datetimes[i-1]`.
///
/// # Example
/// ```
/// use chrono::{DateTime, FixedOffset};
/// let dt1 = DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap();
/// let dt2 = DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap();
/// let diff = datetimescan::delta_datetimes::delta_datetimes(vec![dt1, dt2], false);
/// assert_eq!(diff, vec![60]);
/// ```
pub fn delta_datetimes(datetimes: Vec<DateTime<FixedOffset>>, allow_negatives: bool) -> Vec<i64>
{
    if datetimes.len() == 0 {
        return vec![];
    }
    let mut result = Vec::with_capacity(datetimes.len() - 1);
    let mut i = 1;
    while i < datetimes.len() {
        let delta = datetime_difference_seconds(datetimes[i-1], datetimes[i]);
        if allow_negatives == false && delta < 0 {
            result.push(0);
        } else {
            result.push(delta);
        }
        i += 1;
    }
    result
}


/// Calculates the signed difference between two DateTime<FixedOffset> values in seconds.
///
/// This function takes two DateTime<FixedOffset> values as input and calculates the signed difference
/// between them in seconds. The result is the number of seconds between `dt1` and `dt2`.
/// A positive result indicates that `dt2` is later than `dt1`, and a negative result indicates
/// that `dt2` is earlier than `dt1`.
///
/// # Examples
/// ```
/// use chrono::{DateTime, FixedOffset};
/// use datetimescan::delta_datetimes::datetime_difference_seconds;
/// let dt1 = DateTime::parse_from_rfc3339("2019-05-01T00:00:00+00:00").unwrap();
/// let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
/// assert_eq!(datetime_difference_seconds(dt1, dt2), 126230400);
/// ```
///
/// # Arguments
/// * `dt1` - The first DateTime<FixedOffset> value.
/// * `dt2` - The second DateTime<FixedOffset> value.
///
/// # Returns
/// An `i64` value representing the signed difference between the two DateTime<FixedOffset> values in seconds.
pub fn datetime_difference_seconds(dt1: DateTime<FixedOffset>, dt2: DateTime<FixedOffset>) -> i64 
{
    eprintln!("datetime_difference_seconds, dt1=({}), dt2=({})", dt1, dt2);
    dt2.signed_duration_since(dt1).num_seconds()
}

