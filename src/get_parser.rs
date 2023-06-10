use clap::{App, Arg, SubCommand};

pub fn get_parser() -> clap::App<'static, 'static>
{
    //  Common arguments:
    let input_arg = Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("FILE")
        .help("Select input file (default=stdin)")
        .takes_value(true);

    let output_arg = Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("OUT_FILE")
        .help("Select output file (default=stdout)")
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
    let no_locations = Arg::with_name("no_locations")
        .long("no_locations")
        .help("Do not include positions of datetime matches")
        .takes_value(false);

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
        .arg(output_arg.global(true))
        .arg(no_future.global(true))
        .arg(no_unsorted.global(true))
        .arg(filter_start.global(true))
        .arg(filter_end.global(true))
        .arg(filter_invert.global(true))

        .subcommand(
            SubCommand::with_name("locate")
                .about("List datetime matches and their locations")
                .arg(no_locations.clone())
            )

        .subcommand(
            SubCommand::with_name("parse")
                .about("UNIMPLEMENTED List datetime matches in specified output format (without parsing them)")
            )

        .subcommand(
            SubCommand::with_name("convert")
                .about("UNIMPLEMENTED Print input, with datetimes converted to specified output format")
            )

        .subcommand(
            SubCommand::with_name("filter")
                .about("UNIMPLEMENTED Print input, excluding lines containing datetimes outside filter range")
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
                SubCommand::with_name("groupsum")
                .arg(per_arg.clone())
                .arg(timeout.clone())
                .arg(unit.clone())
            )

        .subcommand(
                SubCommand::with_name("wpm")
                .about("UNIMPLEMENTED")
            )

        ;

    parser
}

