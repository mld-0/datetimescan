//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
#![allow(unused)]
use datetimescan::search_datetimes::search_datetimes;
use datetimescan::parse_datetime::parse_datetime;
use datetimescan::delta_datetimes::datetime_difference_seconds;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;

//  Notes:
//  {{{
//  }}}

fn main() 
{
    let input_arg = Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Select input file (default=stdin)")
        .takes_value(true);

    let per_arg = Arg::with_name("per")
        .long("per")
        .value_name("INTERVAL")
        .help("Count datetimes per interval (d/m/y) (default=d)")
        .takes_value(true)
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

    if let Some(scan_matches) = matches.subcommand_matches("scan") {
        run_scan(&scan_matches);
    } else if let Some(count_matches) = matches.subcommand_matches("count") {
        run_count(&count_matches);
    } else if let Some(deltas_matches) = matches.subcommand_matches("deltas") {
        run_deltas(&deltas_matches);
    } else if let Some(splits_matches) = matches.subcommand_matches("splits") {
        run_splits(&splits_matches);
    } else if let Some(sum_matches) = matches.subcommand_matches("sum") {
        run_sum(&sum_matches);
    } else if let Some(wpm_matches) = matches.subcommand_matches("wpm") {
        run_wpm(&wpm_matches);
    } else {
        eprintln!("No subcommand was used. Use --help for more information.");
    }
}

fn run_scan(scan_matches: &ArgMatches)
{
    let datetimes_and_locations = run_search_datetimes(&scan_matches);
    print_search_datetimes_results(&datetimes_and_locations);
}

fn run_count(count_matches: &ArgMatches)
{
    let datetimes_and_locations = run_search_datetimes(&count_matches);
    let interval = count_matches.value_of("per").unwrap();
    unimplemented!();
}

fn run_deltas(deltas_matches: &ArgMatches)
{
    let datetimes_and_locations = run_search_datetimes(&deltas_matches);
    let allow_negative = deltas_matches.is_present("allow_negative");
    unimplemented!();
}

fn run_splits(splits_matches: &ArgMatches) 
{
    let datetimes_and_locations = run_search_datetimes(&splits_matches);
    let allow_negative = false;
    unimplemented!();
}

fn run_sum(sum_matches: &ArgMatches) 
{
    let datetimes_and_locations = run_search_datetimes(&sum_matches);
    let interval = sum_matches.value_of("per").unwrap();
    let allow_negative = false;
    unimplemented!();
}

fn run_wpm(wpm_matches: &ArgMatches) 
{
    let datetimes_and_locations = run_search_datetimes(&wpm_matches);
    unimplemented!();
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

fn print_search_datetimes_results(datetimes_and_locations: &Vec<(String, usize, usize)>)
{
    let ofs = "\t".to_string();
    for (datetime, line_number, position) in datetimes_and_locations {
        println!("{}{}{}{}{}", datetime, ofs, line_number, ofs, position);
    }
}

