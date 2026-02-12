use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TimeTextContentConstraints {
    in_time: bool,
    has_datetime_attr: bool,
    saw_non_whitespace: bool,
    buf: String,
    span: Option<html_inspector_core::Span>,
}

impl Rule for TimeTextContentConstraints {
    fn id(&self) -> &'static str {
        "html.time.text_content"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                if !ctx.name_is(name, "time") {
                    return;
                }
                self.in_time = true;
                self.has_datetime_attr = ctx.has_attr(attrs, "datetime");
                self.saw_non_whitespace = false;
                self.buf.clear();
                self.span = *span;
            }
            ParseEvent::Text { text, .. } => {
                if !self.in_time || self.has_datetime_attr {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    self.saw_non_whitespace = true;
                }
                self.buf.push_str(text);
            }
            ParseEvent::EndTag { name, .. } => {
                if !ctx.name_is(name, "time") || !self.in_time {
                    return;
                }
                self.in_time = false;
                if self.has_datetime_attr {
                    self.reset();
                    return;
                }
                let value = self.buf.trim();
                if self.saw_non_whitespace && !value.is_empty() && !is_valid_time_text(value) {
                    out.push(Message::new(
                        "html.time.text_content.invalid",
                        Severity::Error,
                        Category::Html,
                        "The text content of element “time” was not in the required format.",
                        self.span,
                    ));
                }
                self.reset();
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.reset();
    }
}

impl TimeTextContentConstraints {
    fn reset(&mut self) {
        self.in_time = false;
        self.has_datetime_attr = false;
        self.saw_non_whitespace = false;
        self.buf.clear();
        self.span = None;
    }
}

fn is_valid_time_text(v: &str) -> bool {
    parse_time(v).is_some()
        || parse_date(v).is_some()
        || parse_year(v).is_some()
        || parse_month(v).is_some()
        || parse_week(v).is_some()
        || parse_datetime_local(v).is_some()
        || parse_global_datetime(v).is_some()
        || parse_duration(v).is_some()
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

    if let Some(time) = rest.strip_suffix('Z') {
        parse_time(time)?;
        return Some(());
    }

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

        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if start == i {
            return None;
        }
        let Some(unit) = bytes.get(i) else {
            return None;
        };
        let unit = *unit as char;
        i += 1;
        match unit {
            'Y' | 'M' | 'W' | 'D' if !in_time => {}
            'H' | 'S' => {
                in_time = true;
                seen_time_component = true;
            }
            'M' if in_time => {
                seen_time_component = true;
            }
            _ => return None,
        }
        seen_any = true;
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

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn time_text_without_datetime_is_validated() {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, "<time>bad</time>").unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TimeTextContentConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.time.text_content.invalid"));
    }

    #[test]
    fn time_text_valid_date_is_allowed() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, "<time>2024-01-02</time>").unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TimeTextContentConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }

    #[test]
    fn time_text_is_ignored_when_datetime_attr_present() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<time datetime=\"2024-01-02\">bad</time>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(TimeTextContentConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }
}
