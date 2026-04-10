# Lao DateTime Library

A Rust library for formatting and parsing dates in Lao language with Buddhist Era calendar support.

## Features
- Format dates in Lao language (ວັນ, ເດືອນ, ປີ)
- Buddhist Era (BE) calendar conversion
- Lao number formatting (໐໑໒໓໔໕໖໗໘໙)
- Parse Lao date strings back to DateTime
- Standard Gregorian date formatting (dd/mm/yyyy, dd-mm-yyyy, dd-MM-yyyy, dd/MM/yyyy)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
Lao_date_format = "0.1.2"
```

## Usage

### Creating a `LaoDateTime` instance

You can create a `LaoDateTime` instance using the `new` constructor:

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).expect("Invalid DateTime");
```

### Formatting Dates and Times

#### Standard Date Formats (Gregorian)

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2026, 1, 31, 14, 30, 0).expect("Invalid DateTime");

// Using slashes with numeric month
println!("{}", dt.format_standard());
// Output: 31/01/2026

// Using dashes with numeric month
println!("{}", dt.format_standard_dash());
// Output: 31-01-2026

// Using dashes with Lao month name
println!("{}", dt.format_lao_month_dash());
// Output: 31-ມັງກອນ-2026

// Using slashes with Lao month name
println!("{}", dt.format_lao_month_slash());
// Output: 31/ມັງກອນ/2026
```

#### Full Lao Date Format

```rust
use Lao_date_format::LaoDateTime;

let dt = LaoDateTime::new(2024, 3, 27, 14, 30, 0).expect("Invalid DateTime");
println!("{}", dt.format_lao_full());
// Output: ວັນພຸດ ທີ ໒໗ ເດືອນມີນາ ປີ ໒໕໖໗
```

#### Short Lao Date Format (Buddhist Era)

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

### Parsing Lao Dates

You can parse Lao date strings (in `DD/MM/YYYY_BE` format) back into `LaoDateTime` objects.

```rust
use Lao_date_format::{parse_lao_date, LaoDateTime};

let date_str_lao = "໒໗/໓/໒໕໖໗";
let dt_lao = parse_lao_date(date_str_lao).expect("Failed to parse Lao date");
// Output: LaoDateTime { year: 2024, month: 3, day: 27, ... }
```
