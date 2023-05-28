
pub trait ConvertSeconds {
    fn convert_seconds(&self, unit: &str) -> String;
    fn get_divisor(&self, unit: &str) -> i64 {
        if unit.to_lowercase() == "h" {
            3600
        } else if unit.to_lowercase() == "m" {
            60
        } else {
            panic!("unit=({}) must equal 'hms' / 'h' / 'm' / 's'", unit);
        }
    }
    fn get_hms(&self, mut s: u64) -> String {
        let mut h = 0_u64;
        let mut m = 0_u64;
        while s >= 3600 {
            h += 1;
            s -= 3600;
        }
        while s >= 60 {
            m += 1;
            s -= 60;
        }
        let mut result = String::new();
        if h > 0 {
            result.push_str(&format!("{}h", h));
        }
        if m > 0 {
            if result.len() == 0 {
                result.push_str(&format!("{}m", m));
            } else {
                result.push_str(&format!("{:02}m", m));
            }
        }
        if s > 0 || result.len() == 0 {
            if result.len() == 0 {
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
        if unit.to_lowercase() == "s" {
            format!("{}", self)
        } else if unit.to_lowercase() == "hms" {
            if *self < 0 {
                format!("-{}", self.get_hms(self.abs() as u64))
            } else {
                format!("{}", self.get_hms(*self as u64))
            }
        } else {
            format!("{:.2}", *self as f64 / self.get_divisor(unit) as f64)
        }
    }
}

impl ConvertSeconds for u64 {
    fn convert_seconds(&self, unit: &str) -> String {
        if unit.to_lowercase() == "s" {
            format!("{}", self)
        } else if unit.to_lowercase() == "hms" {
            format!("{}", self.get_hms(*self))
        } else {
            format!("{:.2}", *self as f64 / self.get_divisor(unit) as f64)
        }
    }
}

