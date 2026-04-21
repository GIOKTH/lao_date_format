//! # Lao DateTime Library
//!
//! A Rust library for formatting and parsing dates in Lao language with Buddhist Era calendar support.
//!
//! ## Features
//! - Format dates in Lao language (ວັນ, ເດືອນ, ປີ)
//! - Buddhist Era (BE) calendar conversion
//! - Lao number formatting (໐໑໒໓໔໕໖໗໘໙)
//! - Number to Lao text conversion (ໜຶ່ງຮ້ອຍຊາວສາມ)
//! - Parse Lao date strings back to DateTime
//!
//! ## Example
//! ```
//! use Lao_date_format::LaoDateTime;
//!
//! let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0);
//! println!("{}", dt.unwrap().format_lao_full()); // ວັນພຸດ ທີ ໒໗ ເດືອນມີນາ ປີ ໒໕໖໗
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
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).unwrap();
    /// ```
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
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_lao_full(), "ວັນພຸດ ທີ ໒໗ ເດືອນມີນາ ປີ ໒໕໖໗");
    /// ```
    pub fn format_lao_full(&self) -> String {
        format!(
            "ວັນ{} ທີ {} ເດືອນ{} ປີ {}",
            self.weekday_lao(),
            to_lao_number(self.day as i32),
            self.month_lao(),
            to_lao_number(self.year_be())
        )
    }

    /// Format date in short Lao format (Buddhist Era)
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_lao_short(), "໒໗/໓/໒໕໖໗");
    /// ```
    pub fn format_lao_short(&self) -> String {
        format!(
            "{}/{}/{}",
            to_lao_number(self.day as i32),
            to_lao_number(self.month as i32),
            to_lao_number(self.year_be())
        )
    }

    /// Format date in dd/mm/yyyy format (Gregorian Era, Arabic numerals)
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_standard(), "27/03/2024");
    /// ```
    pub fn format_standard(&self) -> String {
        format!(
            "{:02}/{:02}/{}",
            self.day,
            self.month,
            self.year
        )
    }

    /// Format date in dd-mm-yyyy format (Gregorian Era, Arabic numerals)
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_standard_dash(), "27-03-2024");
    /// ```
    pub fn format_standard_dash(&self) -> String {
        format!(
            "{:02}-{:02}-{}",
            self.day,
            self.month,
            self.year
        )
    }

    /// Format date in dd-MM-yyyy format (Gregorian Era, Lao month name)
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2026, 1, 31, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_lao_month_dash(), "31-ມັງກອນ-2026");
    /// ```
    pub fn format_lao_month_dash(&self) -> String {
        format!(
            "{:02}-{}-{}",
            self.day,
            self.month_lao(),
            self.year
        )
    }

    /// Format date in dd/MM/yyyy format (Gregorian Era, Lao month name)
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2026, 1, 31, 0, 0, 0).unwrap();
    /// assert_eq!(dt.format_lao_month_slash(), "31/ມັງກອນ/2026");
    /// ```
    pub fn format_lao_month_slash(&self) -> String {
        format!(
            "{:02}/{}/{}",
            self.day,
            self.month_lao(),
            self.year
        )
    }

    /// Format time in Lao numbers
    ///
    /// # Example
    /// ```
    /// use Lao_date_format::LaoDateTime;
    /// let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 5).unwrap();
    /// assert_eq!(dt.format_lao_time(), "໑໔:໓໐:໐໕");
    /// ```
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
///
/// # Example
/// ```
/// use Lao_date_format::to_lao_number;
/// assert_eq!(to_lao_number(123), "໑໒໓");
/// ```
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
///
/// # Example
/// ```
/// use Lao_date_format::to_lao_number_padded;
/// assert_eq!(to_lao_number_padded(5, 2), "໐໕");
/// ```
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

/// Convert a number to Lao text representation
///
/// # Example
/// ```
/// use Lao_date_format::{number_to_lao_text, string_number_to_lao_text};
/// assert_eq!(number_to_lao_text(123), "ໜຶ່ງຮ້ອຍຊາວສາມ");
/// assert_eq!(number_to_lao_text(1000000), "ໜຶ່ງລ້ານ");
/// assert_eq!(string_number_to_lao_text("123"), "ໜຶ່ງຮ້ອຍຊາວສາມ");
/// assert_eq!(string_number_to_lao_text("1000000"), "ໜຶ່ງລ້ານ");
/// ```
pub fn number_to_lao_text(num: u64) -> String {
    if num == 0 {
        return "ສູນ".to_string();
    }

    // Helper function for reading numbers under a million
    fn read_under_million(num: u64, is_part_of_larger: bool) -> String {
        let units = ["", "ສິບ", "ຮ້ອຍ", "ພັນ", "ໝື່ນ", "ແສນ"];
        let digits = ["", "ໜຶ່ງ", "ສອງ", "ສາມ", "ສີ່", "ຫ້າ", "ຫົກ", "ເຈັດ", "ແປດ", "ເກົ້າ"];

        let s = num.to_string();
        let len = s.len();
        let mut result = String::new();

        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as usize;
            let pos = len - i - 1;

            if digit == 0 { continue; }

            match pos {
                1 => { // Tens
                    match digit {
                        1 => result.push_str("ສິບ"),
                        2 => result.push_str("ຊາວ"),
                        _ => {
                            result.push_str(digits[digit]);
                            result.push_str("ສິບ");
                        }
                    }
                }
                0 => { // Units
                    if digit == 1 && (i > 0 || is_part_of_larger) {
                        if len > 1 {
                            result.push_str("ເອັດ");
                        } else if is_part_of_larger {
                            result.push_str("ເອັດ");
                        } else {
                            result.push_str("ໜຶ່ງ");
                        }
                    } else {
                        result.push_str(digits[digit]);
                    }
                }
                _ => {
                    result.push_str(digits[digit]);
                    result.push_str(units[pos]);
                }
            }
        }
        result
    }

    let mut result = String::new();
    let mut n = num;

    // 1. Handle billions ("ຕື້")
    if n >= 1_000_000_000 {
        let billions = n / 1_000_000_000;
        n %= 1_000_000_000;
        result.push_str(&number_to_lao_text(billions));
        result.push_str("ຕື້");
    }

    // 2. Handle millions ("ລ້ານ")
    if n >= 1_000_000 {
        let millions = n / 1_000_000;
        n %= 1_000_000;
        // is_part_of_larger is false here because "1,000,000" starts with "ໜຶ່ງ"
        result.push_str(&read_under_million(millions, false));
        result.push_str("ລ້ານ");
    }

    // 3. Handle remainder
    if n > 0 {
        // Use "ເອັດ" if preceded by millions or billions
        result.push_str(&read_under_million(n, !result.is_empty()));
    }

    result
}

