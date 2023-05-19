//  vim-modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]

use crate::search_datetimes::search_datetimes;
use crate::parse_datetime::{parse_datetime, parse_datetimes};
use crate::delta_datetimes::{delta_datetimes, datetime_difference_seconds, split_deltas};
use crate::group_datetimes::group_datetimes;

use chrono::{DateTime, FixedOffset};
use clap::ArgMatches;
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;
use log::{error, warn, info, debug, trace};
use std::collections::HashMap;

pub fn scan(scan_matches: &ArgMatches)
{
    let datetimes_and_locations = run_search_datetimes(&scan_matches);
    print_search_datetimes_results(&datetimes_and_locations);
}

pub fn parse(parse_matches: &ArgMatches)
{
    unimplemented!("UNIMPLEMENTED");
}

pub fn count(count_matches: &ArgMatches)
{
    let interval = count_matches.value_of("per").unwrap();
    let datetimes_and_locations = run_search_datetimes(&count_matches);
    let datetimes_parsed = run_parse_datetimes(&datetimes_and_locations);
    let datetimes_grouped = group_datetimes(&datetimes_parsed, interval);
    print_datetimes_grouped_counts(datetimes_grouped);
}

pub fn deltas(deltas_matches: &ArgMatches)
{
    let allow_negative = deltas_matches.is_present("allow_negative");
    let datetimes_and_locations = run_search_datetimes(&deltas_matches);
    let datetimes_parsed = run_parse_datetimes(&datetimes_and_locations);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    print_deltas(&deltas);
}

pub fn splits(splits_matches: &ArgMatches) 
{
    let allow_negative = false;
    let timeout = 300;
    let datetimes_and_locations = run_search_datetimes(&splits_matches);
    let datetimes_parsed = run_parse_datetimes(&datetimes_and_locations);
    let deltas = delta_datetimes(&datetimes_parsed, allow_negative);
    let splits = split_deltas(&deltas, timeout);
    print_splits(&splits);
}

pub fn sum(sum_matches: &ArgMatches) 
{
    let interval = sum_matches.value_of("per").unwrap();
    let allow_negative = false;
    let datetimes_and_locations = run_search_datetimes(&sum_matches);
    unimplemented!("UNIMPLEMENTED");
}

pub fn wpm(wpm_matches: &ArgMatches) 
{
    let allow_negative = false;
    let datetimes_and_locations = run_search_datetimes(&wpm_matches);
    unimplemented!("UNIMPLEMENTED");
}


fn run_search_datetimes(matches: &ArgMatches) -> Vec<(String, usize, usize)>
{
    if let Some(file_path) = matches.value_of("input") {
        let file = File::open(&Path::new(file_path)).expect("Failed to open the file");
        search_datetimes(BufReader::new(file))
    } else {
        let stdin = io::stdin();
        search_datetimes(stdin.lock())
    }
}

fn run_parse_datetimes(datetimes_and_locations: &Vec<(String, usize, usize)>) -> Vec<DateTime<FixedOffset>>
{

    let datetimes_strs = datetimes_and_locations.iter().map(|(s, _, _)| s.to_string()).collect();
    let datetimes_parsed = parse_datetimes(&datetimes_strs);
    if datetimes_parsed.is_none() {
        panic!("failed to parse datetimes_strs=({:?})", datetimes_strs);
    }
    datetimes_parsed.unwrap()
}

fn print_search_datetimes_results(datetimes_and_locations: &Vec<(String, usize, usize)>)
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

fn print_datetimes_grouped_counts(datetimes_grouped: HashMap<String, Vec<DateTime<FixedOffset>>>)
{
    let mut intervals: Vec<String> = datetimes_grouped.keys().cloned().collect();
    intervals.sort();
    for interval in &intervals {
        println!("{}: {}", interval, datetimes_grouped.get(interval).unwrap().len());
    }
}

fn print_splits(splits: &Vec<u64>) 
{
    for split in splits {
        println!("{}", split);
    }
}

