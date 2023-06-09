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

use crate::search_datetimes::search_datetimes;
use crate::parse_datetime::{parse_datetimes, parse_datetime};
use crate::delta_datetimes::{delta_datetimes, split_deltas};
use crate::group_datetimes::group_datetimes;
use crate::printer::{Printer, get_printer_writer};

use chrono::{DateTime, FixedOffset, Utc};
use clap::ArgMatches;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn run(matches: &ArgMatches) {
    let mut writer = get_printer_writer(matches);
    let mut printer = match writer.as_deref_mut() {
        Some(w) => Printer::new(Some(w)),
        None => Printer::default(),
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

pub fn locate(matches: &ArgMatches, printer: &mut Printer)
{
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    if matches.is_present("no_locations") {
        printer.print_datetimes_no_locations(&datetimes_and_locations);
    } else {
        printer.print_datetimes_and_locations(&datetimes_and_locations);
    }
}

#[allow(unused_variables)]
pub fn parse(matches: &ArgMatches, printer: &mut Printer)
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn count(matches: &ArgMatches, printer: &mut Printer)
{
    let datetimes_grouped = get_datetimes_grouped(matches);
    printer.print_counts_datetimes_grouped(&datetimes_grouped);
}

#[allow(unused_variables)]
pub fn convert(matches: &ArgMatches, printer: &mut Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

#[allow(unused_variables)]
pub fn filter(matches: &ArgMatches, printer: &mut Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn deltas(matches: &ArgMatches, printer: &mut Printer)
{
    let deltas = get_deltas(matches);
    printer.print_deltas(&deltas);
}

pub fn splits(matches: &ArgMatches, printer: &mut Printer) 
{
    let unit = matches.value_of("unit").expect("expect `matches` argument 'unit'");
    let splits_per_interval = get_splits_per_interval(matches);
    printer.print_splits_per_interval(&splits_per_interval, unit);
}

pub fn sum(matches: &ArgMatches, printer: &mut Printer) 
{
    let unit = matches.value_of("unit").expect("expect `matches` argument 'unit'");
    let sum_splits_per_interval = get_sum_splits_per_interval(matches);
    printer.print_sum_splits_per_interval(&sum_splits_per_interval, unit);
}

#[allow(unused_variables)]
pub fn groupsum(matches: &ArgMatches, printer: &mut Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}

#[allow(unused_variables)]
pub fn wpm(matches: &ArgMatches, printer: &mut Printer) 
{
    unimplemented!("UNIMPLEMENTED");
}



fn get_datetimes_and_locations(matches: &ArgMatches) -> Vec<(String, usize, usize)>
{
    let datetimes_and_locations = if let Some(file_path) = matches.value_of("input") {
        let file = File::open(Path::new(file_path)).expect("Failed to open the `matches` file 'input'");
        search_datetimes(BufReader::new(file))
    } else {
        let stdin = io::stdin();
        search_datetimes(stdin.lock())
    };
    datetimes_and_locations
}

fn get_datetimes_parsed_with_strs_and_positions(matches: &ArgMatches) -> (Vec<DateTime<FixedOffset>>, Vec<(String, usize, usize)>, Vec<bool>)
{
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    let datetimes_strs = datetimes_and_locations.iter().map(|(s, _, _)| s.to_string()).collect();
    let datetimes_parsed = parse_datetimes(&datetimes_strs);
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

    (datetimes_filtered, datetimes_and_locations, indexes_filter)
}

fn get_datetimes_parsed(matches: &ArgMatches) -> Vec<DateTime<FixedOffset>>
{
    let (datetimes_parsed, _, _) = get_datetimes_parsed_with_strs_and_positions(matches);
    datetimes_parsed
}

fn get_datetimes_grouped(matches: &ArgMatches) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    let interval = matches.value_of("per").expect("expected `matches` argument 'per'");
    let datetimes_parsed = get_datetimes_parsed(matches);
    let datetimes_grouped = group_datetimes(&datetimes_parsed, interval);
    datetimes_grouped
}

fn get_deltas(matches: &ArgMatches) -> Vec<i64>
{
    let allow_negative = matches.is_present("allow_negative");
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    deltas
}

fn _get_splits(matches: &ArgMatches) -> Vec<u64>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").expect("expect `matches` argument 'timeout'")
        .parse().unwrap();
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    let splits = split_deltas(&deltas, timeout);
    splits
}

fn get_splits_per_interval(matches: &ArgMatches) -> HashMap<String, Vec<u64>>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").expect("expect `matches` argument 'timeout'")
        .parse().unwrap();
    let datetimes_grouped = get_datetimes_grouped(matches);
    let mut splits_per_interval = HashMap::new();
    for (interval, datetimes) in &datetimes_grouped {
        let deltas = delta_datetimes(datetimes, allow_negative);
        let splits = split_deltas(&deltas, timeout);
        if !splits.is_empty() {
            splits_per_interval.insert(interval.clone(), splits);
        }
    }
    log::trace!("get_splits_per_interval(), result=({:?})", splits_per_interval);
    splits_per_interval
}

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

fn parse_filter_start_end(matches: &ArgMatches) -> (Option<DateTime<FixedOffset>>, Option<DateTime<FixedOffset>>)
{
    let filter_start = {
        if matches.is_present("filter_start") {
            let filter_start_str = matches.value_of("filter_start").unwrap();
            let filter_start = parse_datetime(filter_start_str);
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
            let filter_end = parse_datetime(filter_end_str);
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

