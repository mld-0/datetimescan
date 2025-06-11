
#[cfg(test)]
mod test_cli_splits {

    use datetimescan::create_arg_parser;
    use datetimescan::subcommands;
    use datetimescan::printer::Printer;
    use crate::cli;


    #[test]
    fn test_empty_file() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "splits", "--input", &path_input];
        let expected = "";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_unit_s() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--unit", "s", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_unit_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--unit", "m", "--input", &path_input];
        let expected = 
r"1.88
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_unit_h() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--unit", "h", "--input", &path_input];
        let expected = 
r"0.03
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_unit_hms() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--unit", "hms", "--input", &path_input];
        let expected = 
r"1m53s
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_all() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--per", "all", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--per", "y", "--input", &path_input];
        let expected = 
r"2023: 113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-05: 113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-05-05: 113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_timeout_1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--timeout", "1", "--input", &path_input];
        let expected = 
r"";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_timeout_30() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--timeout", "30", "--input", &path_input];
        let expected = 
r"21
17
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_timeout_1200() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--timeout", "1200", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--no_unsorted", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "splits", "--no_future", "--input", &path_input];
        let expected = 
r"113
";
        run_splits(&args, &expected);
    }


    #[test]
    fn test_isodatetimes2() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--input", &path_input];
        let expected = 
r"206
1638
87
318
7
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_s() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--unit", "s", "--input", &path_input];
        let expected = 
r"206
1638
87
318
7
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--unit", "m", "--input", &path_input];
        let expected = 
r"3.43
27.30
1.45
5.30
0.12
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_h() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--unit", "h", "--input", &path_input];
        let expected = 
r"0.06
0.46
0.02
0.09
0.00
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_unit_hms() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--unit", "hms", "--input", &path_input];
        let expected = 
r"3m26s
27m18s
1m27s
5m18s
7s
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_timeout_1() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--timeout", "1", "--input", &path_input];
        let expected = 
r"";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_timeout_600() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--timeout", "600", "--input", &path_input];
        let expected = 
r"206
1638
753
7
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_timeout_1200() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--timeout", "1200", "--input", &path_input];
        let expected = 
r"206
1638
1416
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_y() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--per", "y", "--input", &path_input];
        let expected = 
r"2023: 2033, 87, 318, 7
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_m() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--per", "m", "--input", &path_input];
        let expected = 
r"2023-04: 2033, 87, 318, 7
";
        run_splits(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_per_d() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--per", "d", "--input", &path_input];
        let expected = 
r"2023-04-19: 2033, 87, 318, 7
";
        run_splits(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject future_datetimes")]
    fn test_isodatetimes2_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--no_future", "--input", &path_input];
        let expected = r"";
        run_splits(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject out_of_order_datetimes")]
    fn test_isodatetimes2_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "splits", "--no_unsorted", "--input", &path_input];
        let expected = r"";
        run_splits(&args, &expected);
    }


    fn run_splits(args: &Vec<&str>, expected: &str) {
        let parser = create_arg_parser::create_arg_parser();
        let matches = parser.get_matches_from(args);
        let mut buffer = Vec::<u8>::new();
        let mut printer = Printer::new(Some(&mut buffer));
        match matches.subcommand() {
            ("splits", Some(matches)) => subcommands::splits(&matches, &mut printer),
            _ => panic!("wrong subcommand"),
        }
        let result = String::from_utf8(buffer).expect("Failed to convert Printer buffer to String");
        assert_eq!(result, expected);
    }
}

