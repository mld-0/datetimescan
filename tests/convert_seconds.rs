
#[cfg(test)]
mod test_convert_seconds {
    use datetimescan::convert_seconds::ConvertSeconds;

    #[test]
    fn test_convert_seconds_i64() {
        let value: i64 = 3600;
        assert_eq!(value.convert_seconds("h"), "1.00");
        assert_eq!(value.convert_seconds("m"), "60.00");
        assert_eq!(value.convert_seconds("s"), "3600");
    }

    #[test]
    fn test_convert_seconds_u64() {
        let value: u64 = 3600;
        assert_eq!(value.convert_seconds("h"), "1.00");
        assert_eq!(value.convert_seconds("m"), "60.00");
        assert_eq!(value.convert_seconds("s"), "3600");
    }

    #[test]
    #[should_panic(expected = "unit=(d) must equal 'h' / 'm' / 's'")]
    fn test_invalid_unit_i64() {
        let value: i64 = 3600;
        let _ = value.convert_seconds("d");
    }

    #[test]
    #[should_panic(expected = "unit=(d) must equal 'h' / 'm' / 's'")]
    fn test_invalid_unit_u64() {
        let value: u64 = 3600;
        let _ = value.convert_seconds("d");
    }
}

