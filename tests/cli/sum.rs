
#[cfg(test)]
mod test_cli_sum {

    use datetimescan::get_parser;
    use datetimescan::subcommands;
    use datetimescan::printer::Printer;
    use crate::cli;

    #[test]
    fn test_empty_stdin() {
    }

    #[test]
    fn test_empty_file() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "sum", "--input", &path_input];
        let expected = r"";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--input", &path_input];
        let expected = 
r"113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--no_unsorted", "--input", &path_input];
        let expected = 
r"113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--no_future", "--input", &path_input];
        let expected = 
r"113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_all() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--per", "all", "--input", &path_input];
        let expected = 
r"113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--per", "y", "--input", &path_input];
        let expected = 
r"2023: 113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-05: 113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "sum", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-05-05: 113
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--input", &path_input];
        let expected = 
r"2256
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_s() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--unit", "s", "--input", &path_input];
        let expected = 
r"2256
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--unit", "m", "--input", &path_input];
        let expected = 
r"37.60
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_h() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--unit", "h", "--input", &path_input];
        let expected = 
r"0.63
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_all() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "all", "--input", &path_input];
        let expected = 
r"2256
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "y", "--input", &path_input];
        let expected = 
r"2023: 2445
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_y_unit_s() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "y", "--unit", "s", "--input", &path_input];
        let expected = 
r"2023: 2445
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_y_unit_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "y", "--unit", "m", "--input", &path_input];
        let expected = 
r"2023: 40.75
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_y_unit_h() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "y", "--unit", "h", "--input", &path_input];
        let expected = 
r"2023: 0.68
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-04: 2445
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-04-19: 2445
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d_unit_s() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "d", "--unit", "s", "--input", &path_input];
        let expected = 
r"2023-04-19: 2445
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d_unit_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "d", "--unit", "m", "--input", &path_input];
        let expected = 
r"2023-04-19: 40.75
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d_unit_h() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "d", "--unit", "h", "--input", &path_input];
        let expected = 
r"2023-04-19: 0.68
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d_unit_hms() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--per", "d", "--unit", "hms", "--input", &path_input];
        let expected = 
r"2023-04-19: 40m45s
";
        run_sum(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_timeout_1200() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--timeout", "1200", "--input", &path_input];
        let expected = 
r"3260
";
        run_sum(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject future_datetimes")]
    fn test_isodatetimes2_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--no_future", "--input", &path_input];
        let expected = r"";
        run_sum(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject out_of_order_datetimes")]
    fn test_isodatetimes2_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "sum", "--no_unsorted", "--input", &path_input];
        let expected = r"";
        run_sum(&args, &expected);
    }


    fn run_sum(args: &Vec<&str>, expected: &str) {
        let parser = get_parser::get_parser();
        let matches = parser.get_matches_from(args);
        let mut buffer = Vec::<u8>::new();
        let mut printer = Printer::new(Some(&mut buffer));
        match matches.subcommand() {
            ("sum", Some(matches)) => subcommands::sum(&matches, &mut printer),
            _ => panic!("wrong subcommand"),
        }
        let result = String::from_utf8(buffer).expect("Failed to convert Printer buffer to String");
        assert_eq!(result, expected);
    }
}

