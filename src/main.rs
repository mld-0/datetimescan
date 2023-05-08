//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use datetimescan::search_datetimes::search_datetimes;
//use datetimescan::parse_datetime::parse_datetime;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self,BufReader};
use std::path::Path;
//  Notes:
//  {{{
//  }}}

fn print_search_datetimes_results(datetimes_and_locations: &Vec<(String, usize, usize)>)
{
    for (datetime, line_number, position) in datetimes_and_locations {
        println!("Found datetime: {} at line {} position {}", datetime, line_number, position);
    }
}

fn main() 
{
    let matches = App::new("datetimescan")
        .version("0.0.1")
        .about("Finds datetime strings in the input")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .get_matches();

    let datetimes_and_locations = if let Some(file_path) = matches.value_of("input") {
        let file = File::open(&Path::new(file_path)).expect("Failed to open the file");
        search_datetimes(BufReader::new(file))
    } else {
        let stdin = io::stdin();
        search_datetimes(stdin.lock())
    };

    print_search_datetimes_results(&datetimes_and_locations);
}