pub fn string_number_to_lao_text(num: &str) -> String {
    let num = num.trim();
    if num == "0" || num == "" {
        return "ສູນ".to_string();
    }

    let parts: Vec<&str> = num.split('.').collect();

    // Read integer part
    let mut result = number_to_lao_text(parts[0].parse::<u64>().unwrap_or(0));

    // Read decimal part (digit by digit)
    if parts.len() > 1 {
        let decimal = parts[1];
        if !decimal.is_empty() {
            result.push_str("ຈຸດ");
            for c in decimal.chars() {
                result.push_str(match c {
                    '0' => "ສູນ", '1' => "ໜຶ່ງ", '2' => "ສອງ", '3' => "ສາມ",
                    '4' => "ສີ່", '5' => "ຫ້າ", '6' => "ຫົກ", '7' => "ເຈັດ",
                    '8' => "ແປດ", '9' => "ເກົ້າ", _ => "",
                });
            }
        }
    }
    result
}

/// Parse a Lao number string to i32
///
/// # Example
/// ```
/// use Lao_date_format::from_lao_number;
/// assert_eq!(from_lao_number("໑໒໓").unwrap(), 123);
/// ```
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
///
/// # Example
/// ```
/// use Lao_date_format::parse_lao_date;
/// let dt = parse_lao_date("໒໗/໓/໒໕໖໗").unwrap();
/// assert_eq!(dt.day(), 27);
/// ```
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
    fn test_number_to_lao_text() {
        assert_eq!(number_to_lao_text(0), "ສູນ");
        assert_eq!(number_to_lao_text(1), "ໜຶ່ງ");
        assert_eq!(number_to_lao_text(10), "ສິບ");
        assert_eq!(number_to_lao_text(11), "ສິບເອັດ");
        assert_eq!(number_to_lao_text(20), "ຊາວ");
        assert_eq!(number_to_lao_text(21), "ຊາວເອັດ");
        assert_eq!(number_to_lao_text(100), "ໜຶ່ງຮ້ອຍ");
        assert_eq!(number_to_lao_text(101), "ໜຶ່ງຮ້ອຍເອັດ");
        assert_eq!(number_to_lao_text(123), "ໜຶ່ງຮ້ອຍຊາວສາມ");
        assert_eq!(number_to_lao_text(1000), "ໜຶ່ງພັນ");
        assert_eq!(number_to_lao_text(1000000), "ໜຶ່ງລ້ານ");
        assert_eq!(number_to_lao_text(1000001), "ໜຶ່ງລ້ານເອັດ");
        assert_eq!(number_to_lao_text(21000000), "ຊາວເອັດລ້ານ");
        assert_eq!(number_to_lao_text(1000000000), "ໜຶ່ງຕື້");
    }

    #[test]
    fn test_string_number_to_lao_text() {
        assert_eq!(string_number_to_lao_text("0"), "ສູນ");
        assert_eq!(string_number_to_lao_text("10"), "ສິບ");
        assert_eq!(string_number_to_lao_text("1000000"), "ໜຶ່ງລ້ານ");
        assert_eq!(string_number_to_lao_text("1000000.5"), "ໜຶ່ງລ້ານຈຸດຫ້າ");
    }

    #[test]
    fn test_lao_number_padded() {
        assert_eq!(to_lao_number_padded(5, 2), "໐໕");
        assert_eq!(to_lao_number_padded(0, 2), "໐໐");
    }

    #[test]
    fn test_from_lao_number() {
        assert_eq!(from_lao_number("໑໒໓").unwrap(), 123);
    }

    #[test]
    fn test_datetime_creation() {
        let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).unwrap();
        assert_eq!(dt.year_be(), 2567);
    }

    #[test]
    fn test_parse_lao_date() {
        let dt = parse_lao_date("໒໗/໓/໒໕໖໗").unwrap();
        assert_eq!(dt.day(), 27);
        assert_eq!(dt.month(), 3);
        assert_eq!(dt.year(), 2024);
    }
}
