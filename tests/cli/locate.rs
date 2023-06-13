//  vim-modelines:  {{{3
//  vim: set tabstop=4 modeline modelines=10:
//  vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
//  {{{2

//  Notes:
//  2023-06-12T22:39:17AEST will linting replace literal tabs in `expected` strings?


#[cfg(test)]
mod test_cli_locate {

    use datetimescan::get_parser;
    use datetimescan::subcommands;
    use datetimescan::printer::Printer;
    use crate::cli;

    #[test]
    fn test_empty_stdin() {
        //  mocking stdin?
    }

    #[test]
    fn test_empty_file() {
        let path_input = cli::utils::get_path_empty();
        let args = vec!["datetimescan", "locate", "--input", &path_input];
        let expected = "";
        run_locate(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1() {
        let path_input = cli::utils::get_path_textwithisodatetimes1();
        let args = vec!["datetimescan", "locate", "--input", &path_input];
        let expected = 
r"2023-05-05T19:34:42+1000	1	0
2023-05-05T19:35:23+1000	2	0
2023-05-05T19:35:44+1000	3	10
2023-05-05T19:36:18+1000	4	0
2023-05-05T19:36:35+1000	5	0
";
        run_locate(&args, &expected);
    }

    #[test]
    fn test_isodatetimes_1_write_to_file() {
    }

    #[test]
    fn test_isodatetimes_2() {
        let path_input = cli::utils::get_path_textwithisodatetimes2();
        let args = vec!["datetimescan", "locate", "--input", &path_input];
        let expected = 
r"2023-04-19T22:07:40AEST	1	10
2023-04-19T22:08:16AEST	2	0
2023-04-19T22:11:06AEST	4	10
2999-04-19T22:12:54AEST	6	0
2023-04-19T22:14:15AEST	10	0
2023-04-19T22:14:20AEST	12	6
2023-04-19T22:15:31AEST	13	0
2023-04-19T22:15:58AEST	15	9
2023-04-19T22:16:28AEST	16	0
2023-04-19T22:16:51AEST	18	0
2023-04-19T22:18:19AEST	19	0
2023-04-19T22:19:41AEST	21	6
2023-04-19T22:19:55AEST	22	0
2023-04-19T22:21:07AEST	24	9
2023-04-19T22:20:05AEST	25	6
2023-04-19T22:23:14AEST	26	0
2023-04-19T22:26:27AEST	28	0
2023-04-19T22:26:44AEST	29	0
2023-04-19T22:27:55AEST	31	0
2023-04-19T22:29:20AEST	32	0
2023-04-19T22:29:43AEST	34	9
2023-04-19T22:30:08AEST	35	0
2023-04-19T22:30:35AEST	37	0
2023-04-19T22:31:13AEST	38	0
2023-04-19T22:32:48AEST	39	0
2023-04-19T22:32:56AEST	41	0
2023-04-19T22:33:10AEST	42	10
2023-04-19T22:33:51AEST	43	6
2023-04-19T22:34:40AEST	44	0
2023-04-19T22:35:03AEST	46	6
2023-04-19T22:35:42AEST	47	0
2023-04-19T22:37:19AEST	49	9
2023-04-19T22:38:07AEST	50	0
2023-04-19T22:38:14AEST	52	0
2023-04-19T22:38:40AEST	53	0
2023-04-19T22:39:01AEST	55	0
2023-04-19T22:39:32AEST	56	0
2023-04-19T22:40:00AEST	59	0
2023-04-19T22:40:05AEST	60	0
2023-04-19T22:40:31AEST	61	0
2023-04-19T23:21:37AEST	63	0
2023-04-19T23:21:47AEST	64	0
2023-04-19T23:22:02AEST	67	0
2023-04-19T23:22:36AEST	68	0
2023-04-19T23:23:04AEST	69	0
2023-04-19T23:28:52AEST	71	10
2023-04-19T23:29:19AEST	73	7
2023-04-19T23:29:34AEST	74	0
2023-04-19T23:29:49AEST	76	6
2023-04-19T23:30:35AEST	77	0
2023-04-19T23:31:23AEST	80	0
2023-04-19T23:32:58AEST	81	0
2023-04-19T23:33:18AEST	83	6
2023-04-19T23:34:10AEST	84	0
2023-04-19T23:45:06AEST	86	0
2023-04-19T23:45:13AEST	87	0
";
        run_locate(&args, &expected);
    }

    fn run_locate(args: &Vec<&str>, expected: &str) {
        let parser = get_parser::get_parser();
        let matches = parser.get_matches_from(args);
        let mut buffer = Vec::<u8>::new();
        let mut printer = Printer::new(Some(&mut buffer));
        match matches.subcommand() {
            ("locate", Some(matches)) => subcommands::locate(&matches, &mut printer),
            _ => panic!("wrong subcommand"),
        }
        let result = String::from_utf8(buffer).expect("Failed to convert Printer buffer to String");
        assert_eq!(result, expected);
    }

}

        //  {{{
        //let temp_dir = env::temp_dir();
        //let temp_file = Builder::new()
        //    .prefix("test_empty")
        //    .tempfile_in(temp_dir)
        //    .expect("Failed to create tempfile")
        //    .keep()
        //    .expect("Failed to keep tempfile")
        //    ;
        //let path_output = "/tmp/datetimescan_locate_test_empty.txt".to_owned();
        //let args: Vec<String> = vec![ "datetimescan".to_owned(), "locate".to_owned(), "--input".to_owned(), path, "--output".to_owned(), path_output, ];
        //let parser = get_parser::get_parser();
        //let matches = parser.get_matches_from(&args);
    //let mut buffer = Vec::new();
    //{
    //    let stdout = std::io::stdout(); // get the global stdout entity
    //    let mut handle = stdout.lock(); // acquire a lock on it
    //    let backup = handle
    //        .get_ref()
    //        .duplicate() // duplicate the underlying handle
    //        .expect("duplication failed");
    //    handle.set_ref(&mut buffer); // reassign stdout
    //        subcommands::run(&matches); // call function
    //    handle.set_ref(&backup); // reassign stdout back to its original value
    //}
        //let orig_stdout = std::io::stdout();
        //let orig_stdout = orig_stdout.lock();
        //let orig_stdout = orig_stdout.clone();
        //// Run the function
        //{
        //    let _ = std::io::set_stdout(fake_stdout);
        //    subcommands::run(&matches);
        //    //subcommands::locate(test_args);
        //    fake_stdout = std::io::stdout();
        //}
        //// Reset stdout
        //let _ = std::io::set_stdout(orig_stdout);
        //// Validate the output
        //let mut output = String::new();
        //let _ = fake_stdout.read_to_string(&mut output);
        ////assert_eq!(output, test_data);
        //  }}}
