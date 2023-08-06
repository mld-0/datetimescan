//  vim-modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

//  Notes:
//  {{{
//  2023-05-20T21:40:41AEST 'sum()', Handling the interval="all" case(?)
//  2023-05-20T22:59:09AEST please rename s/run_subcommands/subcommands/ 
//  2023-05-20T22:59:25AEST (named) subcommands? (or commands?)
//  2023-05-27T20:08:17AEST 'failed to parse datetimes_strs', don't we want an error for which datetime(s) specifically failed?
//  2023-06-12T22:19:14AEST just as we have made 'printer' a parameter to the actual subcommand functions, should we provide input stream as a parameter as well?
//  }}}
//  Ongoing: 2023-05-20T23:47:11AEST explain the size of the difference between the sum of 'splits' and 'sum' for textWithIsoDatetimes-2.txt -> 2256 for 'splits' and 2445 for 'sum'

use crate::search_datetimes;
use crate::parse_datetime;
use crate::delta_datetimes;
use crate::group_datetimes;
use crate::printer;

use chrono::{DateTime, FixedOffset, Utc};
use clap::ArgMatches;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

/// Pass ArgParse matches to the function implementing the subcommand specified
pub fn run(matches: &ArgMatches) {
    let mut writer = printer::get_printer_writer(matches);
    let mut printer = match writer.as_deref_mut() {
        Some(w) => printer::Printer::new(Some(w)),
        None => printer::Printer::default(),
    };
    match matches.subcommand() {
        ("locate", Some(matches)) => locate(matches, &mut printer),
        ("parse", Some(matches)) => parse(matches, &mut printer),
        ("convert", Some(matches)) => convert(matches, &mut printer),
        ("filter", Some(matches)) => filter(matches, &mut printer),
        ("count", Some(matches)) => count(matches, &mut printer),
        ("deltas", Some(matches)) => deltas(matches, &mut printer),
        ("splits", Some(matches)) => splits(matches, &mut printer),
        ("sum", Some(matches)) => sum(matches, &mut printer),
        ("groupsum", Some(matches)) => groupsum(matches, &mut printer),
        ("wpm", Some(matches)) => wpm(matches, &mut printer),
        _ => panic!("No subcommand was used. Use --help for more information."),
    }
}

/// Implement subcommand 'locate'
/// List datetime matches and their locations
pub fn locate(matches: &ArgMatches, printer: &mut printer::Printer)
{
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    if matches.is_present("no_locations") {
        printer.print_datetimes_no_locations(&datetimes_and_locations);
    } else {
        printer.print_datetimes_and_locations(&datetimes_and_locations);
    }
}

#[allow(unused_variables)]
pub fn parse(matches: &ArgMatches, printer: &mut printer::Printer)
{
    unimplemented!("UNIMPLEMENTED");
}

/// Implement subcommand 'count'
/// Output count of datetimes per interval (y/m/d)
pub fn count(matches: &ArgMatches, printer: &mut printer::Printer)
{
    let datetimes_grouped = get_datetimes_grouped(matches);
    printer.print_counts_datetimes_grouped(&datetimes_grouped);
}

