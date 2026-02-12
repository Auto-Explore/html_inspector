use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DelDatetimeConstraints;

impl Rule for DelDatetimeConstraints {
    fn id(&self) -> &'static str {
        "html.del.datetime.datatype"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };
        let element = if ctx.name_is(name, "del") {
            "del"
        } else if ctx.name_is(name, "ins") {
            "ins"
        } else {
            return;
        };

        let datetime = attrs
            .iter()
            .find(|a| ctx.name_is(&a.name, "datetime"))
            .and_then(|a| a.value.as_deref());
        let Some(datetime) = datetime else { return };

        // If the attribute value already contains U+FFFD replacement characters (typically due to
        // tokenizer character-reference errors), avoid emitting a redundant "Bad value ..." error
        // that would otherwise sort before the tokenizer error in the vnu suite.
        if datetime.contains('\u{FFFD}') {
            return;
        }

        // Leading/trailing ASCII whitespace is not allowed (vnu suite).
        if datetime != datetime.trim_matches(|c: char| c.is_ascii_whitespace()) {
            out.push(Message::new(
                "html.del.datetime.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{datetime}” for attribute “datetime” on element “{element}”."),
                *span,
            ));
            return;
        }

        match validate_datetime(datetime) {
            DatetimeValidation::Ok => {}
            DatetimeValidation::Warn => out.push(Message::new(
                "html.del.datetime.warn",
                Severity::Warning,
                Category::Html,
                format!("Bad value “{datetime}” for attribute “datetime” on element “{element}”."),
                *span,
            )),
            DatetimeValidation::Invalid => out.push(Message::new(
                "html.del.datetime.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{datetime}” for attribute “datetime” on element “{element}”."),
                *span,
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatetimeValidation {
    Ok,
    Warn,
    Invalid,
}

#[inline]
fn ok_or_warn(warn: bool) -> DatetimeValidation {
    if warn {
        DatetimeValidation::Warn
    } else {
        DatetimeValidation::Ok
    }
}

fn validate_datetime(v: &str) -> DatetimeValidation {
    // Accept:
    // - date (YYYY-MM-DD)
    // - global date and time with timezone (YYYY-MM-DD[T ]HH:MM[:SS][.s]+HHMM/+HH:MM/-HHMM/-HH:MM/Z)
    // Additional constraints emit warnings rather than errors (vnu suite).

    if let Some((_year, warn_year)) = parse_date_only(v) {
        return ok_or_warn(warn_year);
    }

    parse_global_datetime(v)
}

fn parse_date_only(v: &str) -> Option<(i32, bool)> {
    let (y, rest) = v.split_once('-')?;
    let (m, d) = rest.split_once('-')?;
    if d.contains('-') {
        return None;
    }
    if y.len() < 4 || !y.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if m.len() != 2 || d.len() != 2 {
        return None;
    }
    let year: i32 = y.parse().ok()?;
    if year == 0 {
        return None;
    }
    let month: u32 = m.parse().ok()?;
    let day: u32 = d.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let max_day = days_in_month(year, month);
    if day < 1 || day > max_day {
        return None;
    }
    let warn_year = !(1000..=9999).contains(&year);
    Some((year, warn_year))
}

fn parse_global_datetime(v: &str) -> DatetimeValidation {
    let (date, rest) = if let Some((d, r)) = v.split_once('T') {
        (d, r)
    } else if let Some((d, r)) = v.split_once(' ') {
        (d, r)
    } else {
        return DatetimeValidation::Invalid;
    };

    let Some((_year, warn_year)) = parse_date_only(date) else {
        return DatetimeValidation::Invalid;
    };

    let (time_part, tz_part) = if let Some(time) = rest.strip_suffix('Z') {
        (time, "Z")
    } else if let Some(pos) = rest.rfind(['+', '-']) {
        (&rest[..pos], &rest[pos..])
    } else {
        return DatetimeValidation::Invalid;
    };

    if !parse_time(time_part) {
        return DatetimeValidation::Invalid;
    }

    let mut warn = warn_year;

    if tz_part == "Z" {
        return ok_or_warn(warn);
    }

    let Some((offset_minutes, warn_tz)) = parse_tz_offset_minutes(tz_part) else {
        return DatetimeValidation::Invalid;
    };
    warn = warn || warn_tz;

    // vnu suite: offsets outside -12:00..+14:00 are warnings (not errors).
    if !(-12 * 60..=14 * 60).contains(&offset_minutes) {
        warn = true;
    }

    ok_or_warn(warn)
}

fn parse_time(v: &str) -> bool {
    if v.is_empty() || v.chars().any(|c| c.is_ascii_whitespace()) {
        return false;
    }

    let (base, frac, had_dot) = match v.split_once('.') {
        Some((a, b)) => (a, b, true),
        None => (v, "", false),
    };
    if had_dot {
        if frac.is_empty() {
            return false;
        }
        if !(1..=3).contains(&frac.len()) {
            return false;
        }
        if !frac.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }

    let mut iter = base.split(':');
    let Some(hh_str) = iter.next() else {
        return false;
    };
    let Some(mm_str) = iter.next() else {
        return false;
    };
    let ss_str = iter.next();
    if iter.next().is_some() {
        return false;
    }
    // Fractions are only allowed when seconds are present (suite coverage).
    if had_dot && ss_str.is_none() {
        return false;
    }

    if hh_str.len() != 2 || mm_str.len() != 2 {
        return false;
    }
    if ss_str.is_some_and(|s| s.len() != 2) {
        return false;
    }

    let Ok(hh) = hh_str.parse::<u32>() else {
        return false;
    };
    let Ok(mm) = mm_str.parse::<u32>() else {
        return false;
    };
    let ss: u32 = if let Some(ss_str) = ss_str {
        let Ok(ss) = ss_str.parse::<u32>() else {
            return false;
        };
        ss
    } else {
        0
    };
    hh <= 23 && mm <= 59 && ss <= 59
}

fn parse_tz_offset_minutes(v: &str) -> Option<(i32, bool)> {
    let sign = match v.as_bytes().first()? {
        b'+' => 1i32,
        b'-' => -1i32,
        _ => return None,
    };
    let rest = &v[1..];
    let (hh_str, mm_str) = if let Some((hh, mm)) = rest.split_once(':') {
        (hh, mm)
    } else if rest.len() == 4 && rest.is_ascii() {
        rest.split_at(2)
    } else {
        return None;
    };

    if hh_str.len() != 2 || mm_str.len() != 2 {
        return None;
    }
    let hh: i32 = hh_str.parse().ok()?;
    let mm: i32 = mm_str.parse().ok()?;
    if !(0..=23).contains(&hh) || !(0..=59).contains(&mm) {
        return None;
    }
    let total = sign * (hh * 60 + mm);

    // vnu suite: offsets with minutes not in {00, 30, 45} are warnings.
    let warn = !matches!(mm, 0 | 30 | 45);
    Some((total, warn))
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet, Severity};
    use html_inspector_html::HtmlEventSource;

    fn validate(html: &str) -> html_inspector_core::Report {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(DelDatetimeConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn datetime_with_ascii_whitespace_is_invalid() {
        let report = validate(r#"<del datetime=" 2020-01-01"></del>"#);
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.invalid"));
    }

    #[test]
    fn out_of_range_year_and_tz_minutes_emit_warning() {
        let report = validate(r#"<ins datetime="0999-01-01T00:00:00+12:17"></ins>"#);
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.warn" && m.severity == Severity::Warning));
    }

    #[test]
    fn invalid_datetime_with_replacement_character_is_silently_ignored() {
        let report = validate("<del datetime=\"\u{FFFD}\"></del>");
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.invalid" || m.code == "html.del.datetime.warn"));
    }

    #[test]
    fn xhtml_datetime_attribute_lookup_is_case_sensitive() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Xhtml, r#"<del datetime=" 2020-01-01"/>"#)
                .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(DelDatetimeConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.invalid"));
    }

    #[test]
    fn helper_parsers_cover_invalid_time_tz_and_default_month_branch() {
        assert!(!parse_time("00:00:00.a"));
        assert!(!parse_time("00:00.1"));
        assert!(!parse_time("xx:00"));
        assert!(!parse_time("00:xx"));
        assert!(!parse_time("00:00:xx"));

        assert!(parse_tz_offset_minutes("Z").is_none());
        assert_eq!(days_in_month(2024, 0), 31);
    }

    #[test]
    fn parse_time_accepts_valid_shapes() {
        assert!(parse_time("00:00"));
        assert!(parse_time("23:59"));
        assert!(parse_time("00:00:00"));
        assert!(parse_time("12:34:56"));
        assert!(parse_time("12:34:56.1"));
        assert!(parse_time("12:34:56.12"));
        assert!(parse_time("12:34:56.123"));
    }

    #[test]
    fn parse_time_and_date_only_cover_more_invalid_shapes() {
        assert!(!parse_time(""));
        assert!(!parse_time("00: 00"));
        assert!(!parse_time("00"));
        assert!(!parse_time("0:00"));
        assert!(!parse_time("00:0"));
        assert!(!parse_time("00:00:0"));
        assert!(!parse_time("00:00:00:00"));
        assert!(!parse_time("00:00:00."));
        assert!(!parse_time("00:00:00.1234"));
        assert!(!parse_time("24:00"));

        assert!(parse_date_only("2020-01").is_none());
        assert!(parse_date_only("2020-01-01-01").is_none());
        assert!(parse_date_only("abc-01-01").is_none());
        assert!(parse_date_only("0000-01-01").is_none());
    }

    #[test]
    fn tz_offsets_without_colons_parse() {
        assert_eq!(
            parse_tz_offset_minutes("+1230"),
            Some((12 * 60 + 30, false))
        );
        assert_eq!(
            parse_tz_offset_minutes("-1245"),
            Some((-(12 * 60 + 45), false))
        );
        assert_eq!(parse_tz_offset_minutes("+1234"), Some((12 * 60 + 34, true)));
    }

    #[test]
    fn validate_datetime_classifies_date_only_and_global_datetime() {
        assert_eq!(validate_datetime("2020-01-01"), DatetimeValidation::Ok);
        assert_eq!(validate_datetime("0999-01-01"), DatetimeValidation::Warn);
        assert_eq!(validate_datetime("0000-01-01"), DatetimeValidation::Invalid);

        assert_eq!(
            validate_datetime("2020-01-01T00:00Z"),
            DatetimeValidation::Ok
        );
        assert_eq!(
            validate_datetime("2020-01-01T00:00+12:17"),
            DatetimeValidation::Warn
        );
        assert_eq!(
            validate_datetime("2020-01-01T00:00+15:00"),
            DatetimeValidation::Warn
        );
        assert_eq!(
            validate_datetime("2020-01-01T00:00"),
            DatetimeValidation::Invalid
        );
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}
