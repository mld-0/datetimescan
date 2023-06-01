//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use chrono::{DateTime, FixedOffset};

//  Notes:
//  {{{
//  2023-05-18T21:30:25AEST (do we need to) determine datetimes corresponding to start/end of each split?
//  2023-05-18T21:31:41AEST splits -> whether to use i64/u64(?)
//  2023-05-19T22:35:20AEST remove 'current_sum' from `split_deltas()` (how to sum vector and  compare result against 0_u64 as a condition of the if-statement?)
//  }}}

/// Calculates the difference in seconds between consecutive `DateTime` objects.
///
/// # Arguments
/// * `datetimes` - A reference to `Vec<DateTime<FixedOffset>>`
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
/// let diff = datetimescan::delta_datetimes::delta_datetimes(&vec![dt1, dt2], false);
/// assert_eq!(diff, vec![60]);
/// ```
pub fn delta_datetimes(datetimes: &Vec<DateTime<FixedOffset>>, allow_negatives: bool) -> Vec<i64>
{
    log::trace!("delta_datetimes(), datetimes=({:?}), allow_negatives=({})", datetimes, allow_negatives);
    if datetimes.is_empty() {
        return vec![];
    }
    let mut result = Vec::with_capacity(datetimes.len() - 1);
    let mut i = 1;
    while i < datetimes.len() {
        let delta = datetime_difference_seconds(datetimes[i-1], datetimes[i]);
        if !allow_negatives && delta < 0 {
            result.push(0);
        } else {
            result.push(delta);
        }
        i += 1;
    }
    log::debug!("delta_datetimes(), result=({:?})", result);
    result
}


/// Calculates the signed difference between two `DateTime<FixedOffset>` values in seconds.
///
/// This function takes two `DateTime<FixedOffset>` values as input and calculates the signed difference
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
/// * `dt1` - The first `DateTime<FixedOffset>` value.
/// * `dt2` - The second `DateTime<FixedOffset>` value.
///
/// # Returns
/// An `i64` value representing the signed difference between the two `DateTime<FixedOffset>` values in seconds.
pub fn datetime_difference_seconds(dt1: DateTime<FixedOffset>, dt2: DateTime<FixedOffset>) -> i64 
{
    log::trace!("datetime_difference_seconds(), dt1=({}), dt2=({})", dt1, dt2);
    let result = dt2.signed_duration_since(dt1).num_seconds();
    log::trace!("datetime_difference_seconds(), result=({})", result);
    result
}


/// Splits the provided list of deltas into periods of continuous activity.
///
/// A new period is started whenever a delta is encountered that is either negative or exceeds the specified timeout. The function then returns a list of the total accumulated time for each period of continuous activity.
///
/// # Arguments
/// * `deltas` - A reference to a vector of i64s, where each i64 represents the difference between subsequent datetimes in a log, in seconds.
/// * `timeout` - A u64 representing the maximum allowed difference between subsequent datetimes for them to be considered part of the same period of continuous activity.
///
/// # Returns
/// A vector of u64s where each u64 represents the total length of a period of continuous activity, in seconds.
///
/// # Example
/// ```
/// use datetimescan::delta_datetimes::split_deltas;
/// let deltas = vec![100, 150, 500, 100];
/// let timeout = 300;
/// assert_eq!(split_deltas(&deltas, timeout), vec![250, 100]);
/// ```
pub fn split_deltas(deltas: &Vec<i64>, timeout: u64) -> Vec<u64>
{
    log::debug!("split_deltas(), timeout=({}), deltas=({:?})", timeout, deltas);
    let mut result = vec![];
    let mut splits: Vec<Vec<u64>> = vec![];
    let mut current_split: Vec<u64> = vec![];
    for delta in deltas {
        if *delta < 0 || (*delta as u64) > timeout {
            if !current_split.is_empty() {
                let current_sum: u64 = current_split.iter().sum();
                if current_sum > 0 {
                    splits.push(current_split);
                }
            }
            current_split = vec![];
        } else {
            current_split.push(*delta as u64);
        }
    }
    if !current_split.is_empty() {
        let current_sum: u64 = current_split.iter().sum();
        if current_sum > 0 {
            splits.push(current_split);
        }
    }
    for split in &splits {
        result.push(split.iter().sum());
    }
    log::debug!("split_deltas(), result=({:?})", result);
    result
}

