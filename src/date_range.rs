//  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2
//  Notes:
//  {{{
//  2023-07-18T20:16:44AEST should use an enum instead of y/m/d as a string?
//  }}}

//  Translation of 'tasklogReaderUtil/date_range' into Rust

use chrono::{NaiveDate, Duration, Datelike};

/// Attempts to parse a string into a `NaiveDate` based on its length.
///
/// The function expects the string to be one of the following formats:
/// * "YYYY" (4 characters long)
/// * "YYYY-MM" (7 characters long)
/// * "YYYY-MM-DD" (10 characters long)
///
/// If the string is in the "YYYY" format, it will be interpreted as January 1 of that year.
/// If the string is in the "YYYY-MM" format, it will be interpreted as the first day of that month.
/// If the string is in the "YYYY-MM-DD" format, it will be interpreted as that specific date.
///
/// # Examples
/// ```
/// use chrono::NaiveDate;
/// use datetimescan::date_range::parse_partial_date_str;
/// assert_eq!(parse_partial_date_str("2023"), Some(NaiveDate::from_ymd(2023, 1, 1)));
/// assert_eq!(parse_partial_date_str("2023-03"), Some(NaiveDate::from_ymd(2023, 3, 1)));
/// assert_eq!(parse_partial_date_str("2023-03-03"), Some(NaiveDate::from_ymd(2023, 3, 3)));
/// assert_eq!(datetimescan::date_range::parse_partial_date_str("invalid"), None);
/// ```
///
/// # Arguments
/// * `s` - A string slice that holds the date to parse.
///
/// # Returns
/// * `Some(NaiveDate)` if the string could be successfully parsed.
/// * `None` if the string could not be parsed or was not in the expected format.
pub fn parse_partial_date_str(s: &str) -> Option<NaiveDate> {
    match s.len() {
        4 => NaiveDate::from_ymd_opt(s.parse().ok()?, 1, 1),
        7 => {
            let parts: Vec<&str> = s.split('-').collect();
            NaiveDate::from_ymd_opt(parts[0].parse().ok()?, parts[1].parse().ok()?, 1)
        },
        10 => NaiveDate::parse_from_str(s, "%Y-%m-%d").ok(),
        _ => None,
    }
}

pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl DateRange 
{
    pub fn new(start: &str, end: &str) -> DateRange 
    {
        let s = parse_partial_date_str(start);
        if s.is_none() {
            panic!("invalid date start=({})", start);
        }
        let e = parse_partial_date_str(end);
        if e.is_none() {
            panic!("invalid date end=({})", end);
        }
        DateRange { start: s.unwrap(), end: e.unwrap(), }
    }

    pub fn new_from_str_range(dates: Vec<&str>) -> DateRange
    {
        let mut parsed_dates = dates.iter().map(|x| { parse_partial_date_str(x).expect("Invalid date given for `DateRange::new_from_str_range`") }).collect::<Vec<_>>();
        parsed_dates.sort();
        let first = &parsed_dates[0];
        let last = &parsed_dates[parsed_dates.len()-1];
        DateRange { start: first.clone(), end: last.clone(), }
    }

    /// Get a vec of all yearly/monthly/daily `NaiveDate`s between start and end dates (inclusive)
    ///
    /// For a monthly range, all days=1, and for a yearly range, all months=1 and days=1
    /// 
    /// # Arguments
    /// * `range_type` - A string specify yearly/monthly/daily (must be y/m/d)
    ///
    /// # Panics
    /// Will panic if `range_type` is not (y/m/d)
    ///
    /// # Returns
    /// `Vec<NaiveDate>` a vector of datetime objects
    pub fn get_dates(&self, range_type: &str) -> Vec<NaiveDate> 
    {
        match range_type {
            "y" | "Y" => self.get_years(),
            "m" | "M" => self.get_months(),
            "d" | "D" => self.get_days(),
            _ => panic!("Invalid range_type=({:?}) for `DateRange` (must be y/m/d)", range_type),
        }
    }

    fn get_days(&self) -> Vec<NaiveDate> 
    {
        let mut result = Vec::new();
        let mut date = self.start;
        while date <= self.end {
            result.push(date);
            date += Duration::days(1);
        }
        result
    }

    fn get_months(&self) -> Vec<NaiveDate> 
    {
        let mut result = Vec::new();
        let mut date = self.start;
        while date <= self.end {
            result.push(NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap());
            if date.month() == 12 {
                date = NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap();
            } else {
                date = NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap();
            }
        }
        result
    }

    fn get_years(&self) -> Vec<NaiveDate> 
    {
        let mut result = Vec::new();
        for y in self.start.year()..=self.end.year() {
            result.push(NaiveDate::from_ymd_opt(y, 1, 1).unwrap());
        }
        result
    }

    pub fn is_date_in_range(&self, _date_str: &str) -> bool
    {
        unimplemented!();
    }

    pub fn get_missing_dates(_dates: Vec<&str>, _range_type: &str) -> Vec<String>
    {
        unimplemented!();
    } 

}

