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
//  }}}

//  Ongoing: 2023-05-20T23:47:11AEST explain the size of the difference between the sum of 'splits' and 'sum' for textWithIsoDatetimes-2.txt -> 2256 for 'splits' and 2445 for 'sum'

use crate::search_datetimes::search_datetimes;
use crate::parse_datetime::{parse_datetimes, parse_datetime};
use crate::delta_datetimes::{delta_datetimes, split_deltas};
use crate::group_datetimes::group_datetimes;
use crate::convert_seconds::ConvertSeconds;

use chrono::{DateTime, FixedOffset, Utc};

use clap::ArgMatches;
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn scan(arg_matches: &ArgMatches)
{
    let datetimes_and_locations = get_datetimes_and_locations(arg_matches);
    print_datetimes_and_locations(&datetimes_and_locations);
}

#[allow(unused_variables)]
pub fn parse(arg_matches: &ArgMatches)
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn count(arg_matches: &ArgMatches)
{
    let datetimes_grouped = get_datetimes_grouped(arg_matches);
    print_counts_datetimes_grouped(&datetimes_grouped);
}

#[allow(unused_variables)]
pub fn convert(arg_matches: &ArgMatches) 
{
    unimplemented!("UNIMPLEMENTED");
}

#[allow(unused_variables)]
pub fn filter(arg_matches: &ArgMatches) 
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn deltas(arg_matches: &ArgMatches)
{
    let deltas = get_deltas(arg_matches);
    print_deltas(&deltas);
}

pub fn splits(arg_matches: &ArgMatches) 
{
    let unit = arg_matches.value_of("unit").expect("expect argument unit");
    let splits_per_interval = get_splits_per_interval(arg_matches);
    print_splits_per_interval(&splits_per_interval, unit);
}

pub fn sum(arg_matches: &ArgMatches) 
{
    let unit = arg_matches.value_of("unit").expect("expect argument unit");
    let sum_splits_per_interval = get_sum_splits_per_interval(arg_matches);
    print_sum_splits_per_interval(&sum_splits_per_interval, unit);
}

#[allow(unused_variables)]
pub fn wpm(arg_matches: &ArgMatches) 
{
    unimplemented!("UNIMPLEMENTED");
}


fn get_datetimes_and_locations(matches: &ArgMatches) -> Vec<(String, usize, usize)>
{
    let datetimes_and_locations = if let Some(file_path) = matches.value_of("input") {
        let file = File::open(&Path::new(file_path)).expect("Failed to open the file");
        search_datetimes(BufReader::new(file))
    } else {
        let stdin = io::stdin();
        search_datetimes(stdin.lock())
    };
    datetimes_and_locations
}

fn get_datetimes_parsed(matches: &ArgMatches) -> Vec<DateTime<FixedOffset>>
{
    let (datetimes_parsed, _, _) = get_datetimes_parsed_with_strs_and_positions(matches);
    datetimes_parsed
}

fn get_datetimes_parsed_with_strs_and_positions(matches: &ArgMatches) -> (Vec<DateTime<FixedOffset>>, Vec<(String, usize, usize)>, Vec<bool>)
{
    let no_future = matches.is_present("no_future");
    let no_unsorted = matches.is_present("no_unsorted");
    let filter_start = parse_filter_start(matches);
    let filter_end = parse_filter_end(matches);
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    let datetimes_strs = datetimes_and_locations.iter().map(|(s, _, _)| s.to_string()).collect();
    let datetimes_parsed = parse_datetimes(&datetimes_strs);
    if datetimes_parsed.is_none() {
        panic!("failed to parse datetimes_strs=({:?})", datetimes_strs);
    }
    let datetimes_parsed = datetimes_parsed.unwrap();
    let valid_indexes = filter_datetimes_valid_indexes(&datetimes_parsed, &filter_start, &filter_end);
    let datetimes_filtered = datetimes_parsed.iter()
        .zip(valid_indexes.iter())
        .filter(|(_, &include)| include)
        .map(|(&x, _)| x)
        .collect();
    if no_future {
        reject_datetimes_future(&datetimes_filtered);
    }
    if no_unsorted {
        reject_datetimes_unsorted(&datetimes_filtered);
    }
    (datetimes_filtered, datetimes_and_locations, valid_indexes)
}