#[allow(unused_variables)]
pub fn convert(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

#[allow(unused_variables)]
pub fn filter(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

/// Implement subcommand 'deltas'
/// Output seconds elapsed between each datetime match
pub fn deltas(matches: &ArgMatches, printer: &mut printer::Printer)
{
    let deltas = get_deltas(matches);
    printer.print_deltas(&deltas);
}

/// Implement subcommand 'splits'
/// Output splits - the duration of continuous (each delta <= timeout) deltas 
pub fn splits(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    let unit = matches.value_of("unit").expect("expect argument 'unit' in `matches`");
    let splits_per_interval = get_splits_per_interval(matches);
    printer.print_splits_per_interval(&splits_per_interval, unit);
}

/// Implement subcommand 'sum'
/// Output the sum of splits for given interval (y/m/d)
pub fn sum(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    let unit = matches.value_of("unit").expect("expect argument 'unit' in `matches`");
    let sum_splits_per_interval = get_sum_splits_per_interval(matches);
    printer.print_sum_splits_per_interval(&sum_splits_per_interval, unit);
}

#[allow(unused_variables)]
pub fn groupsum(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

#[allow(unused_variables)]
pub fn wpm(matches: &ArgMatches, printer: &mut printer::Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}


/// Search given input for datetime matches (as strings) and their locations in input
///
/// Matching is handled by `search_datetimes::search_datetimes()`
///
/// # Arguments
/// * `matches`: Reference to command line arguments parsed by `clap::ArgMatches`.
///
/// # Returns
/// A `Vec` containing tuples where:
/// * First element is the found datetime string in supported iso-formats.
/// * Second element is the line number (1-indexed) from where the datetime string was extracted.
/// * Third element is the position on that line where the datetime string starts.
///
/// # Panics
/// Panics if reading input fails
fn get_datetimes_and_locations(matches: &ArgMatches) -> Vec<(String, usize, usize)>
{
    let datetimes_and_locations = if let Some(file_path) = matches.value_of("input") {
        let file = File::open(Path::new(file_path)).expect("Failed to open the `matches` file 'input'");
        search_datetimes::search_datetimes(BufReader::new(file))
    } else {
        let stdin = io::stdin();
        search_datetimes::search_datetimes(stdin.lock())
    };
    datetimes_and_locations
}

/// Get all list of all parsed datetimes in given input which pass filters
///
/// Implemented by `get_datetimes_parsed_with_strs_and_positions()`
///
/// # Arguments
/// * `matches`: Reference to command line arguments parsed by `clap::ArgMatches`.
/// Start and end of filter range are optionally given as 'filter_start' and 'filter_end'
/// If 'filter_invert' is specified, datetimes inside the filter range are excluded instead of those outside the range
/// If 'no_future' is specified, panic if any datetimes > now are encountered
/// If 'no_unsorted' is specified, panic if the located datetimes are not in order
///
/// Returns
/// `Vec<DateTime<FixedOffset>>` all parsed datetimes in input which pass filter
///
/// Panics
/// `get_datetimes_parsed_with_strs_and_positions()` may panic if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_datetimes_parsed(matches: &ArgMatches) -> Vec<DateTime<FixedOffset>>
{
    let (datetimes_parsed, _, _) = get_datetimes_parsed_with_strs_and_positions(matches);
    datetimes_parsed
}

/// Locate all datetimes in given input, and filter them by datetime range. Returns list of parsed-and-filtered datetimes, list of all located datetimes and their positions, and a list of which located datetimes passed the filter
///
/// Reading input is handled by `get_datetimes_and_locations()`
/// Parsing is handled by `parse_datetime::parse_datetimes()`
///
/// # Arguments
/// * `matches`: Reference to command line arguments parsed by `clap::ArgMatches`.
/// Start and end of filter range are optionally given as 'filter_start' and 'filter_end'
/// If 'filter_invert' is specified, datetimes inside the filter range are excluded instead of those outside the range
/// If 'no_future' is specified, panic if any datetimes > now are encountered
/// If 'no_unsorted' is specified, panic if the located datetimes are not in order
///
/// # Returns
/// A 3 element tuple: `(Vec<DateTime<FixedOffset>>, Vec<(String, usize, usize)>, Vec<bool>)`
/// * `Vec<DateTime<FixedOffset>>` all parsed datetimes not excluded by the filter
/// * `Vec<(String,usize,usize)>` all origional datetime strings and their positions (including those excluded by the filter) (first `usize` is the line number (1-indexed), second `usize` is the line character position)
/// * `Vec<bool>` indicates which elements of `Vec<(String,usize,usize)>` passed the filter and were included in `Vec<DateTime<FixedOffset>>`
///
/// # Panics
/// Panics if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_datetimes_parsed_with_strs_and_positions(matches: &ArgMatches) -> (Vec<DateTime<FixedOffset>>, Vec<(String, usize, usize)>, Vec<bool>)
{
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    let datetimes_strs = datetimes_and_locations.iter().map(|(s, _, _)| s.to_string()).collect();
    let datetimes_parsed = parse_datetime::parse_datetimes(&datetimes_strs);
    if datetimes_parsed.is_none() {
        panic!("failed to parse datetimes_strs=({:?})", datetimes_strs);
    }
    let datetimes_parsed = datetimes_parsed.unwrap();

    let (filter_start, filter_end) = parse_filter_start_end(matches);
    let filter_invert = matches.is_present("filter_invert");
    let indexes_filter = filter_datetimes_valid_indexes(&datetimes_parsed, &filter_start, &filter_end);
    let datetimes_filtered = datetimes_parsed.iter()
        .zip(indexes_filter.iter())
        .filter(|(_, &include)| if filter_invert { !include } else { include })
        .map(|(&x, _)| x)
        .collect();

    if matches.is_present("no_future") {
        reject_datetimes_future(&datetimes_filtered);
    }
    if matches.is_present("no_unsorted") {
        reject_datetimes_unsorted(&datetimes_filtered);
    }

    assert_eq!(datetimes_and_locations.len(), indexes_filter.len());
    (datetimes_filtered, datetimes_and_locations, indexes_filter)
}

/// Get parsed-and-filtered datetimes from given input, and group them by interval
///
/// Interval is specified as 'per' `matches` argument, and may be 'y' / 'm' / 'd'
///
/// Getting parsed-and-filtered datetimes from input is handled by `get_datetimes_parsed()`
/// Grouping is handled by `group_datetimes::group_datetimes()`
///
/// Returns
/// `HashMap<String, Vec<DateTime<FixedOffset>>>` a list of all datetimes corresponding to each interval (the intervals are represented as strings, '%Y' / '%Y-%m' / '%Y-%m-%d' depending on interval type)
///
/// # Panics
/// `get_datetimes_parsed_with_strs_and_positions()` may panic if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_datetimes_grouped(matches: &ArgMatches) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    let interval = matches.value_of("per").expect("expected `matches` argument 'per'");
    let datetimes_parsed = get_datetimes_parsed(matches);
    let datetimes_grouped = group_datetimes::group_datetimes(&datetimes_parsed, interval);
    datetimes_grouped
}

/// Get 'deltas' - seconds between each parsed-and-filtered datetimes from given input
///
/// If 'allow_negative' is not specified as `matches` argument, any delta<0 is replaced with 0
///
/// Getting parsed-and-filtered datetimes from input is handled by `get_datetimes_parsed()`
/// Difference between datetimes is found by `delta_datetimes::delta_datetimes()`
///
/// Returns
/// `Vec<i64>` list of deltas in seconds
///
/// # Panics
/// `get_datetimes_parsed_with_strs_and_positions()` may panic if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_deltas(matches: &ArgMatches) -> Vec<i64>
{
    let allow_negative = matches.is_present("allow_negative");
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes::delta_datetimes(&datetimes_parsed, allow_negative);
    deltas
}

fn _get_splits(matches: &ArgMatches) -> Vec<u64>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").expect("expect argument 'timeout' in `matches`")
        .parse().unwrap();
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes::delta_datetimes(&datetimes_parsed, allow_negative);
    let splits = delta_datetimes::split_deltas(&deltas, timeout);
    splits
}

/// Get 'splits' for each interval - sums of continuous 'deltas' where no delta > timeout
///
/// Interval is specified as 'per' `matches` argument, and may be 'y' / 'm' / 'd'
/// 'timeout' is given as `matches` argument (in seconds)
///
/// Getting grouped parsed-and-filtered datetimes from input is handled by `get_datetimes_grouped()`
/// Grouping parsed-and-filtered-datetimes is handled by `delta_datetimes::delta_datetimes()`
/// Getting splits is handled by `delta_datetimes::split_deltas()`
///
/// Returns
/// `HashMap<String, Vec<u64>>` a list of all splits (in seconds) - sums of continuous deltas where delta <= timeout - corresponding to each interval (the intervals are represented as strings, '%Y' / '%Y-%m' / '%Y-%m-%d' depending on interval type)
///
/// # Panics
/// Panics if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_splits_per_interval(matches: &ArgMatches) -> HashMap<String, Vec<u64>>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").expect("expect argument 'timeout' in `matches`")
        .parse().unwrap();
    let datetimes_grouped = get_datetimes_grouped(matches);
    let mut splits_per_interval = HashMap::new();
    for (interval, datetimes) in &datetimes_grouped {
        let deltas = delta_datetimes::delta_datetimes(datetimes, allow_negative);
        let splits = delta_datetimes::split_deltas(&deltas, timeout);
        if !splits.is_empty() {
            splits_per_interval.insert(interval.clone(), splits);
        }
    }
    log::trace!("get_splits_per_interval(), result=({:?})", splits_per_interval);
    splits_per_interval
}

