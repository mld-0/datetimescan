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
            )
        .subcommand(
                SubCommand::with_name("wpm")
                .about("")
            )
        .get_matches();

    if let Some(scan_matches) = matches.subcommand_matches("scan") {
        run_scan(&matches);
    } else if let Some(count_matches) = matches.subcommand_matches("count") {
        run_count(&matches);
    } else if let Some(deltas_matches) = matches.subcommand_matches("deltas") {
        run_deltas(&matches);
    } else if let Some(splits_matches) = matches.subcommand_matches("splits") {
        run_splits(&matches);
    } else if let Some(sum_matches) = matches.subcommand_matches("sum") {
        run_sum(&matches);
    } else if let Some(sum_wpm) = matches.subcommand_matches("wpm") {
        run_wpm(&matches);
    } else {
        eprintln!("No subcommand was used. Use --help for more information.");
    }
}

fn run_scan(matches: &ArgMatches)
{
    let datetimes_and_locations = run_search_datetimes(&matches);
    print_search_datetimes_results(&datetimes_and_locations);
}

fn run_count(matches: &ArgMatches)
{
    unimplemented!();
}

fn run_deltas(matches: &ArgMatches)
{
    unimplemented!();
}

fn run_splits(matches: &ArgMatches) 
{
    unimplemented!();
}

fn run_sum(matches: &ArgMatches) 
{
    unimplemented!();
}

fn run_wpm(matches: &ArgMatches) 
{
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