fn get_datetimes_grouped(matches: &ArgMatches) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    let interval = matches.value_of("per").expect("expected argument per");
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
    let timeout: u64 = matches.value_of("timeout").expect("expect argument timeout")
        .parse().unwrap();
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    let splits = split_deltas(&deltas, timeout);
    splits
}

fn get_splits_per_interval(matches: &ArgMatches) -> HashMap<String, Vec<u64>>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").expect("expect argument timeout")
        .parse().unwrap();
    let datetimes_grouped = get_datetimes_grouped(matches);
    let mut splits_per_interval = HashMap::new();
    for (interval, datetimes) in &datetimes_grouped {
        let deltas = delta_datetimes(datetimes, allow_negative);
        let splits = split_deltas(&deltas, timeout);
        if splits.len() > 0 {
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

fn print_datetimes_and_locations(datetimes_and_locations: &Vec<(String, usize, usize)>)
{
    let ofs = "\t".to_string();
    for (datetime, line_number, position) in datetimes_and_locations {
        println!("{}{}{}{}{}", datetime, ofs, line_number, ofs, position);
    }
}

fn print_deltas(deltas: &Vec<i64>) 
{
    for delta in deltas {
        println!("{}", delta);
    }
}

fn print_counts_datetimes_grouped(datetimes_grouped: &HashMap<String, Vec<DateTime<FixedOffset>>>)
{
    let mut intervals: Vec<String> = datetimes_grouped.keys().cloned().collect();
    intervals.sort();
    if intervals.len() == 1 && intervals[0] == "all" {
        println!("{}", datetimes_grouped.get("all").unwrap().len());
    } else {
        for interval in &intervals {
            println!("{}: {}", interval, datetimes_grouped.get(interval).unwrap().len());
        }
    }
}

fn print_splits_per_interval(splits_per_interval: &HashMap<String, Vec<u64>>, unit: &str) 
{
    let mut intervals: Vec<String> = splits_per_interval.keys().cloned().collect();
    intervals.sort();
    if intervals.len() == 1 && intervals[0] == "all" {
        for split in splits_per_interval.get("all").unwrap() {
            println!("{}", split.convert_seconds(unit));
        }
    } else {
        for interval in &intervals {
            let splits = splits_per_interval.get(interval).unwrap()
                .iter()
                .map(|x| x.convert_seconds(unit))
                .collect::<Vec<String>>()
                .join(", ");
            println!("{}: {}", interval, splits);
        }
    }
}

fn print_sum_splits_per_interval(sum_splits_per_interval: &HashMap<String, u64>, unit: &str)
{
    let mut intervals: Vec<String> = sum_splits_per_interval.keys().cloned().collect();
    intervals.sort();
    if intervals.len() == 1 && intervals[0] == "all" {
        let sum_in_output_unit = sum_splits_per_interval.get("all").unwrap().convert_seconds(unit);
        println!("{}", sum_in_output_unit);
    } else {
        for interval in &intervals {
            let sum_in_output_unit = sum_splits_per_interval.get(interval).unwrap().convert_seconds(unit);
            println!("{}: {}", interval, sum_in_output_unit);
        }
    }
}

fn filter_datetimes_valid_indexes(datetimes: &Vec<DateTime<FixedOffset>>, filter_start: &Option<DateTime<FixedOffset>>, filter_end: &Option<DateTime<FixedOffset>>) -> Vec<bool> 
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
            future_datetimes.push(date.clone());
        }
    }
    if future_datetimes.len() > 0 {
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
    if out_of_order_datetimes.len() > 0 {
        panic!("reject out_of_order_datetimes=({:?})", out_of_order_datetimes);
    }
}

fn parse_filter_start(matches: &ArgMatches) -> Option<DateTime<FixedOffset>>
{
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
}

fn parse_filter_end(matches: &ArgMatches) -> Option<DateTime<FixedOffset>>
{
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
}


//  Placing tests in 'subcommands/tests.rs' works, but produces YCM error
#[cfg(test)]
mod tests {
    use super::*;

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
}

