# Lao DateTime Library

A Rust library for formatting and parsing dates in Lao language with Buddhist Era calendar support.

## Features
- Format dates in Lao language (ວັນ, ເດືອນ, ປີ)
- Buddhist Era (BE) calendar conversion
- Lao number formatting (໐໑໒໓໔໕໖໗໘໙)
- Parse Lao date strings back to DateTime

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
Lao_date_format = "0.1.0"
```

## Usage

### Creating a `LaoDateTime` instance

You can create a `LaoDateTime` instance using the `new` constructor:

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).expect("Invalid DateTime");
```

### Formatting Dates and Times

The library provides several methods for formatting dates and times in Lao.

#### Full Lao Date Format

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).expect("Invalid DateTime");
println!("{}", dt.format_lao_full());
// Output: ວັນພຸດ ທີ ໒໗ ເດືອນມີນາ ປີ ໒໕໖໗
```

#### Short Lao Date Format

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).expect("Invalid DateTime");
println!("{}", dt.format_lao_short());
// Output: ໒໗/໓/໒໕໖໗
```

#### Lao Time Format

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 5).expect("Invalid DateTime");
println!("{}", dt.format_lao_time());
// Output: ໑໔:໓໐:໐໕
```

#### Full Lao Date and Time Format

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 5).expect("Invalid DateTime");
println!("{}", dt.format_lao_datetime());
// Output: ວັນພຸດ ທີ ໒໗ ເດືອນມີນາ ປີ ໒໕໖໗ ເວລາ ໑໔:໓໐:໐໕
```

### Parsing Lao Dates

You can parse Lao date strings (in `DD/MM/YYYY_BE` format) back into `LaoDateTime` objects. Both Lao and Arabic numerals are supported.

```rust
use Lao_date_format::{parse_lao_date, LaoDateTime};

let date_str_lao = "໒໗/໓/໒໕໖໗";
let dt_lao = parse_lao_date(date_str_lao).expect("Failed to parse Lao date");
println!("Parsed Lao date: {:?}", dt_lao);
// Output: Parsed Lao date: LaoDateTime { year: 2024, month: 3, day: 27, hour: 0, minute: 0, second: 0 }

let date_str_arabic = "27/3/2567";
let dt_arabic = parse_lao_date(date_str_arabic).expect("Failed to parse Arabic date");
println!("Parsed Arabic date: {:?}", dt_arabic);
// Output: Parsed Arabic date: LaoDateTime { year: 2024, month: 3, day: 27, hour: 0, minute: 0, second: 0 }
```

### Utility Functions

The library also provides utility functions for converting between Arabic and Lao numerals:

```rust
use Lao_date_format::{to_lao_number, from_lao_number};

let arabic_num = 123;
let lao_num = to_lao_number(arabic_num);
println!("{} in Lao is {}", arabic_num, lao_num);
// Output: 123 in Lao is ໑໒໓

let parsed_arabic_num = from_lao_number(&lao_num).expect("Failed to parse Lao number");
println!("{} in Arabic is {}", lao_num, parsed_arabic_num);
// Output: ໑໒໓ in Arabic is 123
```
