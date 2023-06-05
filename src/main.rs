//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

use datetimescan::subcommands;
use clap::{App, Arg, SubCommand};

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

//  Notes:
//  {{{
//  2023-05-14T23:12:44AEST subcommand 'sum' should be 'sums'?
//  2023-05-20T23:22:12AEST 'version' should be kept in one place (and it shouldn't be in the declaration of 'parser'(?)
//  2023-05-26T17:15:04AEST printing warnings/errors from 'log' without RUST_LOG set?
//  2023-05-28T21:57:06AEST clap, argument 'possible_values' included in the help message
//  2023-05-28T21:57:28AEST clap, if provided 'possible_values', will it allow anything else?
//  }}}

fn main() 
{
    env_logger::init();

    //  Common arguments:
    let input_arg = Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Select input file (default=stdin)")
        .takes_value(true);

    let filter_start = Arg::with_name("filter_start")
        .long("filter_start")
        .value_name("FILTER_START")
        .help("Exclude datetimes before");

    let filter_end = Arg::with_name("filter_end")
        .long("filter_end")
        .value_name("FILTER_END")
        .help("Exclude datetimes before");

    let filter_invert = Arg::with_name("filter_invert")
        .long("filter_invert")
        .help("Invert filter excluded items")
        .takes_value(false);

    let no_unsorted = Arg::with_name("no_unsorted")
        .long("no_unsorted")
        .help("Do not allow out-of-order datetimes in input")
        .takes_value(false);

    let no_future = Arg::with_name("no_future")
        .long("no_future")
        .help("Do not allow datetimes after the present")
        .takes_value(false);

    //  Subcommand arguments:
    let per_arg = Arg::with_name("per")
        .long("per")
        .value_name("INTERVAL")
        .help("Count/Sum datetimes per interval (d/m/y/all) (default=all)")
        .takes_value(true)
        .possible_values(&["d", "m", "y", "all"])
        .default_value("all");

    let allow_negative = Arg::with_name("allow_negative")
        .long("allow_negative")
        .help("Set negative deltas to 0")
        .takes_value(false);

    let validator_unsigned = |value: String| -> Result<(), String> {
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
        .validator(validator_unsigned)
        .default_value("300");

    let unit = Arg::with_name("unit")
        .long("unit")
        .value_name("UNIT")
        .help("Output in seconds/minutes/hours")
        .possible_values(&["s", "m", "h", "hms"])
        .default_value("s");

    let parser = App::new("datetimescan")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Utility for finding/analysing datetime strings in input")
        .arg(input_arg.global(true)) 
        .arg(no_future.global(true))
        .arg(no_unsorted.global(true))
        .arg(filter_start.global(true))
        .arg(filter_end.global(true))
        .arg(filter_invert.global(true))
        .subcommand(
            SubCommand::with_name("locate")
                .about("List datetime matches and their locations")
            )
        .subcommand(
            SubCommand::with_name("parse")
                .about("List datetime matches in specified output format (without parsing them)")
            )
        .subcommand(
            SubCommand::with_name("convert")
                .about("Print input, with datetimes converted to specified output format")
            )
        .subcommand(
            SubCommand::with_name("filter")
                .about("Print input, excluding lines containing datetimes outside filter range")
            )
        .subcommand(
            SubCommand::with_name("count")
                .about("Count datetimes per interval")
                .arg(per_arg.clone())
            )
        .subcommand(
            SubCommand::with_name("deltas")
                .about("Report seconds elapsed between each datetime match")
                .arg(allow_negative.clone())
            )
        .subcommand(
            SubCommand::with_name("splits")
                .about("Report length of continuous deltas where no delta > timeout")
                .arg(per_arg.clone())
                .arg(timeout.clone())
                .arg(unit.clone())
            )
        .subcommand(
            SubCommand::with_name("sum")
                .about("Sum splits per interval")
                .arg(per_arg.clone())
                .arg(timeout.clone())
                .arg(unit.clone())
            )
        .subcommand(
                SubCommand::with_name("wpm")
                .about("")
            );

    let matches = parser.get_matches();
    log::trace!("main(), matches=({:?})", matches);

    if let Some(arg_matches) = matches.subcommand_matches("locate") {
        subcommands::locate(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("parse") {
        subcommands::parse(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("convert") {
        subcommands::convert(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("filter") {
        subcommands::filter(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("count") {
        subcommands::count(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("deltas") {
        subcommands::deltas(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("splits") {
        subcommands::splits(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("sum") {
        subcommands::sum(arg_matches)
    } else if let Some(arg_matches) = matches.subcommand_matches("wpm") {
        subcommands::wpm(arg_matches)
    } else {
        eprintln!("No subcommand was used. Use --help for more information.");
    }

    log::trace!("main(), DONE");
}

