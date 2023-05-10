use regex::Regex;
use std::io::BufRead;

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
pub fn search_datetimes<R: BufRead>(reader: R) -> Vec<(String, usize, usize)> 
{
    let datetime_regex = Regex::new(
        r"(?P<datetime>\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(?:[A-Z]{3,4}|[+-]\d{2}:?\d{2})?)",
    )
    .unwrap();

    let mut results: Vec<(String, usize, usize)> = Vec::new();

    let mut line_number = 1;
    for line in reader.lines() {
        if let Ok(line) = line {
            for capture in datetime_regex.captures_iter(&line) {
                let datetime = capture[1].to_string();
                let start_position = capture.get(1).unwrap().start();
                results.push( (datetime, line_number, start_position) );
            }
            line_number += 1;
        }
    }
    results
}

