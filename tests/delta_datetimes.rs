
mod test_delta_datetimes {
    use chrono::DateTime;
    use datetimescan::delta_datetimes::delta_datetimes; 

    #[test]
    fn test_empty_input() {
        let input = vec![];
        let allow_negatives = true;
        let expected_output = vec![];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_single_element() {
        let input = vec![DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap()];
        let allow_negatives = true;
        let expected_output = vec![];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_positive_deltas() {
        let input = vec![
            DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:02:30+00:00").unwrap(),
        ];
        let allow_negatives = true;
        let expected_output = vec![60, 90];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_negative_deltas() {
        let input = vec![
            DateTime::parse_from_rfc3339("2023-05-11T00:02:30+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap(),
        ];
        let allow_negatives = true;
        let expected_output = vec![-90, -60];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_negative_deltas_not_allowed() {
        let input = vec![
            DateTime::parse_from_rfc3339("2023-05-11T00:02:30+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap(),
        ];
        let allow_negatives = false;
        let expected_output = vec![0, 0];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_mixed_deltas() {
        let input = vec![
            DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:00:30+00:00").unwrap(),
        ];
        let allow_negatives = true;
        let expected_output = vec![60, -30];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_mixed_deltas_negatives_not_allowed() {
        let input = vec![
            DateTime::parse_from_rfc3339("2023-05-11T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:01:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-05-11T00:00:30+00:00").unwrap(),
        ];
        let allow_negatives = false;
        let expected_output = vec![60, 0];

        let result = delta_datetimes(&input, allow_negatives);
        assert_eq!(result, expected_output);
    }
}

mod test_difference_seconds {
    use chrono::DateTime;
    use datetimescan::delta_datetimes::datetime_difference_seconds; 

    #[test]
    fn test_same_datetimes() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
        let dt2 = dt1.clone();

        assert_eq!(datetime_difference_seconds(dt1, dt2), 0);
    }

    #[test]
    fn test_positive_difference() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-01T01:00:00+00:00").unwrap();

        assert_eq!(datetime_difference_seconds(dt1, dt2), 3600);
    }

    #[test]
    fn test_negative_difference() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-01T01:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();

        assert_eq!(datetime_difference_seconds(dt1, dt2), -3600);
    }

    #[test]
    fn test_different_offsets() {
        let dt1 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+03:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00-02:00").unwrap();

        assert_eq!(datetime_difference_seconds(dt1, dt2), 5 * 3600);
    }

    #[test]
    fn test_years_appart() {
        let dt1 = DateTime::parse_from_rfc3339("2019-05-01T00:00:00+00:00").unwrap();
        let dt2 = DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap();

        assert_eq!(datetime_difference_seconds(dt1, dt2), 1461 * 24 * 60 * 60);
    }
}

mod test_split_deltas {
    use datetimescan::delta_datetimes::split_deltas;

    #[test]
    fn test_split_deltas_contiguous_periods() {
        let deltas = vec![100, 150, 500, 100];
        let timeout = 300;
        assert_eq!(split_deltas(&deltas, timeout), vec![250, 100]);
    }

    #[test]
    fn test_split_deltas_non_contiguous_periods() {
        let deltas = vec![400, 100, 350, 150, 400, 200, 350, 250];
        let timeout = 300;
        assert_eq!(split_deltas(&deltas, timeout), vec![100, 150, 200, 250]);
    }

    #[test]
    fn test_split_deltas_single_contiguous_period() {
        let deltas = vec![100, 150, 200];
        let timeout = 300;
        assert_eq!(split_deltas(&deltas, timeout), vec![450]);
    }

    #[test]
    fn test_split_deltas_no_contiguous_periods() {
        let deltas = vec![400, 350, 400, 350];
        let timeout = 300;
        assert_eq!(split_deltas(&deltas, timeout), vec![]);
    }

    #[test]
    fn test_split_deltas_zero_timeout() {
        let deltas = vec![100, 150, 200, 50];
        let timeout = 0;
        assert_eq!(split_deltas(&deltas, timeout), vec![]);
    }
}

