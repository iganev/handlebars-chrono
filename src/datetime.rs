#[cfg(feature = "locale")]
use chrono::Locale;
use chrono::{DateTime, Datelike, Days, FixedOffset, Local, Months, NaiveDateTime, TimeDelta, Timelike, Utc};
#[cfg(feature = "timezone")]
use chrono_tz::Tz;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError, RenderErrorReason};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
/// Chrono DateTime helper for Handlebars
///
/// # Registration
///
/// ```rust
/// use chrono::Utc;
/// use handlebars::Handlebars;
/// use handlebars_chrono::HandlebarsChronoDateTime;
/// use serde_json::json;
///
/// let mut h = Handlebars::new();
/// h.register_helper("datetime", Box::new(HandlebarsChronoDateTime));
///
/// assert_eq!(h.render_template(r#"{{datetime}}"#, &json!({})).map(|s| s.as_str()[..16].to_string()).expect("Render error"), Utc::now().to_rfc3339().as_str()[..16].to_string());
/// ```
///
/// # Behavior
///
/// TODO
///
/// # Hash parameters
///
/// TODO
///
/// # Example usage:
///
///
///
pub struct HandlebarsChronoDateTime;

impl HelperDef for HandlebarsChronoDateTime {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _r: &'reg Handlebars,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        // INITIALIZERS
        //
        // default Utc::now()
        // from_timestamp (secs, 0)
        // from_timestamp_millis (millis)
        // from_timestamp_micros (micros)
        // from_timestamp_nanos (nanos)
        // parse_from_rfc2822
        // parse_from_rfc3339
        // parse_from_str + input_format
        let datetime = if let Some(timestamp) = h.hash_get("from_timestamp") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp(
                timestamp.parse().map_err(|e: ParseIntError| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid seconds timestamp: {}", e)))
                })?,
                0,
            )
            .ok_or::<RenderError>(RenderErrorReason::Other("Out-of-range number of seconds".to_string()).into())?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_millis") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_millis(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid milli-seconds timestamp: {}", e)))
            })?)
            .ok_or::<RenderError>(RenderErrorReason::Other("Out-of-range number of milliseconds".to_string()).into())?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_micros") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_micros(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid micro-seconds timestamp: {}", e)))
            })?)
            .ok_or::<RenderError>(
                RenderErrorReason::Other(
                    "Number of microseconds would be out of range for a NaiveDateTime (more than ca. 262,000 years away from common era)"
                        .to_string(),
                )
                .into(),
            )?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_nanos") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_nanos(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid nano-seconds timestamp: {}", e)))
            })?)
        } else if let Some(input_str) = h.hash_get("from_rfc2822") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc2822(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                        "Invalid RFC2822 datetime format: {}",
                        e
                    )))
                })?
                .to_utc()
        } else if let Some(input_str) = h.hash_get("from_rfc3339") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc3339(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                        "Invalid RFC3339 datetime format: {}",
                        e
                    )))
                })?
                .to_utc()
        } else if let Some(input_str) = h.hash_get("from_str") {
            if let Some(input_format) = h.hash_get("input_format") {
                let input_str = input_str.render();
                let input_format = input_format.render();

                NaiveDateTime::parse_from_str(&input_str, &input_format)
                    .map_err(|e| {
                        <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                            "Invalid datetime format or format doesn't match input: {}",
                            e
                        )))
                    })?
                    .and_utc()
            } else {
                // error, missing input format
                return Err(RenderErrorReason::Other("Missing `input_format` hash parameter".to_string()).into());
            }
        } else {
            Utc::now()
        };

        // MODIFIERS (by default everything is converted to UTC by the initializer)
        //
        // with_timezone
        // with_ordinal
        // with_ordinal0
        // with_year
        // with_month
        // with_month0
        // with_day
        // with_day0
        // with_hour
        // with_minute
        // with_second
        // with_nanosecond
        // add_months
        // add_weeks
        // add_days
        // add_hours
        // add_minutes
        // add_seconds
        // add_milliseconds
        // add_microseconds
        // add_nanoseconds
        // sub_months
        // sub_weeks
        // sub_days
        // sub_hours
        // sub_minutes
        // sub_seconds
        // sub_milliseconds
        // sub_microseconds
        // sub_nanoseconds
        let datetime = if let Some(timezone) = h.hash_get("with_timezone") {
            let timezone = timezone.render();
            let tz: FixedOffset = if timezone.to_lowercase() == "local" {
                Local::now().fixed_offset().timezone()
            } else if timezone.contains('0') {
                if let Ok(tz) = FixedOffset::from_str(&timezone) {
                    tz
                } else {
                    return Err(RenderErrorReason::Other(
                        "Failed to parse timezone offset. Supported values are IANA timezones, local or valid fixed offset".to_string(),
                    )
                    .into());
                }
            } else {
                #[cfg(feature = "timezone")]
                if let Ok(tz) = timezone.parse::<Tz>() {
                    datetime.with_timezone(&tz).fixed_offset().timezone()
                } else {
                    return Err(RenderErrorReason::Other(
                        "Failed to parse IANA timezone. Supported values are IANA timezones, local or valid fixed offset".to_string(),
                    )
                    .into());
                }

                #[cfg(not(feature = "timezone"))]
                return Err(RenderErrorReason::Other(
                    "You need to enable the `timezone` feature of the `handlebars-chrono` create for IANA timezones to work.".to_string(),
                )
                .into());
            };

            datetime.with_timezone(&tz)
        } else {
            datetime.fixed_offset()
        };

        let datetime = if let Some(day) = h.hash_get("with_ordinal") {
            let day = day.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid ordinal parameter: {}", e)))
            })?;

            datetime
                .with_ordinal(day)
                .ok_or::<RenderError>(RenderErrorReason::Other("Ordinal parameter out of range".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(day) = h.hash_get("with_ordinal0") {
            let day = day.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid ordinal parameter: {}", e)))
            })?;

            datetime
                .with_ordinal0(day)
                .ok_or::<RenderError>(RenderErrorReason::Other("Ordinal parameter out of range".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(year) = h.hash_get("with_year") {
            let year = year.render().parse::<i32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid year parameter: {}", e)))
            })?;

            datetime
                .with_year(year)
                .ok_or::<RenderError>(RenderErrorReason::Other("Year parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(month) = h.hash_get("with_month") {
            let month = month.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid month parameter: {}", e)))
            })?;

            datetime.with_month(month).ok_or::<RenderError>(
                RenderErrorReason::Other("Month parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(month) = h.hash_get("with_month0") {
            let month = month.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid month parameter: {}", e)))
            })?;

            datetime.with_month0(month).ok_or::<RenderError>(
                RenderErrorReason::Other("Month parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(day) = h.hash_get("with_day") {
            let day = day.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid day parameter: {}", e)))
            })?;

            datetime
                .with_day(day)
                .ok_or::<RenderError>(RenderErrorReason::Other("Day parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(day) = h.hash_get("with_day0") {
            let day = day.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid day parameter: {}", e)))
            })?;

            datetime
                .with_day0(day)
                .ok_or::<RenderError>(RenderErrorReason::Other("Day parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(hour) = h.hash_get("with_hour") {
            let hour = hour.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid hour parameter: {}", e)))
            })?;

            datetime
                .with_hour(hour)
                .ok_or::<RenderError>(RenderErrorReason::Other("Hour parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(min) = h.hash_get("with_minute") {
            let min = min.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid minute parameter: {}", e)))
            })?;

            datetime.with_minute(min).ok_or::<RenderError>(
                RenderErrorReason::Other("Minute parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(sec) = h.hash_get("with_second") {
            let sec = sec.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid second parameter: {}", e)))
            })?;

            datetime.with_second(sec).ok_or::<RenderError>(
                RenderErrorReason::Other("Second parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(nsec) = h.hash_get("with_nanosecond") {
            let nsec = nsec.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid nano-second parameter: {}", e)))
            })?;

            datetime.with_nanosecond(nsec).ok_or::<RenderError>(
                RenderErrorReason::Other("Nano-second parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        // add_

        let datetime = if let Some(months) = h.hash_get("add_months") {
            let months = months.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid months parameter: {}", e)))
            })?;

            datetime.checked_add_months(Months::new(months)).ok_or::<RenderError>(
                RenderErrorReason::Other("Months parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(weeks) = h.hash_get("add_weeks") {
            let weeks = weeks.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid weeks parameter: {}", e)))
            })?;

            datetime
                .checked_add_signed(
                    TimeDelta::try_weeks(weeks)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Weeks parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Weeks parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(days) = h.hash_get("add_days") {
            let days = days.render().parse::<u64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid days parameter: {}", e)))
            })?;

            datetime
                .checked_add_days(Days::new(days))
                .ok_or::<RenderError>(RenderErrorReason::Other("Days parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(hours) = h.hash_get("add_hours") {
            let hours = hours.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid hours parameter: {}", e)))
            })?;

            datetime
                .checked_add_signed(
                    TimeDelta::try_hours(hours)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Hours parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Hours parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(min) = h.hash_get("add_minutes") {
            let min = min.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid minutes parameter: {}", e)))
            })?;

            datetime
                .checked_add_signed(
                    TimeDelta::try_minutes(min)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Minutes parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Minutes parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(sec) = h.hash_get("add_seconds") {
            let sec = sec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid seconds parameter: {}", e)))
            })?;

            datetime
                .checked_add_signed(
                    TimeDelta::try_seconds(sec)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Seconds parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Seconds parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(msec) = h.hash_get("add_milliseconds") {
            let msec = msec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid milli-seconds parameter: {}", e)))
            })?;

            datetime
                .checked_add_signed(
                    TimeDelta::try_milliseconds(msec)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Milli-seconds parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Milli-seconds parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(usec) = h.hash_get("add_microseconds") {
            let usec = usec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid micro-seconds parameter: {}", e)))
            })?;

            datetime.checked_add_signed(TimeDelta::microseconds(usec)).ok_or::<RenderError>(
                RenderErrorReason::Other("Micro-seconds parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(nsec) = h.hash_get("add_nanoseconds") {
            let nsec = nsec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid nano-seconds parameter: {}", e)))
            })?;

            datetime.checked_add_signed(TimeDelta::nanoseconds(nsec)).ok_or::<RenderError>(
                RenderErrorReason::Other("Nano-seconds parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        // sub_

        let datetime = if let Some(months) = h.hash_get("sub_months") {
            let months = months.render().parse::<u32>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid months parameter: {}", e)))
            })?;

            datetime.checked_sub_months(Months::new(months)).ok_or::<RenderError>(
                RenderErrorReason::Other("Months parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(weeks) = h.hash_get("sub_weeks") {
            let weeks = weeks.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid weeks parameter: {}", e)))
            })?;

            datetime
                .checked_sub_signed(
                    TimeDelta::try_weeks(weeks)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Weeks parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Weeks parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(days) = h.hash_get("sub_days") {
            let days = days.render().parse::<u64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid days parameter: {}", e)))
            })?;

            datetime
                .checked_sub_days(Days::new(days))
                .ok_or::<RenderError>(RenderErrorReason::Other("Days parameter out of range or produces invalid date".to_string()).into())?
        } else {
            datetime
        };

        let datetime = if let Some(hours) = h.hash_get("sub_hours") {
            let hours = hours.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid hours parameter: {}", e)))
            })?;

            datetime
                .checked_sub_signed(
                    TimeDelta::try_hours(hours)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Hours parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Hours parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(min) = h.hash_get("sub_minutes") {
            let min = min.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid minutes parameter: {}", e)))
            })?;

            datetime
                .checked_sub_signed(
                    TimeDelta::try_minutes(min)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Minutes parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Minutes parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(sec) = h.hash_get("sub_seconds") {
            let sec = sec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid seconds parameter: {}", e)))
            })?;

            datetime
                .checked_sub_signed(
                    TimeDelta::try_seconds(sec)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Seconds parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Seconds parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(msec) = h.hash_get("sub_milliseconds") {
            let msec = msec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid milli-seconds parameter: {}", e)))
            })?;

            datetime
                .checked_sub_signed(
                    TimeDelta::try_milliseconds(msec)
                        .ok_or::<RenderError>(RenderErrorReason::Other("Milli-seconds parameter out of range".to_string()).into())?,
                )
                .ok_or::<RenderError>(
                    RenderErrorReason::Other("Milli-seconds parameter out of range or produces invalid date".to_string()).into(),
                )?
        } else {
            datetime
        };

        let datetime = if let Some(usec) = h.hash_get("sub_microseconds") {
            let usec = usec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid micro-seconds parameter: {}", e)))
            })?;

            datetime.checked_sub_signed(TimeDelta::microseconds(usec)).ok_or::<RenderError>(
                RenderErrorReason::Other("Micro-seconds parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        let datetime = if let Some(nsec) = h.hash_get("sub_nanoseconds") {
            let nsec = nsec.render().parse::<i64>().map_err(|e| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!("Invalid nano-seconds parameter: {}", e)))
            })?;

            datetime.checked_sub_signed(TimeDelta::nanoseconds(nsec)).ok_or::<RenderError>(
                RenderErrorReason::Other("Nano-seconds parameter out of range or produces invalid date".to_string()).into(),
            )?
        } else {
            datetime
        };

        // FINALIZERS

        // format - output_format
        // format_localized - output_format + locale
        // to_rfc3339 (default)
        // to_rfc2822
        // timestamp
        // timestamp_millis
        // timestamp_micros
        // timestamp_nanos
        // years_since + (parse_from_rfc3339)
        let output = if let Some(output_format) = h.hash_get("output_format") {
            let output_format = output_format.render();

            if let Some(locale) = h.hash_get("locale") {
                let locale = locale.render();
                #[cfg(feature = "locale")]
                {
                    let locale = Locale::from_str(&locale).map_err(|_e| {
                        <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                            "Invalid locale provided: {}",
                            &locale
                        )))
                    })?;

                    format!("{}", datetime.format_localized(&output_format, locale))
                }
                #[cfg(not(feature = "locale"))]
                return Err(RenderErrorReason::Other(format!(
                    "You need to enable the `locale` feature of `handlebars-chrono` for the `locale`={} param to work.",
                    locale
                ))
                .into());
            } else {
                format!("{}", datetime.format(&output_format))
            }
        } else if h.hash_get("to_rfc2822").is_some() {
            datetime.to_rfc2822()
        } else if h.hash_get("to_timestamp").is_some() {
            datetime.timestamp().to_string()
        } else if h.hash_get("to_timestamp_millis").is_some() {
            datetime.timestamp_millis().to_string()
        } else if h.hash_get("to_timestamp_micros").is_some() {
            datetime.timestamp_micros().to_string()
        } else if h.hash_get("to_timestamp_nanos").is_some() {
            datetime
                .timestamp_nanos_opt()
                .ok_or::<RenderError>(
                    RenderErrorReason::Other(
                        "An i64 with nanosecond precision can span a range of ~584 years. This timestamp is out of range.".to_string(),
                    )
                    .into(),
                )?
                .to_string()
        } else if let Some(input_rfc3339) = h.hash_get("years_since") {
            let input_rfc3339 = input_rfc3339.render();

            let base_datetime = DateTime::parse_from_rfc3339(&input_rfc3339)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                        "Invalid RFC3339 datetime format: {}",
                        e
                    )))
                })?
                .to_utc();

            datetime
                .years_since(base_datetime.into())
                .ok_or::<RenderError>(RenderErrorReason::Other("Negative range, try swapping the parameters.".to_string()).into())?
                .to_string()
        } else {
            // DEFAULT to_rfc3339

            datetime.to_rfc3339()
        };

        out.write(&output)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime};

    #[test]
    fn it_works() {
        use handlebars::Handlebars;

        let mut h = Handlebars::new();
        h.register_helper("datetime", Box::new(HandlebarsChronoDateTime));

        // default: Utc::now() -> to_rfc3339
        assert_eq!(
            h.render_template(r#"{{datetime}}"#, &String::new())
                .map(|s| s.as_str()[..16].to_string())
                .expect("Render error"),
            Utc::now().to_rfc3339().as_str()[..16].to_string(),
            "Failed to render RFC3339 default output"
        );

        // default to output_format: Utc::now() -> format
        let comparison = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            h.render_template(r#"{{datetime output_format="%Y-%m-%d %H:%M:%S"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S"
        );

        // default to output_format + locale: Utc::now() -> format_localized
        #[cfg(feature = "locale")]
        let comparison = Utc::now().format_localized("%A, %B %e", Locale::fr_FR).to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(r#"{{datetime output_format="%A, %B %e" locale="fr_FR"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C"
        );

        // default to to_rfc2822: Utc::now() -> to_rfc2822
        let comparison = Utc::now().to_rfc2822();
        assert_eq!(
            h.render_template(r#"{{datetime to_rfc2822=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC2822"
        );

        // default to to_timestamp: Utc::now() -> timestamp
        let comparison = Utc::now().timestamp().to_string();
        assert_eq!(
            h.render_template(r#"{{datetime to_timestamp=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp"
        );

        // default to to_timestamp_millis: Utc::now() -> timestamp_millis
        let comparison = Utc::now().timestamp_millis().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime to_timestamp_millis=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds"
        );

        // default to to_timestamp_micros: Utc::now() -> timestamp_micros
        let comparison = Utc::now().timestamp_micros().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime to_timestamp_micros=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds"
        );

        // default to to_timestamp_nanos: Utc::now() -> timestamp_nanos
        let comparison = Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds"
        );

        // default to years_since + (parse_from_rfc3339):
        let comparison = Utc::now()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(r#"{{datetime years_since="1985-06-16T12:00:00Z"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render years since"
        );

        //

        // from_timestamp to default: from_timestamp -> to_rfc3339
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from timestamp"
        );

        // from_timestamp to output_format: from_timestamp -> format
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp"
        );

        // from_timestamp to output_format + locale: from_timestamp -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp"
        );

        // from_timestamp to to_rfc2822: from_timestamp -> to_rfc2822
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().to_rfc2822();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_rfc2822=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp"
        );

        // from_timestamp to to_timestamp: from_timestamp -> timestamp
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().timestamp().to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_timestamp=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp"
        );

        //

        // default to to_timestamp_millis: from_timestamp -> timestamp_millis
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp"
        );

        // from_timestamp to to_timestamp_micros: from_timestamp -> timestamp_micros
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp"
        );

        // from_timestamp to to_timestamp_nanos: from_timestamp -> timestamp_nanos
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp"
        );

        // from_timestamp to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp"
        );
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" years_since=(datetime from_timestamp="487771200")}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp"
        );

        //

        // from_timestamp_millis to default: from_timestamp_millis -> to_rfc3339
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from timestamp in milli-seconds"
        );

        // from_timestamp_millis to output_format: from_timestamp_millis -> format
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in milli-seconds"
        );

        // from_timestamp_millis to output_format + locale: from_timestamp_millis -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .format_localized("%A, %B %e", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" output_format="%A, %B %e" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_rfc2822: from_timestamp_millis -> to_rfc2822
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp: from_timestamp_millis -> timestamp
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().timestamp().to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_millis: from_timestamp_millis -> timestamp_millis
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_micros: from_timestamp_millis -> timestamp_micros
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_nanos: from_timestamp_millis -> timestamp_nanos
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_millis(618658211123)
            .unwrap()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_millis="618658211123" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp in milli-seconds"
        );

        //

        // from_timestamp_micros to default: from_timestamp_micros -> to_rfc3339
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from timestamp in micro-seconds"
        );

        // from_timestamp_micros to output_format: from_timestamp_micros -> format
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in micro-seconds"
        );

        // from_timestamp_micros to output_format + locale: from_timestamp_micros -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_rfc2822: from_timestamp_micros -> to_rfc2822
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp: from_timestamp_micros -> timestamp
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().timestamp().to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_millis: from_timestamp_micros -> timestamp_millis
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_micros: from_timestamp_micros -> timestamp_micros
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_nanos: from_timestamp_micros -> timestamp_nanos
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_micros(618658211123456)
            .unwrap()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_micros="618658211123456" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp in micro-seconds"
        );

        //

        // from_timestamp_nanos to default: from_timestamp_nanos -> to_rfc3339
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to output_format: from_timestamp_nanos -> format
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to output_format + locale: from_timestamp_nanos -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_rfc2822: from_timestamp_nanos -> to_rfc2822
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp: from_timestamp_nanos -> timestamp
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).timestamp().to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_millis: from_timestamp_nanos -> timestamp_millis
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_micros: from_timestamp_nanos -> timestamp_micros
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_nanos: from_timestamp_nanos -> timestamp_nanos
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_nanos(618658211123456789)
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp_nanos="618658211123456789" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp in nano-seconds"
        );

        //

        // from_rfc2822 to default: parse_from_rfc2822 -> to_rfc3339
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from RFC2822"
        );

        // from_rfc2822 to output_format: parse_from_rfc2822 -> format
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from RFC2822"
        );

        // from_rfc2822 to output_format + locale: parse_from_rfc2822 -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from RFC2822"
        );

        // from_rfc2822 to to_rfc2822: parse_from_rfc2822 -> to_rfc2822
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from RFC2822"
        );

        // from_rfc2822 to to_timestamp: parse_from_rfc2822 -> timestamp
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from RFC2822"
        );

        // from_rfc2822 to to_timestamp_millis: parse_from_rfc2822 -> timestamp_millis
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from RFC2822"
        );

        // from_rfc2822 to to_timestamp_micros: parse_from_rfc2822 -> timestamp_micros
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from RFC2822"
        );

        // from_rfc2822 to to_timestamp_nanos: parse_from_rfc2822 -> timestamp_nanos
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from RFC2822"
        );

        // from_rfc2822 to years_since + (parse_from_rfc3339)
        let comparison = DateTime::parse_from_rfc2822("Wed, 09 Aug 1989 09:30:11 +0200")
            .unwrap()
            .to_utc()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc2822="Wed, 09 Aug 1989 09:30:11 +0200" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from RFC2822"
        );

        //

        // from_rfc3339 to default: parse_from_rfc3339 -> to_rfc3339
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .to_rfc3339();
        assert_eq!(
            h.render_template(r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from RFC3339"
        );

        // from_rfc3339 to output_format: parse_from_rfc3339 -> format
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" output_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from RFC3339"
        );

        // from_rfc3339 to output_format + locale: parse_from_rfc3339 -> format_localized
        #[cfg(feature = "locale")]
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from RFC3339"
        );

        // from_rfc3339 to to_rfc2822: parse_from_rfc3339 -> to_rfc2822
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from RFC3339"
        );

        // from_rfc3339 to to_timestamp: parse_from_rfc3339 -> timestamp
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from RFC3339"
        );

        // from_rfc3339 to to_timestamp_millis: parse_from_rfc3339 -> timestamp_millis
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from RFC3339"
        );

        // from_rfc3339 to to_timestamp_micros: parse_from_rfc3339 -> timestamp_micros
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from RFC3339"
        );

        // from_rfc3339 to to_timestamp_nanos: parse_from_rfc3339 -> timestamp_nanos
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from RFC3339"
        );

        // from_rfc3339 to years_since + (parse_from_rfc3339)
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from RFC3339"
        );

        //

        //  + input_format

        // from_str to default: parse_from_str -> to_rfc3339
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .to_rfc3339();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to output_format: parse_from_rfc3339 -> format
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .format("%Y-%d-%m %H:%M")
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" output_format="%Y-%d-%m %H:%M"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render format %Y-%d-%m %H:%M from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to output_format + locale: parse_from_str -> format_localized
        #[cfg(feature = "locale")]
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .format_localized("%A, %B %C", Locale::fr_FR)
            .to_string();
        #[cfg(feature = "locale")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" output_format="%A, %B %C" locale="fr_FR"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to to_rfc2822: parse_from_str -> to_rfc2822
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .to_rfc2822();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_rfc2822=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to to_timestamp: parse_from_str -> timestamp
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to to_timestamp_millis: parse_from_str -> timestamp_millis
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp_millis()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to to_timestamp_micros: parse_from_str -> timestamp_micros
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp_micros()
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to to_timestamp_nanos: parse_from_str -> timestamp_nanos
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string()
            .as_str()[..9]
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .map(|v| v.as_str()[..9].to_string())
            .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from %Y-%m-%d %H:%M:%S string"
        );

        // from_str to years_since + (parse_from_rfc3339)
        let comparison = NaiveDateTime::parse_from_str("1989-08-09 09:30:11", "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .years_since(
                NaiveDate::from_ymd_opt(1985, 6, 16)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap()
                    .and_utc(),
            )
            .unwrap_or(0)
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_str="1989-08-09 09:30:11" input_format="%Y-%m-%d %H:%M:%S" years_since="1985-06-16T12:00:00Z"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render years since from %Y-%m-%d %H:%M:%S string"
        );

        // modifiers

        #[cfg(feature = "timezone")]
        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .with_timezone(&Tz::America__Edmonton)
            .to_rfc3339();
        #[cfg(feature = "timezone")]
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="America/Edmonton"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from RFC3339 with timezone America/Edmonton"
        );

        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .with_timezone(&FixedOffset::west_opt(6 * 3600).unwrap())
            .to_rfc3339();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="-06:00"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from RFC3339 with fixed offset -06:00"
        );

        let comparison = DateTime::parse_from_rfc3339("1989-08-09T09:30:11+02:00")
            .unwrap()
            .to_utc()
            .with_timezone(&Local)
            .to_rfc3339();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="local"}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render RFC3339 from RFC3339 in local time"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_ordinal(42)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_ordinal="42" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with ordinal 42"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_ordinal0(41)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_ordinal0="41" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with ordinal0 41"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_year(2024)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_year="2024" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with year 2024"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_month(11)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_month="11" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with month 11"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_month0(0)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_month0="0" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with month0 0"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_day(11)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_day="11" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with day 11"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_day0(12)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_day0="12" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with day0 11"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_hour(16)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_hour="16" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with hour 16"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_minute(12)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_minute="12" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with minute 12"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_second(30)
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_second="30" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with second 30"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .with_nanosecond(123456789)
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" with_nanosecond="123456789" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp with nano-second 123456789"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_months(Months::new(24))
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_months="24" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 24 months"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_weeks(4).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_weeks="4" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 4 weeks"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_days(2).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_days="2" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 2 days"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_hours(8).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_hours="8" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 8 hours"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_minutes(42).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_minutes="42" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 42 minutes"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_seconds(7).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_seconds="7" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 7 seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::try_milliseconds(42).unwrap())
            .unwrap()
            .timestamp_millis()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_milliseconds="42" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 42 milli-seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::microseconds(123))
            .unwrap()
            .timestamp_micros()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_microseconds="123" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 123 micro-seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_add_signed(TimeDelta::nanoseconds(123456789))
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" add_nanoseconds="123456789" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp plus 123456789 nano-seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_months(Months::new(24))
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_months="24" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 24 months"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_weeks(4).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_weeks="4" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 4 weeks"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_days(2).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_days="2" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 2 days"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_hours(8).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_hours="8" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 8 hours"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_minutes(42).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_minutes="42" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 42 minutes"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_seconds(7).unwrap())
            .unwrap()
            .timestamp()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_seconds="7" to_timestamp=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 7 seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::try_milliseconds(42).unwrap())
            .unwrap()
            .timestamp_millis()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_milliseconds="42" to_timestamp_millis=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 42 milli-seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::microseconds(123))
            .unwrap()
            .timestamp_micros()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_microseconds="123" to_timestamp_micros=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 123 micro-seconds"
        );

        let comparison = DateTime::from_timestamp(618658211, 0)
            .unwrap()
            .checked_sub_signed(TimeDelta::nanoseconds(123456789))
            .unwrap()
            .timestamp_nanos_opt()
            .unwrap()
            .to_string();
        assert_eq!(
            h.render_template(
                r#"{{datetime from_timestamp="618658211" sub_nanoseconds="123456789" to_timestamp_nanos=true}}"#,
                &String::new()
            )
            .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp minus 123456789 nano-seconds"
        );
    }

    #[test]
    fn it_craps() {
        use handlebars::Handlebars;

        let mut h = Handlebars::new();
        h.register_helper("datetime", Box::new(HandlebarsChronoDateTime));

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp="618658ergwerg211" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp_millis="618658ergwerg211123" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp in milli-seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp_micros="618658ergwerg211123" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp in micro-seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp_micros="12345678901234567890" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp in micro-seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp_nanos="618658ergwerg211123123" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp in nano-seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp_nanos="12345678901234567890" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid timestamp in nano-seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc2822="Wed, 09 AAA 1989 09:30:11 +0200" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid RFC2822"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1985_06_16T12_00_00Z" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid RFC3339"
        );

        assert!(
            matches!(
                h.render_template(r#"{{datetime from_str="1985-06-16T12:00:00Z" to_timestamp=true}}"#, &String::new()),
                Err(_e)
            ),
            "Failed to produce error with invalid datetime str and format"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_str="1985-06-16T12:00:00" input_format="%Y-%m-%d %H:%M:%S" to_timestamp=true}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid datetime str and format"
        );

        //

        #[cfg(feature = "locale")]
        assert!(
            matches!(
                h.render_template(r#"{{datetime output_format="%A, %B %C" locale="GAGA"}}"#, &String::new()),
                Err(_e)
            ),
            "Failed to produce error with invalid locale"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_timestamp="618658211" years_since="1985-06-16 12:00:00"}}"#,
                    &String::new()
                ),
                Err(_e)
            ),
            "Failed to produce error with invalid years since base date"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="-2500"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid offset"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_timezone="Plovdiv"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid timezone name"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_ordinal="above9000"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid ordinal"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_ordinal0="above9000"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid ordinal0"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_year="above9000"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid year"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_month="June"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid month"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_month0="June"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid month0"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_day="1st"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid day"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_day0="2nd"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid day0"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_hour="noon"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid hour"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_minute="last"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid minute"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_second="last"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid second"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" with_nanosecond="last"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid nanosecond"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_months="june"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid month"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_weeks="many"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid weeks"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_hours="many"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid hours"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_minutes="many"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid minutes"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_seconds="seven"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid seconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_milliseconds="a little"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid milliseconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_microseconds="some micros"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid microseconds"
        );

        assert!(
            matches!(
                h.render_template(
                    r#"{{datetime from_rfc3339="1989-08-09T09:30:11+02:00" add_nanoseconds="some nanos"}}"#,
                    &String::new()
                ),
                Err(_e),
            ),
            "Failed to produce error with invalid nanoseconds"
        );
    }
}
