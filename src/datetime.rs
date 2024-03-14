use chrono::{DateTime, Locale, Utc};
use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
    RenderErrorReason,
};
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
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid seconds timestamp: {}", e),
                    ))
                })?,
                0,
            )
            .ok_or::<RenderError>(
                RenderErrorReason::Other("Out-of-range number of seconds".to_string()).into(),
            )?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_millis") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_millis(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                    "Invalid milli-seconds timestamp: {}",
                    e
                )))
            })?)
            .ok_or::<RenderError>(
                RenderErrorReason::Other("Out-of-range number of milliseconds".to_string()).into(),
            )?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_micros") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_micros(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                    "Invalid micro-seconds timestamp: {}",
                    e
                )))
            })?)
            .ok_or::<RenderError>(
                RenderErrorReason::Other("Number of microseconds would be out of range for a NaiveDateTime (more than ca. 262,000 years away from common era)".to_string())
                .into(),
            )?
        } else if let Some(timestamp) = h.hash_get("from_timestamp_nanos") {
            let timestamp = timestamp.render();

            DateTime::from_timestamp_nanos(timestamp.parse().map_err(|e: ParseIntError| {
                <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(format!(
                    "Invalid nano-seconds timestamp: {}",
                    e
                )))
            })?)
        } else if let Some(input_str) = h.hash_get("from_rfc2822") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc2822(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid RFC2822 datetime format: {}", e),
                    ))
                })?
                .to_utc()
        } else if let Some(input_str) = h.hash_get("from_rfc3339") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc3339(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid RFC3339 datetime format: {}", e),
                    ))
                })?
                .to_utc()
        } else if let Some(input_str) = h.hash_get("from_str") {
            if let Some(input_format) = h.hash_get("input_format") {
                let input_str = input_str.render();
                let input_format = input_format.render();

                DateTime::parse_from_str(&input_str, &input_format)
                    .map_err(|e| {
                        <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                            format!(
                                "Invalid datetime format or format doesn't match input: {}",
                                e
                            ),
                        ))
                    })?
                    .to_utc()
            } else {
                // error, missing input format
                return Err(RenderErrorReason::Other(
                    "Missing `input_format` hash parameter".to_string(),
                )
                .into());
            }
        } else {
            Utc::now()
        };

        // MODIFIERS (by default everything is converted to UTC by the initializer)
        //
        // with_timezone
        // with_ordinal?
        // with_ordinal0?
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
        // add_secs
        // add_milliseconds
        // add_microseconds
        // add_nanos
        // sub_months
        // sub_weeks
        // sub_days
        // sub_hours
        // sub_minutes
        // sub_secs
        // sub_milliseconds
        // sub_microseconds
        // sub_nanos

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
                let locale = Locale::from_str(&locale).map_err(|_e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid locale provided: {}", &locale),
                    ))
                })?;

                format!("{}", datetime.format_localized(&output_format, locale))
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
            datetime.timestamp_nanos_opt().ok_or::<RenderError>(
                RenderErrorReason::Other("An i64 with nanosecond precision can span a range of ~584 years. This timestamp is out of range.".to_string())
                    .into(),
            )?
            .to_string()
        } else if let Some(input_rfc3339) = h.hash_get("years_since") {
            let input_rfc3339 = input_rfc3339.render();

            let base_datetime = DateTime::parse_from_rfc3339(&input_rfc3339)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid RFC3339 datetime format: {}", e),
                    ))
                })?
                .to_utc();

            datetime
                .years_since(base_datetime)
                .ok_or::<RenderError>(
                    RenderErrorReason::Other(
                        "Negative range, try swapping the parameters.".to_string(),
                    )
                    .into(),
                )?
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
    use chrono::{NaiveDate};
    use super::*;

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
        let comparison = Utc::now().format_localized("%A, %B %C", Locale::fr_FR).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime output_format="%A, %B %C" locale="fr_FR"}}"#, &String::new())
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
        let comparison = Utc::now().years_since(NaiveDate::from_ymd_opt(1989, 8, 9).unwrap().and_hms_opt(9, 30, 11).unwrap().and_utc()).unwrap_or(0).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime years_since="1989-08-09T09:30:11Z"}}"#, &String::new())
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
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" output_format="%Y-%m-%d %H:%M:%S"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp"
        );

        // from_timestamp to output_format + locale: from_timestamp -> format_localized
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().format_localized("%A, %B %C", Locale::fr_FR).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" output_format="%A, %B %C" locale="fr_FR"}}"#, &String::new())
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
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().timestamp_millis().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_timestamp_millis=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp"
        );

        // from_timestamp to to_timestamp_micros: from_timestamp -> timestamp_micros
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().timestamp_micros().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_timestamp_micros=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp"
        );

        // from_timestamp to to_timestamp_nanos: from_timestamp -> timestamp_nanos
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().timestamp_nanos_opt().unwrap_or(0).to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp"
        );

        // from_timestamp to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp(618658211, 0).unwrap().years_since(NaiveDate::from_ymd_opt(1985, 6, 16).unwrap().and_hms_opt(12, 00, 00).unwrap().and_utc()).unwrap_or(0).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp="618658211" years_since="1985-06-16T12:00:00Z"}}"#, &String::new())
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
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" output_format="%Y-%m-%d %H:%M:%S"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in milli-seconds"
        );

        // from_timestamp_millis to output_format + locale: from_timestamp_millis -> format_localized
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().format_localized("%A, %B %C", Locale::fr_FR).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" output_format="%A, %B %C" locale="fr_FR"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_rfc2822: from_timestamp_millis -> to_rfc2822
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().to_rfc2822();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" to_rfc2822=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp: from_timestamp_millis -> timestamp
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().timestamp().to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" to_timestamp=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_millis: from_timestamp_millis -> timestamp_millis
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().timestamp_millis().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_millis=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_micros: from_timestamp_millis -> timestamp_micros
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().timestamp_micros().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_micros=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to to_timestamp_nanos: from_timestamp_millis -> timestamp_nanos
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().timestamp_nanos_opt().unwrap_or(0).to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in milli-seconds"
        );

        // from_timestamp_millis to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_millis(618658211123).unwrap().years_since(NaiveDate::from_ymd_opt(1989, 8, 9).unwrap().and_hms_opt(9, 30, 11).unwrap().and_utc()).unwrap_or(0).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_millis="618658211123" years_since="1989-08-09T09:30:11Z"}}"#, &String::new())
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
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" output_format="%Y-%m-%d %H:%M:%S"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in micro-seconds"
        );

        // from_timestamp_micros to output_format + locale: from_timestamp_micros -> format_localized
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().format_localized("%A, %B %C", Locale::fr_FR).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" output_format="%A, %B %C" locale="fr_FR"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_rfc2822: from_timestamp_micros -> to_rfc2822
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().to_rfc2822();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" to_rfc2822=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp: from_timestamp_micros -> timestamp
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().timestamp().to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_millis: from_timestamp_micros -> timestamp_millis
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().timestamp_millis().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_millis=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_micros: from_timestamp_micros -> timestamp_micros
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().timestamp_micros().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_micros=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to to_timestamp_nanos: from_timestamp_micros -> timestamp_nanos
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().timestamp_nanos_opt().unwrap_or(0).to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in micro-seconds"
        );

        // from_timestamp_micros to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_micros(618658211123456).unwrap().years_since(NaiveDate::from_ymd_opt(1989, 8, 9).unwrap().and_hms_opt(9, 30, 11).unwrap().and_utc()).unwrap_or(0).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_micros="618658211123456" years_since="1989-08-09T09:30:11Z"}}"#, &String::new())
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
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" output_format="%Y-%m-%d %H:%M:%S"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render format %Y-%m-%d %H:%M:%S from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to output_format + locale: from_timestamp_nanos -> format_localized
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).format_localized("%A, %B %C", Locale::fr_FR).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" output_format="%A, %B %C" locale="fr_FR"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render localized format %A, %B %C from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_rfc2822: from_timestamp_nanos -> to_rfc2822
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).to_rfc2822();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" to_rfc2822=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render RFC2822 from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp: from_timestamp_nanos -> timestamp
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).timestamp().to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp=true}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_millis: from_timestamp_nanos -> timestamp_millis
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).timestamp_millis().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_millis=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in milli-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_micros: from_timestamp_nanos -> timestamp_micros
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).timestamp_micros().to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_micros=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in micro-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to to_timestamp_nanos: from_timestamp_nanos -> timestamp_nanos
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).timestamp_nanos_opt().unwrap_or(0).to_string().as_str()[..9].to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" to_timestamp_nanos=true}}"#, &String::new())
                .map(|v| v.as_str()[..9].to_string())
                .expect("Render error"),
            comparison,
            "Failed to render timestamp in nano-seconds from timestamp in nano-seconds"
        );

        // from_timestamp_nanos to years_since + (parse_from_rfc3339)
        let comparison = DateTime::from_timestamp_nanos(618658211123456789).years_since(NaiveDate::from_ymd_opt(1989, 8, 9).unwrap().and_hms_opt(9, 30, 11).unwrap().and_utc()).unwrap_or(0).to_string();
        assert_eq!(
            h.render_template(r#"{{datetime from_timestamp_nanos="618658211123456789" years_since="1989-08-09T09:30:11Z"}}"#, &String::new())
                .expect("Render error"),
            comparison,
            "Failed to render years since from timestamp in nano-seconds"
        );

        // parse_from_rfc2822
        // parse_from_rfc3339
        // parse_from_str + input_format
    }
}
