
#[cfg(test)]
mod test {
    use datetimescan::convert_seconds::ConvertSeconds;

    #[test]
    fn test_i64() {
        let value: i64 = 3600;
        assert_eq!(value.convert_seconds("hms"), "1h");
        assert_eq!(value.convert_seconds("h"), "1.00");
        assert_eq!(value.convert_seconds("m"), "60.00");
        assert_eq!(value.convert_seconds("s"), "3600");
    }

    #[test]
    fn test_u64() {
        let value: u64 = 3600;
        assert_eq!(value.convert_seconds("hms"), "1h");
        assert_eq!(value.convert_seconds("h"), "1.00");
        assert_eq!(value.convert_seconds("m"), "60.00");
        assert_eq!(value.convert_seconds("s"), "3600");
    }

    #[test]
    #[should_panic(expected = "unit=(d) must equal 'hms' / 'h' / 'm' / 's'")]
    fn test_invalid_unit_i64() {
        let value: i64 = 3600;
        let _ = value.convert_seconds("d");
    }

    #[test]
    #[should_panic(expected = "unit=(d) must equal 'hms' / 'h' / 'm' / 's'")]
    fn test_invalid_unit_u64() {
        let value: u64 = 3600;
        let _ = value.convert_seconds("d");
    }

    #[test]
    fn test_i64_hms() {
        let seconds: i64 = 3661;
        assert_eq!(seconds.convert_seconds("hms"), "1h01m01s");

        let seconds: i64 = -3661;
        assert_eq!(seconds.convert_seconds("hms"), "-1h01m01s");
    }

    #[test]
    fn test_u64_hms() {
        let seconds: u64 = 3661;
        assert_eq!(seconds.convert_seconds("hms"), "1h01m01s");
    }

    #[test]
    fn test_i64_hms_zeros() {
        let seconds: i64 = 3600;
        assert_eq!(seconds.convert_seconds("hms"), "1h");

        let seconds: i64 = 60;
        assert_eq!(seconds.convert_seconds("hms"), "1m");

        let seconds: i64 = 0;
        assert_eq!(seconds.convert_seconds("hms"), "0s");
    }

    #[test]
    fn test_u64_hms_zeros() {
        let seconds: u64 = 3600;
        assert_eq!(seconds.convert_seconds("hms"), "1h");

        let seconds: u64 = 60;
        assert_eq!(seconds.convert_seconds("hms"), "1m");

        let seconds: u64 = 0;
        assert_eq!(seconds.convert_seconds("hms"), "0s");
    }
}

