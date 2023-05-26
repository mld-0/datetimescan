
pub trait ConvertSeconds {
    fn convert_seconds(&self, unit: &str) -> String;
    fn get_divisor(&self, unit: &str) -> i64 {
        if unit == "h" {
            3600
        } else if unit == "m" {
            60
        } else if unit == "s" {
            1
        } else {
            panic!("unit=({}) must equal 'h' / 'm' / 's'", unit);
        }
    }
}

impl ConvertSeconds for i64 {
    fn convert_seconds(&self, unit: &str) -> String {
        let divisor = self.get_divisor(unit);
        if divisor == 1 {
            format!("{}", self)
        } else {
            format!("{:.2}", *self as f64 / divisor as f64)
        }
    }
}

impl ConvertSeconds for u64 {
    fn convert_seconds(&self, unit: &str) -> String {
        let divisor = self.get_divisor(unit);
        if divisor == 1 {
            format!("{}", self)
        } else {
            format!("{:.2}", *self as f64 / divisor as f64)
        }
    }
}

