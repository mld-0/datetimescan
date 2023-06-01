//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
use regex::Regex;
use std::io::BufRead;
use std::fmt::Debug;

//  Notes:
//  {{{
//  2023-05-23T23:16:07AEST (how to) handle cases like '2023-02-09T03:57:30<>', '2023-02-10T04:18:03TZ', '2023-02-10T04:20:15T', <ect> (do we ignore/reject them, or grab everything before the timezone?) (do we want start/end word boundries) (need 'fancy-regex' to handle lookahead/lookbehind)
//  2023-05-23T23:18:01AEST regex should be constructed of smaller pieces (instead of declared as one blob)
//  }}}

/// Searches for datetime strings in the format iso-format in the provided reader.
///
/// The function takes a generic `BufRead` type as an argument, allowing it to work with both standard input and file handles.
/// Supported formats: '2023-05-08T19:29:50AEST', '2023-05-08T19:29:50UTC', '2023-05-08T19:29:50+1000', '2023-05-08T19:29:50+10:00', '2023-05-08 19:29:50', '2023-05-08T19:29:50'
///
/// # Arguments
/// * `reader` - A type implementing `BufRead` from which the function will read lines.
///
/// # Returns
/// A vector of tuples containing the found datetime strings, their line numbers, and positions.
pub fn search_datetimes<R: BufRead + Debug>(reader: R) -> Vec<(String, usize, usize)> 
{
    log::trace!("search_datetimes(), reader=({:?})", reader);
    let datetime_regex = Regex::new(
        //r"(?P<datetime>\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(?:[A-Z]{3,4}|[+-]\d{2}:?\d{2})?)\b",
        r"(?P<datetime>\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(?:[A-Z]{3,4}|[+-]\d{2}:?\d{2})?)",
    )
    .unwrap();
    log::trace!("search_datetimes(), datetime_regex=({})", datetime_regex);
    log::trace!("search_datetimes(), reader=({:?})", reader);
    let mut results: Vec<(String, usize, usize)> = Vec::new();
    let mut line_number = 1;
    for line in reader.lines().map(|l| l.unwrap()) {
        for capture in datetime_regex.captures_iter(&line) {
            let datetime = capture[1].to_string();
            let start_position = capture.get(1).unwrap().start();
            results.push( (datetime, line_number, start_position) );
        }
        line_number += 1;
    }
    log::trace!("search_datetimes(), results=({:?})", results);
    results
}

