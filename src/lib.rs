//! # Lao DateTime Library
//!
//! A Rust library for formatting and parsing dates in Lao language with Buddhist Era calendar support.
//!
//! ## Features
//! - Format dates in Lao language (ວັນ, ເດືອນ, ປີ)
//! - Buddhist Era (BE) calendar conversion
//! - Lao number formatting (໐໑໒໓໔໕໖໗໘໙)
//! - Parse Lao date strings back to DateTime
//!
//! ## Example
//! ```
//! use Lao_date_format::LaoDateTime;
//!
//! let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0);
//! println!("{}", dt.unwrap().format_lao_full()); // ວັນພຸດ ທີ 27 ເດືອນມີນາ ປີ 2567
//! ```

use std::fmt;

/// Represents a date and time with Lao formatting capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaoDateTime {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl LaoDateTime {
    /// Creates a new LaoDateTime
    ///
    /// # Arguments
    /// * `year` - Gregorian year (e.g., 2024)
    /// * `month` - Month (1-12)
    /// * `day` - Day (1-31)
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0-59)
    /// * `second` - Second (0-59)
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self, DateTimeError> {
        if !(1..=12).contains(&month) {
            return Err(DateTimeError::InvalidMonth);
        }
        if !(1..=31).contains(&day) {
            return Err(DateTimeError::InvalidDay);
        }
        if hour >= 24 {
            return Err(DateTimeError::InvalidHour);
        }
        if minute >= 60 {
            return Err(DateTimeError::InvalidMinute);
        }
        if second >= 60 {
            return Err(DateTimeError::InvalidSecond);
        }

        Ok(LaoDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    /// Get the year in Buddhist Era (BE)
    pub fn year_be(&self) -> i32 {
        self.year + 543
    }

    /// Get the Gregorian year
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Get the month (1-12)
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Get the day
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Get the hour
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Get the second
    pub fn second(&self) -> u8 {
        self.second
    }

    /// Format date in full Lao format
    /// Example: "ວັນພຸດ ທີ 27 ເດືອນມີນາ ປີ 2567"
    pub fn format_lao_full(&self) -> String {
        format!(
            "ວັນ{} ທີ {} ເດືອນ{} ປີ {}",
            self.weekday_lao(),
            to_lao_number(self.day as i32),
            self.month_lao(),
            to_lao_number(self.year_be())
        )
    }

    /// Format date in short Lao format
    /// Example: "27/3/2567"
    pub fn format_lao_short(&self) -> String {
        format!(
            "{}/{}/{}",
            to_lao_number(self.day as i32),
            to_lao_number(self.month as i32),
            to_lao_number(self.year_be())
        )
    }

    /// Format time in Lao numbers
    /// Example: "໑໔:໓໐:໐໐"
    pub fn format_lao_time(&self) -> String {
        format!(
            "{}:{}:{}",
            to_lao_number_padded(self.hour as i32, 2),
            to_lao_number_padded(self.minute as i32, 2),
            to_lao_number_padded(self.second as i32, 2)
        )
    }

    /// Format date and time in full Lao format
    pub fn format_lao_datetime(&self) -> String {
        format!("{} ເວລາ {}", self.format_lao_full(), self.format_lao_time())
    }

    /// Get the month name in Lao
    pub fn month_lao(&self) -> &'static str {
        match self.month {
            1 => "ມັງກອນ",
            2 => "ກຸມພາ",
            3 => "ມີນາ",
            4 => "ເມສາ",
            5 => "ພຶດສະພາ",
            6 => "ມິຖຸນາ",
            7 => "ກໍລະກົດ",
            8 => "ສິງຫາ",
            9 => "ກັນຍາ",
            10 => "ຕຸລາ",
            11 => "ພະຈິກ",
            12 => "ທັນວາ",
            _ => unreachable!(),
        }
    }

    /// Get the weekday name in Lao
    pub fn weekday_lao(&self) -> &'static str {
        let weekday = self.calculate_weekday();
        match weekday {
            0 => "ອາທິດ",
            1 => "ຈັນ",
            2 => "ອັງຄານ",
            3 => "ພຸດ",
            4 => "ພະຫັດ",
            5 => "ສຸກ",
            6 => "ເສົາ",
            _ => unreachable!(),
        }
    }

    /// Calculate day of week (0 = Sunday, 6 = Saturday)
    /// Using Zeller's congruence algorithm
    fn calculate_weekday(&self) -> u8 {
        let mut year = self.year;
        let mut month = self.month as i32;
        
        if month < 3 {
            month += 12;
            year -= 1;
        }
        
        let k = year % 100;
        let j = year / 100;
        
        let h = (self.day as i32 + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        
        // Convert to 0=Sunday format
        ((h + 6) % 7) as u8
    }
}

