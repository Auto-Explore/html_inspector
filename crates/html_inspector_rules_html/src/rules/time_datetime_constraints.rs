use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TimeDatetimeConstraints;

impl Rule for TimeDatetimeConstraints {
    fn id(&self) -> &'static str {
        "html.time.datetime.datatype"
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
        if !is(ctx, name, "time") {
            return;
        }

        let dt = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("datetime"),
                html_inspector_core::InputFormat::Xhtml => a.name == "datetime",
            })
            .and_then(|a| a.value.as_deref());
        let Some(dt) = dt else { return };
        let v = dt.trim();

        let ok = parse_time(v).is_some()
            || parse_date(v).is_some()
            || parse_year(v).is_some()
            || parse_month(v).is_some()
            || parse_week(v).is_some()
            || parse_datetime_local(v).is_some()
            || parse_global_datetime(v).is_some()
            || parse_duration(v).is_some();

        if !ok {
            out.push(Message::new(
                "html.time.datetime.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v}” for attribute “datetime” on element “time”."),
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn parse_time(v: &str) -> Option<()> {
    let v = v.trim();
    let (base, _frac) = v.split_once('.').unwrap_or((v, ""));
    let mut parts = base.split(':');
    let hh = parts.next().and_then(|s| s.parse::<u32>().ok())?;
    let mm = parts.next().and_then(|s| s.parse::<u32>().ok())?;
    let ss = parts.next().map_or(Some(0), |s| s.parse::<u32>().ok())?;
    if parts.next().is_some() {
        return None;
    }
    if hh > 23 || mm > 59 || ss > 59 {
        return None;
    }
    Some(())
}

fn parse_date(v: &str) -> Option<()> {
    let v = v.trim();
    let mut it = v.split('-');
    let y = it.next()?.parse::<i32>().ok()?;
    let m = it.next()?.parse::<u32>().ok()?;
    let d = it.next()?.parse::<u32>().ok()?;
    if it.next().is_some() {
        return None;
    }
    if m == 0 || m > 12 {
        return None;
    }
    let max_day = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => return None,
    };
    if d == 0 || d > max_day {
        return None;
    }
    Some(())
}

fn parse_datetime_local(v: &str) -> Option<()> {
    let v = v.trim();
    let (date, time) = v.split_once('T')?;
    parse_date(date)?;
    parse_time(time)?;
    Some(())
}

fn parse_month(v: &str) -> Option<()> {
    let v = v.trim();
    let mut it = v.split('-');
    let _y = it.next()?.parse::<i32>().ok()?;
    let m = it.next()?.parse::<u32>().ok()?;
    if it.next().is_some() {
        return None;
    }
    if (1..=12).contains(&m) {
        Some(())
    } else {
        None
    }
}

fn parse_year(v: &str) -> Option<()> {
    let v = v.trim();
    (v.len() >= 4 && v.as_bytes().iter().all(|b| b.is_ascii_digit())).then_some(())
}

fn parse_week(v: &str) -> Option<()> {
    let v = v.trim();
    let (y, w) = v.split_once("-W")?;
    let _y = y.parse::<i32>().ok()?;
    let ww = w.parse::<u32>().ok()?;
    if (1..=53).contains(&ww) {
        Some(())
    } else {
        None
    }
}

fn parse_global_datetime(v: &str) -> Option<()> {
    let v = v.trim();
    let (date, rest) = v.split_once('T')?;
    parse_date(date)?;

    // timezone: 'Z' or ±HH:MM
    if let Some(time) = rest.strip_suffix('Z') {
        parse_time(time)?;
        return Some(());
    }

    // Find last '+' or '-' for offset, but not the one in the date (already split).
    let (time, offset) = if let Some(pos) = rest.rfind('+') {
        (&rest[..pos], &rest[pos + 1..])
    } else if let Some(pos) = rest.rfind('-') {
        (&rest[..pos], &rest[pos + 1..])
    } else {
        return None;
    };
    parse_time(time)?;
    let mut it = offset.split(':');
    let hh = it.next()?.parse::<u32>().ok()?;
    let mm = it.next()?.parse::<u32>().ok()?;
    if it.next().is_some() || hh > 23 || mm > 59 {
        return None;
    }
    Some(())
}

fn parse_duration(v: &str) -> Option<()> {
    let v = v.trim();
    if !v.starts_with('P') {
        return None;
    }
    let mut i = 1usize;
    let bytes = v.as_bytes();

    let mut seen_any = false;
    let mut in_time = false;
    let mut seen_time_component = false;

    while i < bytes.len() {
        if bytes[i] == b'T' {
            if in_time {
                return None;
            }
            in_time = true;
            i += 1;
            continue;
        }

        let num_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == num_start || i >= bytes.len() {
            return None;
        }
        let num = &v[num_start..i];
        if num.len() > 1 && num.starts_with('0') {
            return None;
        }
        if num.as_bytes().iter().all(|&b| b == b'0') {
            return None;
        }

        let designator = bytes[i] as char;
        i += 1;

        let allowed = if !in_time {
            matches!(designator, 'Y' | 'M' | 'W' | 'D')
        } else {
            matches!(designator, 'H' | 'M' | 'S')
        };
        if !allowed {
            return None;
        }

        seen_any = true;
        if in_time {
            seen_time_component = true;
        }
    }

    if !seen_any {
        return None;
    }
    if in_time && !seen_time_component {
        return None;
    }
    Some(())
}

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Attribute, Config, InputFormat, Span};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    fn time_start_tag(datetime: &str) -> ParseEvent {
        ParseEvent::StartTag {
            name: "time".to_string(),
            attrs: vec![attr("datetime", Some(datetime))],
            self_closing: false,
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    #[test]
    fn parse_time_accepts_hh_mm_and_hh_mm_ss() {
        assert!(parse_time("00:00").is_some());
        assert!(parse_time("23:59:59").is_some());
        assert!(parse_time("24:00").is_none());
        assert!(parse_time("12:00:00:00").is_none());
    }

    #[test]
    fn parse_date_validates_month_day_and_leap_year() {
        assert!(parse_date("2020-02-29").is_some());
        assert!(parse_date("2019-02-29").is_none());
        assert!(parse_date("2020-00-01").is_none());
        assert!(parse_date("2020-13-01").is_none());
        assert!(parse_date("2020-01-32").is_none());
        assert!(parse_date("2020-01-01-01").is_none());
    }

    #[test]
    fn parse_month_year_week_datetime_local_cover_invalid_paths() {
        assert!(parse_month("2020-01").is_some());
        assert!(parse_month("2020-13").is_none());

        assert!(parse_year("2020").is_some());
        assert!(parse_year(" 2020 ").is_some());
        assert!(parse_year("20").is_none());
        assert!(parse_year("202a").is_none());

        assert!(parse_week("2020-W01").is_some());
        assert!(parse_week("2020-W54").is_none());

        assert!(parse_datetime_local("2020-01-01T00:00").is_some());
        assert!(parse_datetime_local("2020-01-01 00:00").is_none());
        assert!(parse_datetime_local("2020-01-01T25:00").is_none());
    }

    #[test]
    fn parse_global_datetime_accepts_z_and_offsets() {
        assert!(parse_global_datetime("2020-01-01T00:00Z").is_some());
        assert!(parse_global_datetime("2020-01-01T00:00+00:00").is_some());
        assert!(parse_global_datetime("2020-01-01T00:00-00:00").is_some());
        assert!(parse_global_datetime("2020-01-01T00:00+24:00").is_none());
        assert!(parse_global_datetime("2020-01-01T00:00").is_none());
    }

    #[test]
    fn parse_duration_covers_time_and_date_components_and_invalids() {
        assert!(parse_duration("P").is_none());
        assert!(parse_duration("P1D").is_some());
        assert!(parse_duration("  P1D ").is_some());
        assert!(parse_duration("PT1H").is_some());
        assert!(parse_duration("P1DT1H").is_some());
        assert!(parse_duration("PT").is_none());
        assert!(parse_duration("P0D").is_none());
        assert!(parse_duration("P01D").is_none());
        assert!(parse_duration("P1DT0H").is_none());
        assert!(parse_duration("P1QT1H").is_none());
        assert!(parse_duration("P1DT1H2M3S").is_some());
        assert!(parse_duration("P1DT1HT1M").is_none());
    }

    #[test]
    fn rule_reports_invalid_datetime_values_and_ignores_non_start_tags() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = TimeDatetimeConstraints::default();

        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());

        rule.on_event(&time_start_tag("bad"), &mut ctx, &mut sink);
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.time.datetime.invalid")
        );

        let mut sink = Sink::default();
        rule.on_event(&time_start_tag("2020-01-01"), &mut ctx, &mut sink);
        assert!(sink.0.is_empty());
    }
}
