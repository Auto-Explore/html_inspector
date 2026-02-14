use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputDatetimeLocalConstraints;

impl Rule for InputDatetimeLocalConstraints {
    fn id(&self) -> &'static str {
        "html.input.datetime_local.datatype"
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
        if !ctx.name_is(name, "input") {
            return;
        }

        let t = ctx.attr_value(attrs, "type").unwrap_or("");
        if !t.eq_ignore_ascii_case("datetime-local") {
            return;
        }

        for attr_name in ["min", "max", "value"] {
            let Some(v) = ctx.attr_value(attrs, attr_name) else {
                continue;
            };
            if parse_datetime_local(v).is_none() {
                out.push(Message::new(
                    "html.input.datetime_local.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{v}” for attribute “{attr_name}” on element “input”."),
                    *span,
                ));
                return;
            }
        }
    }
}

fn parse_datetime_local(v: &str) -> Option<()> {
    let v = v.trim();
    let (date, time) = v.split_once('T')?;
    super_parse_date(date)?;
    parse_time(time)?;
    Some(())
}

fn super_parse_date(v: &str) -> Option<()> {
    // Keep this local copy to avoid cross-module coupling.
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

fn parse_time(v: &str) -> Option<()> {
    // Accept HH:MM[:SS[.fff]]
    let v = v.trim();
    let (base, _frac) = v.split_once('.').unwrap_or((v, ""));
    let mut parts = base.split(':');
    let hh = parts.next()?.parse::<u32>().ok()?;
    let mm = parts.next()?.parse::<u32>().ok()?;
    let ss = if let Some(s) = parts.next() {
        s.parse::<u32>().ok()?
    } else {
        0
    };
    if parts.next().is_some() {
        return None;
    }
    if hh > 23 || mm > 59 || ss > 59 {
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

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn invalid_datetime_local_attr_emits_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<input type="datetime-local" min="2024-02-30T10:00">"#,
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(InputDatetimeLocalConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.input.datetime_local.invalid")
        );
    }

    #[test]
    fn valid_datetime_local_attrs_do_not_emit_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<input type="datetime-local" value="2024-02-29T23:59:59.123">"#,
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(InputDatetimeLocalConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.input.datetime_local.invalid")
        );
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            r#"<input TYPE="datetime-local" min="2024-02-30T10:00"/>"#,
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(InputDatetimeLocalConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.input.datetime_local.invalid")
        );
    }

    #[test]
    fn xhtml_invalid_datetime_local_attr_emits_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            r#"<input type="datetime-local" min="2024-02-30T10:00"/>"#,
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(InputDatetimeLocalConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.input.datetime_local.invalid")
        );
    }

    #[test]
    fn helper_parsers_cover_more_invalid_paths() {
        assert!(parse_datetime_local("2024-01-01T10:20:30:40").is_none());
        assert!(parse_datetime_local("2024-01-01T24:00").is_none());
        assert!(parse_datetime_local("2024-01-01-02T10:00").is_none());
        assert!(parse_datetime_local("2023-02-29T10:00").is_none());
    }
}