/// Convert a number to Lao numerals
pub fn to_lao_number(num: i32) -> String {
    let lao_digits = ['໐', '໑', '໒', '໓', '໔', '໕', '໖', '໗', '໘', '໙'];
    
    if num == 0 {
        return lao_digits[0].to_string();
    }
    
    let is_negative = num < 0;
    let mut n = num.abs();
    let mut result = String::new();
    
    while n > 0 {
        let digit = (n % 10) as usize;
        result.insert(0, lao_digits[digit]);
        n /= 10;
    }
    
    if is_negative {
        result.insert(0, '-');
    }
    
    result
}

/// Convert a number to Lao numerals with padding
pub fn to_lao_number_padded(num: i32, width: usize) -> String {
    let lao_zero = '໐';
    let result = to_lao_number(num.abs());
    let is_negative = num < 0;
    
    // Count actual characters (Lao digits are multi-byte)
    let result_len = result.chars().count();
    
    if result_len >= width {
        if is_negative {
            format!("-{}", result)
        } else {
            result
        }
    } else {
        let padding = lao_zero.to_string().repeat(width - result_len);
        if is_negative {
            format!("-{}{}", padding, result)
        } else {
            format!("{}{}", padding, result)
        }
    }
}

/// Parse a Lao number string to i32
pub fn from_lao_number(lao_num: &str) -> Result<i32, ParseError> {
    let lao_to_arabic: std::collections::HashMap<char, char> = [
        ('໐', '0'), ('໑', '1'), ('໒', '2'), ('໓', '3'), ('໔', '4'),
        ('໕', '5'), ('໖', '6'), ('໗', '7'), ('໘', '8'), ('໙', '9'),
    ].iter().cloned().collect();
    
    let mut arabic = String::new();
    let mut has_negative = false;
    
    for ch in lao_num.chars() {
        if ch == '-' {
            has_negative = true;
            continue;
        }
        
        if let Some(&digit) = lao_to_arabic.get(&ch) {
            arabic.push(digit);
        } else if ch.is_ascii_digit() {
            arabic.push(ch);
        } else if !ch.is_whitespace() {
            return Err(ParseError::InvalidCharacter(ch));
        }
    }
    
    let num = arabic.parse::<i32>()
        .map_err(|_| ParseError::InvalidNumber)?;
    
    Ok(if has_negative { -num } else { num })
}

/// Parse a Lao date string
pub fn parse_lao_date(date_str: &str) -> Result<LaoDateTime, ParseError> {
    // Support formats like "໒໗/໓/໒໕໖໗" or "27/3/2567"
    let parts: Vec<&str> = date_str.split('/').collect();
    
    if parts.len() != 3 {
        return Err(ParseError::InvalidFormat);
    }
    
    let day = from_lao_number(parts[0].trim())?;
    let month = from_lao_number(parts[1].trim())?;
    let year_be = from_lao_number(parts[2].trim())?;
    
    // Convert BE to Gregorian
    let year = year_be - 543;
    
    LaoDateTime::new(year, month as u8, day as u8, 0, 0, 0)
        .map_err(|e| ParseError::DateTimeError(e))
}

