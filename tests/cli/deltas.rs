
#[cfg(test)]
mod test_cli_deltas {

    use datetimescan::get_parser;
    use datetimescan::subcommands;
    use datetimescan::printer::Printer;
    use crate::cli;

    #[test]
    fn test_empty_file() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "deltas", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "deltas", "--input", &path_input];
        let expected = 
r"41
21
34
17
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "deltas", "--no_unsorted", "--input", &path_input];
        let expected = 
r"41
21
34
17
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes1_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "deltas", "--no_future", "--input", &path_input];
        let expected = 
r"41
21
34
17
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "deltas", "--input", &path_input];
        let expected = 
r"36
170
30799612908
0
5
71
27
30
23
88
82
14
72
0
189
193
17
71
85
23
25
27
38
95
8
14
41
49
23
39
97
48
7
26
21
31
28
5
26
2466
10
15
34
28
348
27
15
15
46
48
95
20
52
656
7
";
        run_count(&args, &expected);
    }

    #[test]
    fn test_isodatetimes2_allow_negative() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "deltas", "--allow_negative", "--input", &path_input];
        let expected = 
r"36
170
30799612908
-30799612719
5
71
27
30
23
88
82
14
72
-62
189
193
17
71
85
23
25
27
38
95
8
14
41
49
23
39
97
48
7
26
21
31
28
5
26
2466
10
15
34
28
348
27
15
15
46
48
95
20
52
656
7
";
        run_count(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject out_of_order_datetimes")]
    fn test_isodatetimes2_no_unsorted() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "deltas", "--no_unsorted", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }

    #[test]
    #[should_panic(expected = "reject future_datetimes")]
    fn test_isodatetimes2_no_future() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "deltas", "--no_future", "--input", &path_input];
        let expected = "";
        run_count(&args, &expected);
    }


    fn run_count(args: &Vec<&str>, expected: &str) {
        let parser = get_parser::get_parser();
        let matches = parser.get_matches_from(args);
        let mut buffer = Vec::<u8>::new();
        let mut printer = Printer::new(Some(&mut buffer));
        match matches.subcommand() {
            ("deltas", Some(matches)) => subcommands::deltas(&matches, &mut printer),
            _ => panic!("wrong subcommand"),
        }
        let result = String::from_utf8(buffer).expect("Failed to convert Printer buffer to String");
        assert_eq!(result, expected);
    }
}

