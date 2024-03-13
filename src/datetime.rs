use chrono::{DateTime, Local, Locale, Utc};
use handlebars::{
    BlockContext, Context, Handlebars, Helper, HelperDef, HelperResult, JsonRender, Output,
    PathAndJson, RenderContext, RenderError, RenderErrorReason, Renderable, ScopedJson,
    StringOutput,
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
        r: &'reg Handlebars,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
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
                        format!("Invalid seconds timestamp: {}", e.to_string()),
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
                    e.to_string()
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
                    e.to_string()
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
                    e.to_string()
                )))
            })?)
        } else if let Some(input_str) = h.hash_get("from_rfc2822") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc2822(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid RFC2822 datetime format: {}", e.to_string()),
                    ))
                })?
                .to_utc()
        } else if let Some(input_str) = h.hash_get("from_rfc3339") {
            let input_str = input_str.render();

            DateTime::parse_from_rfc3339(&input_str)
                .map_err(|e| {
                    <RenderErrorReason as Into<RenderError>>::into(RenderErrorReason::Other(
                        format!("Invalid RFC3339 datetime format: {}", e.to_string()),
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
                                e.to_string()
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
                let locale = Locale::from_str(&locale).map_err(|e| {
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
                        format!("Invalid RFC3339 datetime format: {}", e.to_string()),
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
    use super::*;

    #[test]
    fn it_works() {
        use handlebars::Handlebars;
        use serde_json::json;

        let mut h = Handlebars::new();
        h.register_helper("datetime", Box::new(HandlebarsChronoDateTime));

        assert_eq!(
            h.render_template(r#"{{datetime}}"#, &String::new())
                .map(|s| s.as_str()[..16].to_string())
                .expect("Render error"),
            Utc::now().to_rfc3339().as_str()[..16].to_string(),
            "Failed to render RFC3339 default output"
        );
    }
}