/// Error types for DateTime operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    InvalidMonth,
    InvalidDay,
    InvalidHour,
    InvalidMinute,
    InvalidSecond,
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateTimeError::InvalidMonth => write!(f, "Invalid month (must be 1-12)"),
            DateTimeError::InvalidDay => write!(f, "Invalid day (must be 1-31)"),
            DateTimeError::InvalidHour => write!(f, "Invalid hour (must be 0-23)"),
            DateTimeError::InvalidMinute => write!(f, "Invalid minute (must be 0-59)"),
            DateTimeError::InvalidSecond => write!(f, "Invalid second (must be 0-59)"),
        }
    }
}

impl std::error::Error for DateTimeError {}

/// Error types for parsing operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidCharacter(char),
    InvalidNumber,
    InvalidFormat,
    DateTimeError(DateTimeError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidCharacter(ch) => write!(f, "Invalid character: {}", ch),
            ParseError::InvalidNumber => write!(f, "Invalid number format"),
            ParseError::InvalidFormat => write!(f, "Invalid date format"),
            ParseError::DateTimeError(e) => write!(f, "DateTime error: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lao_number_conversion() {
        assert_eq!(to_lao_number(0), "໐");
        assert_eq!(to_lao_number(123), "໑໒໓");
        assert_eq!(to_lao_number(2567), "໒໕໖໗");
        assert_eq!(to_lao_number(-42), "-໔໒");
    }

    #[test]
    fn test_lao_number_padded() {
        assert_eq!(to_lao_number_padded(5, 2), "໐໕");
        assert_eq!(to_lao_number_padded(15, 2), "໑໕");
        assert_eq!(to_lao_number_padded(123, 2), "໑໒໓");
        assert_eq!(to_lao_number_padded(0, 2), "໐໐");
        assert_eq!(to_lao_number_padded(-5, 2), "-໐໕");
    }

    #[test]
    fn test_from_lao_number() {
        assert_eq!(from_lao_number("໐").unwrap(), 0);
        assert_eq!(from_lao_number("໑໒໓").unwrap(), 123);
        assert_eq!(from_lao_number("໒໕໖໗").unwrap(), 2567);
        assert_eq!(from_lao_number("-໔໒").unwrap(), -42);
    }

    #[test]
    fn test_mixed_arabic_lao_numbers() {
        assert_eq!(from_lao_number("123").unwrap(), 123);
        assert_eq!(from_lao_number("1໒3").unwrap(), 123);
    }

    #[test]
    fn test_datetime_creation() {
        let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.day(), 27);
        assert_eq!(dt.year_be(), 2567);
    }

    #[test]
    fn test_lao_formatting() {
        let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).unwrap();
        assert_eq!(dt.month_lao(), "ມີນາ");
        assert!(dt.format_lao_full().contains("ມີນາ"));
        assert!(dt.format_lao_full().contains("໒໕໖໗"));
    }

    #[test]
    fn test_lao_time_formatting() {
        let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 5).unwrap();
        assert_eq!(dt.format_lao_time(), "໑໔:໓໐:໐໕");
    }

    #[test]
    fn test_parse_lao_date() {
        let dt = parse_lao_date("໒໗/໓/໒໕໖໗").unwrap();
        assert_eq!(dt.day(), 27);
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.year(), 2024);
    }

    #[test]
    fn test_parse_mixed_date() {
        let dt = parse_lao_date("27/3/2567").unwrap();
        assert_eq!(dt.day(), 27);
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.year(), 2024);
    }

    #[test]
    fn test_weekday_calculation() {
        // March 27, 2024 is a Wednesday (ພຸດ)
        let dt = LaoDateTime::new(2024, 3, 27, 0, 0, 0).unwrap();
        assert_eq!(dt.weekday_lao(), "ພຸດ");
    }

    #[test]
    fn test_invalid_dates() {
        assert!(LaoDateTime::new(2024, 13, 1, 0, 0, 0).is_err());
        assert!(LaoDateTime::new(2024, 1, 32, 0, 0, 0).is_err());
        assert!(LaoDateTime::new(2024, 1, 1, 24, 0, 0).is_err());
    }
}
