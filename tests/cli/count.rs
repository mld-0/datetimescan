//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

#[cfg(test)]
mod test_cli_count {

    use datetimescan::create_arg_parser;
    use datetimescan::subcommands;
    use datetimescan::printer::Printer;
    use crate::cli;

    #[test]
    fn test_empty_stdin() {
    }


    #[test]
    fn test_empty_file() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "count", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    fn test_empty_file_per_all() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "count", "--per", "all", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    fn test_empty_file_per_y() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "count", "--per", "y", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    fn test_empty_file_per_m() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "count", "--per", "m", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    fn test_empty_file_per_d() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "count", "--per", "d", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }


    #[test]
    fn test_isodatetimes_1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();

        let args = vec!["datetimescan", "count", "--input", &path_input];
        let expected = 
r"5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--no_unsorted", "--input", &path_input];
        let expected = 
r"5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--no_future", "--input", &path_input];
        let expected = 
r"5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_per_all() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--per", "all", "--input", &path_input];
        let expected = 
r"5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--per", "y", "--input", &path_input];
        let expected = 
"2023: 5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-05: 5
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "count", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-05-05: 5
";
        run_count(&args, &expected);
    }


    #[test]
    fn test_isodatetimes_2() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--input", &path_input];
        let expected = 
r"56
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_2_per_all() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--per", "all", "--input", &path_input];
        let expected = 
r"56
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_2_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--per", "y", "--input", &path_input];
        let expected = 
r"2023: 55
2999: 1
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_2_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-04: 55
2999-04: 1
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_2_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-04-19: 55
2999-04-19: 1
";
        run_count(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject out_of_order_datetimes")]
    fn test_isodatetimes_2_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--no_unsorted", "--input", &path_input];
        let expected = 
r"56
";
        run_count(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject future_datetimes")]
    fn test_isodatetimes_2_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "count", "--no_future", "--input", &path_input];
        let expected = 
r"56
";
        run_count(&args, &expected);
    }


    #[test]
    fn test_worklog_sample() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--input", &path_input];
        let expected = 
r"346
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_all() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "all", "--input", &path_input];
        let expected = 
r"346
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_y() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "y", "--input", &path_input];
        let expected = 
r"2022: 62
2023: 284
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_m() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "m", "--input", &path_input];
        let expected = 
r"2022-06: 6
2022-07: 15
2022-09: 9
2022-11: 13
2022-12: 19
2023-01: 5
2023-02: 15
2023-03: 12
2023-04: 64
2023-05: 188
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_d() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "d", "--input", &path_input];
        let expected =
r"2022-06-05: 5
2022-06-06: 1
2022-07-13: 13
2022-07-14: 2
2022-09-03: 9
2022-11-05: 13
2022-12-11: 8
2022-12-27: 11
2023-01-01: 5
2023-02-20: 15
2023-03-29: 12
2023-04-13: 64
2023-05-01: 20
2023-05-02: 9
2023-05-03: 11
2023-05-04: 4
2023-05-05: 17
2023-05-06: 12
2023-05-08: 9
2023-05-09: 6
2023-05-11: 5
2023-05-14: 29
2023-05-15: 13
2023-05-18: 4
2023-05-19: 9
2023-05-20: 22
2023-05-22: 12
2023-05-23: 6
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_m_filter() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "m", "--filter_start", "2022-10-01T00:00:00", "--filter_end", "2023-03-31T23:59:59", "--input", &path_input];
        let expected = 
r"2022-11: 13
2022-12: 19
2023-01: 5
2023-02: 15
2023-03: 12
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_worklog_sample_per_m_filter_invert() {
        let path_input = cli::utils::get_path_worklogscrambledsamples();
        let args = vec!["datetimescan", "count", "--per", "m", "--filter_invert", "--filter_start", "2022-10-01T00:00:00", "--filter_end", "2023-03-31T23:59:59", "--input", &path_input];
        let expected = 
r"2022-06: 6
2022-07: 15
2022-09: 9
2023-04: 64
2023-05: 188
";
        run_count(&args, &expected);
    }


    fn run_count(args: &Vec<&str>, expected: &str) {
        let parser = create_arg_parser::create_arg_parser();
        let matches = parser.get_matches_from(args);
        let mut buffer = Vec::<u8>::new();
        let mut printer = Printer::new(Some(&mut buffer));
        match matches.subcommand() {
            ("count", Some(matches)) => subcommands::count(&matches, &mut printer),
            _ => panic!("wrong subcommand"),
        }
        let result = String::from_utf8(buffer).expect("Failed to convert Printer buffer to String");
        assert_eq!(result, expected);
    }
}