/// Get the sum of splits for each interval
///
/// Interval is specified as 'per' `matches` argument, and may be 'y' / 'm' / 'd'
/// 'timeout' is given as `matches` argument (in seconds)
///
/// Getting splits-per-interval is handled by `get_splits_per_interval()`
/// 
/// Returns
/// `HashMap<String, u64>` sum of all splits (in seconds) - sums of continuous deltas where delta <= timeout - corresponding to each interval (the intervals are represented as strings, '%Y' / '%Y-%m' / '%Y-%m-%d' depending on interval type)
///
/// # Panics
/// `get_datetimes_parsed_with_strs_and_positions()` may panic if reading input fails, parsing a matched datetime fails, a datetime is in the future from now and 'no_future' was specified, or datetimes located are out of order and 'no_unsorted' was specified
fn get_sum_splits_per_interval(matches: &ArgMatches) -> HashMap<String, u64>
{
    let splits_per_interval = get_splits_per_interval(matches);
    let mut sum_splits_per_interval: HashMap<String, u64> = HashMap::new();
    for (interval, splits) in splits_per_interval.iter() {
        let sum: u64 = splits.iter().sum();
        sum_splits_per_interval.insert(interval.clone(), sum);
    }
    log::trace!("get_sum_splits_per_interval(), result=({:?})", sum_splits_per_interval);
    sum_splits_per_interval
}

