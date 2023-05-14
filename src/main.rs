//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]

use datetimescan::run_subcommands::{scan, count, deltas, splits, sum, wpm};

use chrono::{DateTime, FixedOffset};
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;
use log::{error, warn, info, debug, trace};

//  Notes:
//  {{{
//  }}}

fn main() 
{
    env_logger::init();

    let input_arg = Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Select input file (default=stdin)")
        .takes_value(true);

    let per_arg = Arg::with_name("per")
        .long("per")
        .value_name("INTERVAL")
        .help("Count/Sum datetimes per interval (d/m/y/all) (default=d)")
        .takes_value(true)
        .possible_values(&["d", "m", "y", "all"])
        .default_value("d");

    let allow_negative = Arg::with_name("allow_negative")
        .long("allow_negative")
        .value_name("ALLOW_NEGATIVE")
        .help("Set negative deltas to 0")
        .takes_value(false);

    let matches = App::new("datetimescan")
        .version("0.0.1")
        .about("Finds datetime strings in the input")
        .arg(input_arg.clone().global(true)) 
        .subcommand(
            SubCommand::with_name("scan")
                .about("")
            )
        .subcommand(
            SubCommand::with_name("count")
                .about("")
                .arg(per_arg.clone())
            )
        .subcommand(
            SubCommand::with_name("deltas")
                .about("")
                .arg(allow_negative.clone())
            )
        .subcommand(
            SubCommand::with_name("splits")
                .about("")
            )
        .subcommand(
            SubCommand::with_name("sum")
                .about("")
                .arg(per_arg.clone())
            )
        .subcommand(
                SubCommand::with_name("wpm")
                .about("")
            )
        .get_matches();

    log::trace!("main(), matches=({:?})", matches);
    if let Some(scan_matches) = matches.subcommand_matches("scan") {
        scan(&scan_matches);
    } else if let Some(count_matches) = matches.subcommand_matches("count") {
        count(&count_matches);
    } else if let Some(deltas_matches) = matches.subcommand_matches("deltas") {
        deltas(&deltas_matches);
    } else if let Some(splits_matches) = matches.subcommand_matches("splits") {
        splits(&splits_matches);
    } else if let Some(sum_matches) = matches.subcommand_matches("sum") {
        sum(&sum_matches);
    } else if let Some(wpm_matches) = matches.subcommand_matches("wpm") {
        wpm(&wpm_matches);
    } else {
        eprintln!("No subcommand was used. Use --help for more information.");
    }
    log::trace!("main(), DONE");
}

