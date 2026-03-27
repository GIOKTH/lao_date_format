use lao_format::{LaoDateTime, to_lao_number, from_lao_number, parse_lao_date};

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

    // Example 2: Buddhist Era conversion
    println!("--- Example 2: Buddhist Era Conversion ---");
    let new_year = LaoDateTime::new(2024, 1, 1, 0, 0, 0).unwrap();
    println!("Gregorian Year: {}", new_year.year());
    println!("Buddhist Era: {}", new_year.year_be());
    println!();

    // Example 3: Month and weekday names
    println!("--- Example 3: Month and Weekday Names ---");
    for month in 1..=12 {
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

    // Example 4: Number conversion
    println!("--- Example 4: Number Conversion ---");
    let numbers = [0, 123, 2567, -42];
    for num in numbers {
        let lao = to_lao_number(num);
        println!("Arabic: {} → Lao: {}", num, lao);
    }
    println!();

    // Example 5: Parse Lao numbers
    println!("--- Example 5: Parse Lao Numbers ---");
    let lao_numbers = ["໐", "໑໒໓", "໒໕໖໗", "-໔໒"];
    for lao_num in lao_numbers {
        match from_lao_number(lao_num) {
            Ok(num) => println!("Lao: {} → Arabic: {}", lao_num, num),
            Err(e) => println!("Error parsing '{}': {}", lao_num, e),
        }
    }
    println!();

    // Example 6: Parse dates
    println!("--- Example 6: Parse Lao Date Strings ---");
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

    // Example 7: Current date simulation
    println!("--- Example 7: Various Date Formats ---");
    let christmas = LaoDateTime::new(2024, 12, 25, 18, 0, 0).unwrap();
    println!("Christmas 2024:");
    println!("  Full: {}", christmas.format_lao_full());
    println!("  Short: {}", christmas.format_lao_short());
    println!("  With time: {}", christmas.format_lao_datetime());
    println!("  Weekday: {}", christmas.weekday_lao());
    println!();

    // Example 8: Error handling
    println!("--- Example 8: Error Handling ---");
    match LaoDateTime::new(2024, 13, 1, 0, 0, 0) {
        Ok(_) => println!("Should not reach here"),
        Err(e) => println!("Expected error for invalid month: {}", e),
    }
    
    match parse_lao_date("invalid/date/format") {
        Ok(_) => println!("Should not reach here"),
        Err(e) => println!("Expected error for invalid date: {}", e),
    }
}
