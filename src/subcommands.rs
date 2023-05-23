//  vim-modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

//  Notes:
//  {{{
//  2023-05-20T21:40:41AEST 'sum()', Handling the interval="all" case(?)
//  2023-05-20T22:59:09AEST please rename s/run_subcommands/subcommands/ 
//  2023-05-20T22:59:25AEST subcommands? (or commands?)
//  }}}

//  Ongoing: 2023-05-20T23:47:11AEST explain the size of the difference between the sum of 'splits' and 'sum' for textWithIsoDatetimes-2.txt -> 2256 for 'splits' and 2445 for 'sum'

use crate::search_datetimes::search_datetimes;
use crate::parse_datetime::parse_datetimes;
use crate::delta_datetimes::{delta_datetimes, split_deltas};
use crate::group_datetimes::group_datetimes;

use chrono::{DateTime, FixedOffset};
use clap::ArgMatches;
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub fn scan(scan_matches: &ArgMatches)
{
    let datetimes_and_locations = get_datetimes_and_locations(scan_matches);
    print_datetimes_and_locations(&datetimes_and_locations);
}

#[allow(unused_variables)]
pub fn parse(parse_matches: &ArgMatches)
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn count(count_matches: &ArgMatches)
{
    let datetimes_grouped = get_datetimes_grouped(count_matches);
    print_counts_datetimes_grouped(&datetimes_grouped);
}

#[allow(unused_variables)]
pub fn convert(convert_matches: &ArgMatches) 
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn deltas(deltas_matches: &ArgMatches)
{
    let deltas = get_deltas(deltas_matches);
    print_deltas(&deltas);
}

pub fn splits(splits_matches: &ArgMatches) 
{
    let splits = get_splits(splits_matches);
    print_splits(&splits);
}

pub fn sum(sum_matches: &ArgMatches) 
{
    let sum_splits_per_interval = get_sum_splits_per_interval(sum_matches);
    print_sum_splits_per_interval(&sum_splits_per_interval);
}

#[allow(unused_variables)]
pub fn wpm(wpm_matches: &ArgMatches) 
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
    let datetimes_and_locations = get_datetimes_and_locations(matches);
    let datetimes_strs = datetimes_and_locations.iter().map(|(s, _, _)| s.to_string()).collect();
    let datetimes_parsed = parse_datetimes(&datetimes_strs);
    if datetimes_parsed.is_none() {
        panic!("failed to parse datetimes_strs=({:?})", datetimes_strs);
    }
    let datetimes_parsed = datetimes_parsed.unwrap();
    datetimes_parsed
}

fn get_datetimes_grouped(matches: &ArgMatches) -> HashMap<String, Vec<DateTime<FixedOffset>>>
{
    let interval = matches.value_of("per").unwrap();
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

fn get_splits(matches: &ArgMatches) -> Vec<u64>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").unwrap().parse().unwrap();
    let datetimes_parsed = get_datetimes_parsed(matches);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    let splits = split_deltas(&deltas, timeout);
    splits
}

fn get_splits_per_interval(matches: &ArgMatches) -> HashMap<String, Vec<u64>>
{
    let allow_negative = false;
    let timeout: u64 = matches.value_of("timeout").unwrap().parse().unwrap();
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

fn print_splits(splits: &Vec<u64>) 
{
    for split in splits {
        println!("{}", split);
    }
}

fn print_sum_splits_per_interval(sum_splits_per_interval: &HashMap<String, u64>)
{
    let mut intervals: Vec<String> = sum_splits_per_interval.keys().cloned().collect();
    intervals.sort();
    if intervals.len() == 1 && intervals[0] == "all" {
        println!("{}", sum_splits_per_interval.get("all").unwrap());
    } else {
        for interval in &intervals {
            println!("{}: {}", interval, sum_splits_per_interval.get(interval).unwrap());
        }
    }
}