/// Filters a slice of `DateTime<FixedOffset>` values based on optional start and end bounds.
///
/// For each datetime in the input slice, this function checks if it falls within the provided bounds and returns a boolean vector of the same length, where each boolean indicates whether the corresponding datetime passed the filter.
///
/// # Arguments
/// * `datetimes`: A slice of `DateTime<FixedOffset>` values to be filtered.
/// * `filter_start`: An optional `DateTime<FixedOffset>` representing the start bound of the filter.
/// * `filter_end`: An optional `DateTime<FixedOffset>` representing the end bound of the filter.
///
/// # Returns
/// A `Vec<bool>` where each element indicates whether the corresponding datetime in `datetimes` passed the filter.
fn filter_datetimes_valid_indexes(datetimes: &[DateTime<FixedOffset>], filter_start: &Option<DateTime<FixedOffset>>, filter_end: &Option<DateTime<FixedOffset>>) -> Vec<bool> 
{
    datetimes.iter().map(|datetime| {
        match (filter_start, filter_end) {
            (Some(start), Some(end)) => *datetime >= *start && *datetime <= *end,
            (Some(start), None) => *datetime >= *start,
            (None, Some(end)) => *datetime <= *end,
            (None, None) => true,
        }
    }).collect()
}

/// Checks if any `DateTime<FixedOffset>` values in the provided vector are in the future and panics if any are found.
///
/// This function compares each datetime in the input vector to the current datetime adjusted to the corresponding timezone. If any datetimes are detected to be in the future, the function panics and lists those future datetimes.
///
/// # Arguments
/// * `datetimes`: A reference to a `Vec<DateTime<FixedOffset>>` to be checked for future datetimes.
///
/// # Panics
/// Panics if any datetimes in the provided vector are in the future, listing the offending datetimes.
fn reject_datetimes_future(datetimes: &Vec<DateTime<FixedOffset>>)
{
    let mut future_datetimes = Vec::new();
    for date in datetimes {
        let now_in_timezone = Utc::now().with_timezone(date.offset());
        if *date > now_in_timezone {
            future_datetimes.push(*date);
        }
    }
    if !future_datetimes.is_empty() {
        panic!("reject future_datetimes=({:?})", future_datetimes);
    }
}

