
pub trait ConvertSeconds {
    fn convert_seconds(&self, unit: &str) -> String;
    fn get_hms(&self, seconds: u64) -> String {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;
        let s = seconds % 60;
        let mut result = String::new();
        if h > 0 {
            result.push_str(&format!("{}h", h));
        }
        if m > 0 {
            if result.is_empty() {
                result.push_str(&format!("{}m", m));
            } else {
                result.push_str(&format!("{:02}m", m));
            }
        }
        if s > 0 || result.is_empty() {
            if result.is_empty() {
                result.push_str(&format!("{}s", s));
            } else {
                result.push_str(&format!("{:02}s", s));
            }
        }
        result
    }
}

impl ConvertSeconds for i64 {
    fn convert_seconds(&self, unit: &str) -> String {
        match unit.to_lowercase().as_str() {
            "hms" => 
                if *self < 0 {
                    format!("-{}", self.get_hms(self.unsigned_abs()))
                } else {
                    self.get_hms(*self as u64)
                }
            "h" => format!("{:.2}", *self as f64 / 3600.0),
            "m" => format!("{:.2}", *self as f64 / 60.0),
            "s" => format!("{}", self),
            _ => panic!("unit=({}) must equal 'hms' / 'h' / 'm' / 's'", unit),
        }
    }
}

impl ConvertSeconds for u64 {
    fn convert_seconds(&self, unit: &str) -> String {
        match unit.to_lowercase().as_str() {
            "hms" => self.get_hms(*self),
            "h" => format!("{:.2}", *self as f64 / 3600.0),
            "m" => format!("{:.2}", *self as f64 / 60.0),
            "s" => format!("{}", self),
            _ => panic!("unit=({}) must equal 'hms' / 'h' / 'm' / 's'", unit),
        }
    }
}

