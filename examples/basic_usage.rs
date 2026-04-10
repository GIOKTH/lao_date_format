use Lao_date_format::{LaoDateTime, to_lao_number, from_lao_number, parse_lao_date, number_to_lao_text, string_number_to_lao_text};

fn main() {
    println!("=== Lao DateTime Library Examples ===\n");

    // Example 1: Create and format dates
    println!("--- Example 1: Basic Formatting ---");
    let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 15).unwrap();
    println!("Full format: {}", dt.format_lao_full());
    println!("Short format: {}", dt.format_lao_short());
    println!("Time format: {}", dt.format_lao_time());
    println!("Full datetime: {}", dt.format_lao_datetime());
    println!();

    // Example 2: New Gregorian Formats
    println!("--- Example 2: New Gregorian Formats (Standard Arabic Numerals) ---");
    let future_dt = LaoDateTime::new(2026, 12, 31, 0, 0, 0).unwrap();
    println!("Standard (slash): {}", future_dt.format_standard());
    println!("Standard (dash):  {}", future_dt.format_standard_dash());
    println!();

    // Example 3: Mixed Lao Month Formats
    println!("--- Example 3: Mixed Lao Month Formats (Gregorian Year) ---");
    let jan_dt = LaoDateTime::new(2026, 1, 31, 0, 0, 0).unwrap();
    println!("Lao Month (dash):  {}", jan_dt.format_lao_month_dash());
    println!("Lao Month (slash): {}", jan_dt.format_lao_month_slash());
    println!();

    // Example 4: Number to Lao Text
    println!("--- Example 4: Number to Lao Text ---");
    let amounts = [123, 211, 2_003, 2_071, 2_001,2_101, 2_500, 1_000_000, 1_234_567_890, 2_100_000_001];
    let amounts_str = ["123", "211.2", "2003.10", "2071.04", "2001.00","2101.50", "2500.03", "1000000.07", "1234567890.05", "2100000001.01"];
    for &amt in &amounts {
        println!("Arabic: {:12} → Lao Text: {}", amt, number_to_lao_text(amt));
    }
    println!("---------------- from string number to text ------------------");
    for &amt in &amounts_str {
        println!("Arabic: {:12} → Lao Text: {}", amt, string_number_to_lao_text(amt));
    }
    println!();

    // Example 5: Buddhist Era conversion
    println!("--- Example 5: Buddhist Era Conversion ---");
    let new_year = LaoDateTime::new(2024, 1, 1, 0, 0, 0).unwrap();
    println!("Gregorian Year: {}", new_year.year());
    println!("Buddhist Era: {}", new_year.year_be());
    println!();

    // Example 6: Month and weekday names
    println!("--- Example 6: Month and Weekday Names ---");
    for month in 1..=3 {
        let dt = LaoDateTime::new(2024, month, 1, 0, 0, 0).unwrap();
        println!("Month {}: {}", month, dt.month_lao());
    }
    println!();

    println!("Weekdays for first week of January 2024:");
    for day in 1..=7 {
        let dt = LaoDateTime::new(2024, 1, day, 0, 0, 0).unwrap();
        println!("Jan {}: {}", day, dt.weekday_lao());
    }
    println!();

    // Example 7: Number conversion (Numerals)
    println!("--- Example 7: Number Conversion (Numerals) ---");
    let numbers = [0, 123, 2567, -42];
    for num in numbers {
        let lao = to_lao_number(num);
        println!("Arabic: {} → Lao: {}", num, lao);
    }
    println!();

    // Example 8: Parse Lao numbers
    println!("--- Example 8: Parse Lao Numbers ---");
    let lao_numbers = ["໐", "໑໒໓", "໒໕໖໗", "-໔໒"];
    for lao_num in lao_numbers {
        match from_lao_number(lao_num) {
            Ok(num) => println!("Lao: {} → Arabic: {}", lao_num, num),
            Err(e) => println!("Error parsing '{}': {}", lao_num, e),
        }
    }
    println!();

    // Example 9: Parse dates
    println!("--- Example 9: Parse Lao Date Strings ---");
    let date_strings = ["໒໗/໓/໒໕໖໗", "27/3/2567", "1/1/2567"];
    for date_str in date_strings {
        match parse_lao_date(date_str) {
            Ok(dt) => {
                println!("Parsed '{}': {}", date_str, dt.format_lao_full());
            }
            Err(e) => println!("Error parsing '{}': {}", date_str, e),
        }
    }
    println!();

    // Example 10: Current date simulation
    println!("--- Example 10: Various Date Formats ---");
    let christmas = LaoDateTime::new(2024, 12, 25, 18, 0, 0).unwrap();
    println!("Christmas 2024:");
    println!("  Full Lao:      {}", christmas.format_lao_full());
    println!("  Standard Dash: {}", christmas.format_standard_dash());
    println!("  Lao Month:     {}", christmas.format_lao_month_dash());
    println!("  Weekday:       {}", christmas.weekday_lao());
    println!();

    // Example 11: Error handling
    println!("--- Example 11: Error Handling ---");
    match LaoDateTime::new(2024, 13, 1, 0, 0, 0) {
        Ok(_) => println!("Should not reach here"),
        Err(e) => println!("Expected error for invalid month: {}", e),
    }
    
    match parse_lao_date("invalid/date/format") {
        Ok(_) => println!("Should not reach here"),
        Err(e) => println!("Expected error for invalid date: {}", e),
    }
}