/// Checks if the `DateTime<FixedOffset>` values in the provided vector are out of ascending order and panics if any are found.
///
/// The function iterates through the provided vector and checks if any datetime is earlier than its predecessor. If any out-of-order datetimes are detected, the function panics and lists those datetimes.
///
/// # Arguments
/// * `datetimes`: A reference to a `Vec<DateTime<FixedOffset>>` to be checked for ordering.
///
/// # Panics
/// Panics if any consecutive datetimes in the provided vector are out of ascending order, listing the offending datetimes.
fn reject_datetimes_unsorted(datetimes: &Vec<DateTime<FixedOffset>>)
{
    let mut out_of_order_datetimes = Vec::new();
    for i in 1..datetimes.len() {
        if datetimes[i] < datetimes[i-1] {
            out_of_order_datetimes.push(datetimes[i]);
        }
    }
    if !out_of_order_datetimes.is_empty() {
        panic!("reject out_of_order_datetimes=({:?})", out_of_order_datetimes);
    }
}

/// Parses `filter_start` and `filter_end` arguments from `ArgMatches` into `DateTime<FixedOffset>` options.
///
/// The function checks the presence of `filter_start` and `filter_end` arguments in the given `ArgMatches`. If present and valid, they are parsed into `DateTime<FixedOffset>`. If invalid, the function will panic with the offending argument.
///
/// # Arguments
/// * `matches`: A reference to `ArgMatches` which might contain `filter_start` and `filter_end` arguments.
///
/// # Returns
/// A tuple containing `Option<DateTime<FixedOffset>>` values for `filter_start` and `filter_end`, respectively.
///
/// # Panics
/// Panics if `filter_start` or `filter_end` are present in `matches` but are invalid datetime strings.
fn parse_filter_start_end(matches: &ArgMatches) -> (Option<DateTime<FixedOffset>>, Option<DateTime<FixedOffset>>)
{
    let filter_start = {
        if matches.is_present("filter_start") {
            let filter_start_str = matches.value_of("filter_start").unwrap();
            let filter_start = parse_datetime::parse_datetime(filter_start_str);
            if filter_start.is_none() {
                panic!("invalid filter_start=({})", filter_start_str);
            } else {
                filter_start
            }
        } else {
            None
        }
    };
    let filter_end = {
        if matches.is_present("filter_end") {
            let filter_end_str = matches.value_of("filter_end").unwrap();
            let filter_end = parse_datetime::parse_datetime(filter_end_str);
            if filter_end.is_none() {
                panic!("invalid filter_end=({})", filter_end_str);
            } else {
                filter_end 
            }
        } else {
            None
        }
    };
    (filter_start, filter_end)
}


//  Tests: filter_datetimes / reject_datetimes
//  (placing these tests out-of-the-way in 'subcommands/tests.rs' works, but produces error in vim-YCM)
#[cfg(test)]
mod tests {
    //  {{{
    use super::*;
    use std::panic;

    #[test]
    fn filter_datetimes_valid_indexes_all() {
        let datetimes: Vec<DateTime<FixedOffset>> = vec![
            DateTime::parse_from_rfc3339("2023-05-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-06-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-07-27T00:00:00+00:00").unwrap(),
        ];
        let result = filter_datetimes_valid_indexes(&datetimes, &None, &None);
        assert_eq!(result, vec![true, true, true]);
    }
    
