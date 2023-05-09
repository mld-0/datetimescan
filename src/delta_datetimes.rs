//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset};

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
///
/// let dt1 = DateTime::parse_from_rfc3339("2019-05-01T00:00:00+00:00").unwrap();
/// let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
///
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
    dt2.signed_duration_since(dt1).num_seconds()
}

