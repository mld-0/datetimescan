//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

//#![allow(unused)]

use datetimescan::subcommands::{scan, parse, convert, count, deltas, splits, sum, wpm};

use clap::{App, Arg, SubCommand};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

//  Notes:
//  {{{
//  2023-05-14T23:12:44AEST subcommand 'sum' should be 'sums'?
//  2023-05-20T23:22:12AEST 'version' should be kept in one place (and it shouldn't be in the declaration of 'parser'(?)
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

    let unsigned_validator = |value: String| -> Result<(), String> {
        match value.parse::<u64>() {
            Ok(_num) => { Ok( () ) },
            Err(_) => Err("Invalid unsigned-integer value".to_string()),
        }
    };

    let timeout = Arg::with_name("timeout")
        .long("timeout")
        .value_name("TIMEOUT")
        .help("Max positive delta not considered a split")
        .takes_value(true)
        .validator(unsigned_validator)
        .default_value("300");

    let parser = App::new("datetimescan")
        .version("0.0.1")
        .about("Util for finding/analysing datetime strings in input")
        .arg(input_arg.clone().global(true)) 
        .subcommand(
            SubCommand::with_name("scan")
                .about("")
            )
        .subcommand(
            SubCommand::with_name("parse")
                .about("")
            )
        .subcommand(
            SubCommand::with_name("convert")
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
                .arg(timeout.clone())
            )
        .subcommand(
            SubCommand::with_name("sum")
                .about("")
                .arg(per_arg.clone())
                .arg(timeout.clone())
            )
        .subcommand(
                SubCommand::with_name("wpm")
                .about("")
            );

    let matches = parser.get_matches();
    log::trace!("main(), matches=({:?})", matches);

    if let Some(scan_matches) = matches.subcommand_matches("scan") {
        scan(&scan_matches)
    } else if let Some(parse_matches) = matches.subcommand_matches("parse") {
        parse(&parse_matches)
    } else if let Some(convert_matches) = matches.subcommand_matches("convert") {
        convert(&convert_matches)
    } else if let Some(count_matches) = matches.subcommand_matches("count") {
        count(&count_matches)
    } else if let Some(deltas_matches) = matches.subcommand_matches("deltas") {
        deltas(&deltas_matches)
    } else if let Some(splits_matches) = matches.subcommand_matches("splits") {
        splits(&splits_matches)
    } else if let Some(sum_matches) = matches.subcommand_matches("sum") {
        sum(&sum_matches)
    } else if let Some(wpm_matches) = matches.subcommand_matches("wpm") {
        wpm(&wpm_matches)
    } else {
        eprintln!("No subcommand was used. Use --help for more information.");
    }

    log::trace!("main(), DONE");
}