    #[test]
    fn filter_datetimes_valid_indexes_start() {
        let datetimes: Vec<DateTime<FixedOffset>> = vec![
            DateTime::parse_from_rfc3339("2023-05-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-06-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-07-27T00:00:00+00:00").unwrap(),
        ];
        let start = Some(DateTime::parse_from_rfc3339("2023-06-01T00:00:00+00:00").unwrap());
        let result = filter_datetimes_valid_indexes(&datetimes, &start, &None);
        assert_eq!(result, vec![false, true, true]);
    }
    
    #[test]
    fn filter_datetimes_valid_indexes_end() {
        let datetimes: Vec<DateTime<FixedOffset>> = vec![
            DateTime::parse_from_rfc3339("2023-05-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-06-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-07-27T00:00:00+00:00").unwrap(),
        ];
        let end = Some(DateTime::parse_from_rfc3339("2023-06-30T00:00:00+00:00").unwrap());
        let result = filter_datetimes_valid_indexes(&datetimes, &None, &end);
        assert_eq!(result, vec![true, true, false]);
    }
    
    #[test]
    fn filter_datetimes_valid_indexes_start_end() {
        let datetimes: Vec<DateTime<FixedOffset>> = vec![
            DateTime::parse_from_rfc3339("2023-05-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-06-27T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-07-27T00:00:00+00:00").unwrap(),
        ];
        let start = Some(DateTime::parse_from_rfc3339("2023-06-01T00:00:00+00:00").unwrap());
        let end = Some(DateTime::parse_from_rfc3339("2023-06-30T00:00:00+00:00").unwrap());
        let result = filter_datetimes_valid_indexes(&datetimes, &start, &end);
        assert_eq!(result, vec![false, true, false]);
    }

    #[test]
    fn reject_datetimes_future_no_future_dates() {
        let now = Utc::now();
        let date_strings = vec![
            now.to_rfc3339(),
            (now - chrono::Duration::days(1)).to_rfc3339(),
            (now - chrono::Duration::hours(1)).to_rfc3339(),
        ];
        let dates: Vec<DateTime<FixedOffset>> = date_strings.into_iter()
            .map(|s| DateTime::parse_from_rfc3339(&s).unwrap())
            .collect();

        let result = panic::catch_unwind(|| reject_datetimes_future(&dates));
        assert!(result.is_ok());
    }

    #[test]
    fn reject_datetimes_future_with_future_dates() {
        let now = Utc::now();
        let date_strings = vec![
            now.to_rfc3339(),
            (now - chrono::Duration::days(1)).to_rfc3339(),
            (now + chrono::Duration::hours(1)).to_rfc3339(), // future date
        ];
        let dates: Vec<DateTime<FixedOffset>> = date_strings.into_iter()
            .map(|s| DateTime::parse_from_rfc3339(&s).unwrap())
            .collect();

        let result = panic::catch_unwind(|| reject_datetimes_future(&dates));
        assert!(result.is_err());
    }


    #[test]
    fn reject_datetimes_unsorted_sorted_dates() {
        let date_strings = vec![
            "2023-05-27T09:10:00+00:00",
            "2023-05-27T10:10:00+00:00",
            "2023-05-27T11:10:00+00:00",
        ];
        let dates: Vec<DateTime<FixedOffset>> = date_strings.into_iter()
            .map(|s| DateTime::parse_from_rfc3339(s).unwrap())
            .collect();

        let result = panic::catch_unwind(|| reject_datetimes_unsorted(&dates));
        assert!(result.is_ok());
    }

    #[test]
    fn reject_datetimes_unsorted_unsorted_dates() {
        let date_strings = vec![
            "2023-05-27T09:10:00+00:00",
            "2023-05-27T11:10:00+00:00", // out of order
            "2023-05-27T10:10:00+00:00",
        ];
        let dates: Vec<DateTime<FixedOffset>> = date_strings.into_iter()
            .map(|s| DateTime::parse_from_rfc3339(s).unwrap())
            .collect();

        let result = panic::catch_unwind(|| reject_datetimes_unsorted(&dates));
        assert!(result.is_err());
    }
}
//  }}}

