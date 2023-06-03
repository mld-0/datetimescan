//  vim-modelines: {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

use crate::convert_seconds::ConvertSeconds;

use clap::ArgMatches;
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use std::fmt::Write;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

macro_rules! out
{
    ($opt:expr, $fmt:expr) => {
        match $opt {
            Some(ref mut out) => writeln!(out, $fmt).unwrap(),
            None => println!($fmt),
        }
    };
    ($opt:expr, $fmt:expr, $($arg:tt)*) => {
        match $opt {
            Some(ref mut out) => writeln!(out, $fmt, $($arg)*).unwrap(),
            None => println!($fmt, $($arg)*),
        }
    };
}

pub struct Printer {
    output: Option<Box<dyn Write>>,
}

impl Printer {
    pub fn new_with_writer(output: Box<dyn Write>) -> Printer {
        Printer {
            output: Some(output),
        }
    }
    pub fn new(_arg_matches: &ArgMatches) -> Printer {
        Printer::default()
    }
    pub fn default() -> Printer {
        Printer { output: None }
    }

    pub fn print_datetimes_and_locations(
        &mut self,
        datetimes_and_locations: &Vec<(String, usize, usize)>,
    ) {
        let ofs = "\t".to_string();
        for (datetime, line_number, position) in datetimes_and_locations {
            out!(
                self.output,
                "{}{}{}{}{}",
                datetime,
                ofs,
                line_number,
                ofs,
                position
            );
        }
    }

    pub fn print_deltas(&mut self, deltas: &Vec<i64>) {
        for delta in deltas {
            out!(self.output, "{}", delta);
        }
    }

    pub fn print_counts_datetimes_grouped(
        &mut self,
        datetimes_grouped: &HashMap<String, Vec<DateTime<FixedOffset>>>,
    ) {
        let mut intervals: Vec<String> = datetimes_grouped.keys().cloned().collect();
        intervals.sort();
        if intervals.len() == 1 && intervals[0] == "all" {
            out!(
                self.output,
                "{}",
                datetimes_grouped.get("all").unwrap().len()
            );
        } else {
            for interval in &intervals {
                out!(
                    self.output,
                    "{}: {}",
                    interval,
                    datetimes_grouped.get(interval).unwrap().len()
                );
            }
        }
    }

    pub fn print_splits_per_interval(
        &mut self,
        splits_per_interval: &HashMap<String, Vec<u64>>,
        unit: &str,
    ) {
        let mut intervals: Vec<String> = splits_per_interval.keys().cloned().collect();
        intervals.sort();
        if intervals.len() == 1 && intervals[0] == "all" {
            for split in splits_per_interval.get("all").unwrap() {
                out!(self.output, "{}", split.convert_seconds(unit));
            }
        } else {
            for interval in &intervals {
                let splits = splits_per_interval
                    .get(interval)
                    .unwrap()
                    .iter()
                    .map(|x| x.convert_seconds(unit))
                    .collect::<Vec<String>>()
                    .join(", ");
                out!(self.output, "{}: {}", interval, splits);
            }
        }
    }

    pub fn print_sum_splits_per_interval(
        &mut self,
        sum_splits_per_interval: &HashMap<String, u64>,
        unit: &str,
    ) {
        let mut intervals: Vec<String> = sum_splits_per_interval.keys().cloned().collect();
        intervals.sort();
        if intervals.len() == 1 && intervals[0] == "all" {
            let sum_in_output_unit = sum_splits_per_interval
                .get("all")
                .unwrap()
                .convert_seconds(unit);
            out!(self.output, "{}", sum_in_output_unit);
        } else {
            for interval in &intervals {
                let sum_in_output_unit = sum_splits_per_interval
                    .get(interval)
                    .unwrap()
                    .convert_seconds(unit);
                out!(self.output, "{}: {}", interval, sum_in_output_unit);
            }
        }
    }
}
