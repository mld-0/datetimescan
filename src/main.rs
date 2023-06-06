//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

use datetimescan::subcommands;
use datetimescan::get_parser;

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
    let parser = get_parser::get_parser();
    let matches = parser.get_matches();
    log::trace!("main(), matches=({:?})", matches);
    subcommands::run(&matches);
    log::trace!("main(), DONE");
}

