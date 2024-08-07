[![Crates.io](https://img.shields.io/crates/v/handlebars-chrono?color=4d76ae)](https://crates.io/crates/handlebars-chrono)
[![API](https://docs.rs/handlebars-chrono/badge.svg)](https://docs.rs/handlebars-chrono)
[![dependency status](https://deps.rs/repo/github/iganev/handlebars-chrono/status.svg)](https://deps.rs/repo/github/iganev/handlebars-chrono)
[![build and test](https://github.com/iganev/handlebars-chrono/actions/workflows/rust.yml/badge.svg)](https://github.com/iganev/handlebars-chrono/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/iganev/handlebars-chrono/graph/badge.svg?token=B5P2TAV5BB)](https://codecov.io/github/iganev/handlebars-chrono)


# handlebars-chrono
[chrono](https://github.com/chronotope/chrono) [DateTime](https://docs.rs/chrono/latest/chrono/struct.DateTime.html) helper for [handlebars-rust](https://github.com/sunng87/handlebars-rust)

## Quick Start

Developed and tested with handlebars-rust v5.1.0 and chrono v0.4.35.

Optional features:
- `locale` includes `unstable-locales` in `chrono` and enables the `locale` parameter to produce localized timestamps
- `timezone` includes `chrono-tz` and enables parsing IANA timezone names passed as `timezone` parameter

### Versions
Versions `0.1.*` are compatible with handlebars `5`.  
Versions `0.2.*` are compatible with handlebars `6`.  

### Include

Add to `Cargo.toml`:
```toml
handlebars-chrono = { version = "^0", features = [ "locale", "timezone" ] }
```

### Registration

```rust
    use handlebars::Handlebars;
    use handlebars_chrono::HandlebarsChronoDateTime;
    
    let mut h = Handlebars::new();
    h.register_helper("datetime", Box::new(HandlebarsChronoDateTime));
```

### Behavior

By default, if no parameters are supplied to the helper it produces a RFC3339 current UTC timestamp.

### Parameters

There are 3 groups of possible parameters in various different combinations:
- Initializers: Parameters controlling how the `chrono::DateTime` is being constructed. All initializers produce a `DateTime<Utc>` internally.
- Modifiers: Parameters modifying the internally constructed `DateTime<Utc>`. At this stage the internal `DateTime` gets converted to `DateTime<FixedOffset>` to support all possible modifiers and finalizers. By default, the `FixedOffset` remains tied to UTC.
- Finalizers: Parameters determining how the internal `DateTime` will get output as `String`

#### Initializers

The default initializer is equivalent to `Utc::now()` and produces a `DateTime<Utc>` of the current time.

Other possible initializers:
- `from_timestamp`: taking UNIX timestamp in seconds as value
- `from_timestamp_millis`: taking UNIX timestamp in milli-seconds as value
- `from_timestamp_micros`: taking UNIX timestamp in micro-seconds as value
- `from_timestamp_nanos`: taking UNIX timestamp in nano-seconds as value
- `from_rfc2822`: taking a valid RFC2822 formatted string as value
- `from_rfc3339`: taking a valid RFC3339 formatted string as value
- `from_str` + `input_format`: taking an arbitrarily formatted datetime string and its corresponding format as values

If the selected initializer holds a timezone or offset information it will be dropped and converted to the equivalent DateTime in UTC.

#### Modifiers

By default, no modifiers are being applied. Each modifier can be applied only once. Any variations of modifiers can be used together.

Possible modifiers:
- `with_timezone`: Sets the offset of the internal `DateTime`. Possible values are: `local` (for local time), valid fixed offset (ex. `-06:00`) or a valid IANA timezone, if the `timezone` feature is enabled (ex. `America/Edmonton`).
- `with_ordinal`: Sets the date to a specific day of the year, starting from 1.
- `with_ordinal0`: Sets the date to a specific day of the year, starting from 0.
- `with_year`: Sets the date to a specific year.
- `with_month`: Sets the date to a specific month, starting from 1.
- `with_month0`: Sets the date to a specific month, starting from 0.
- `with_day`: Sets the date to a specific day of the month, starting from 1.
- `with_day0`: Sets the date to a specific day of the month, starting from 0.
- `with_hour`: Sets the time to a specific hour of the day.
- `with_minute`: Sets the time to a specific minute of the hour.
- `with_second`: Sets the time to a specific second of the minute.
- `with_nanosecond`: Sets the nano-seconds segment of the `DateTime`.
- `add_months`: Adds a given number of months.
- `add_weeks`: Adds a given number of weeks.
- `add_days`: Adds a given number of days.
- `add_hours`: Adds a given number of hours.
- `add_minutes`: Adds a given number of minutes.
- `add_seconds`: Adds a given number of seconds.
- `add_milliseconds`: Adds a given number of milli-seconds.
- `add_microseconds`: Adds a given number of micro-seconds.
- `add_nanoseconds`: Adds a given number of nano-seconds.
- `sub_months`: Subtracts a given number of months.
- `sub_weeks`: Subtracts a given number of weeks.
- `sub_days`: Subtracts a given number of days.
- `sub_hours`: Subtracts a given number of hours.
- `sub_minutes`: Subtracts a given number of minutes.
- `sub_seconds`: Subtracts a given number of seconds.
- `sub_milliseconds`: Subtracts a given number of milli-seconds.
- `sub_microseconds`: Subtracts a given number of micro-seconds.
- `sub_nanoseconds`: Subtracts a given number of nano-seconds.

#### Finalizers

The default finalizer is equivalent to `DateTime::to_rfc3339()` and produces a valid RFC3339 `String`.

Other possible finalizers:
- `output_format` + `locale`: Takes a [strftime date time format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) to use as an output format. The `locale` parameter works only if the `locale` feature is enabled. In that case it accepts [a valid locale name](https://docs.rs/chrono/latest/chrono/enum.Locale.html).
- `to_rfc2822`: Makes the helper output a valid RFC2822 string. To be a valid hash parameter you must supply a value. For example `true`, or `1`. The value is being ignored.
- `to_timestamp`: Makes the helper output a UNIX timestamp in seconds. To be a valid hash parameter you must supply a value. For example `true`, or `1`. The value is being ignored.
- `to_timestamp_millis`: Makes the helper output a UNIX timestamp in milli-seconds. To be a valid hash parameter you must supply a value. For example `true`, or `1`. The value is being ignored.
- `to_timestamp_micros`: Makes the helper output a UNIX timestamp in micro-seconds. To be a valid hash parameter you must supply a value. For example `true`, or `1`. The value is being ignored.
- `to_timestamp_nanos`: Makes the helper output a UNIX timestamp in nano-seconds. To be a valid hash parameter you must supply a value. For example `true`, or `1`. The value is being ignored.
- `years_since`: Takes a RFC3339 formatted date time to compare against the internal `DateTime` and calculate the years difference. The `years_since` value must be the further in the past.

### Examples

Current date and time in RFC3339:
```handlebars
{{datetime}}
```

Current date and time in format like `2024-03-16 15:39:42`:
```handlebars
{{datetime output_format="%Y-%m-%d %H:%M:%S"}}
```

Calendar date in French, for example `samedi, mars 16`:    
```handlebars
{{datetime output_format="%A, %B %e" locale="fr_FR"}}
```

Current date and time in RFC2822, like `Sat, 16 Mar 2024 21:43:31 +0000`:
```handlebars
{{datetime to_rfc2822=true}}
```

Current UNIX timestamp:
```handlebars
{{datetime to_timestamp=true}}
```

Years since June 1985:
```handlebars
{{datetime years_since="1985-06-16T12:00:00Z"}}
```

Reformat a UNIX timestamp to MySQL datetime:
```handlebars
{{datetime from_timestamp="618658211" output_format="%Y-%m-%d %H:%M:%S"}}
```

Years between two dates in the past:
```handlebars
{{datetime from_timestamp="618658211" years_since="1985-06-16T12:00:00Z"}}
```

Years between two dates in the past, but with UNIX timestamps only:
```handlebars
{{datetime from_timestamp="618658211" years_since=(datetime from_timestamp="487771200")}}
```

From UNIX timestamp in milli-seconds to calendar date in French (ex. `mercredi, août  9`):
```handlebars
{{datetime from_timestamp_millis="618658211123" output_format="%A, %B %e" locale="fr_FR"}}
```

From RFC2822 date time to MySQL datetime:
```handlebars
{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" output_format="%Y-%m-%d %H:%M:%S"}}
```

From RFC3339 to RFC2822:
```handlebars
{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_rfc2822=true}}
```

From MySQL datetime to RFC2822:
```handlebars
{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_rfc2822=true}}
```

From/To RFC3339 with timezone change:
```handlebars
{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="America/Edmonton"}}
```

Switch from August to November:
```handlebars
{{datetime from_timestamp="618658211" with_month="11" to_timestamp=true}}
```

Switch from the 9th to the 11th:
```handlebars
{{datetime from_timestamp="618658211" with_day="11" to_timestamp=true}}
```

Add 24 months:
```handlebars
{{datetime from_timestamp="618658211" add_months="24" to_timestamp=true}}
```

Subtract 4 weeks:
```handlebars
{{datetime from_timestamp="618658211" sub_weeks="4" to_timestamp=true}}
```

## License

This library (handlebars-chrono) is open sourced under the BSD 2 License.
