use html_inspector_core::ValidationContext;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Namespace {
    Html,
    Svg,
    Math,
}

pub fn namespace_for_next_start_tag(ctx: &ValidationContext, name: &str) -> Namespace {
    match ctx.foreign_insertion_namespace() {
        html_inspector_core::ForeignContentNamespace::Html => {}
        html_inspector_core::ForeignContentNamespace::Svg => return Namespace::Svg,
        html_inspector_core::ForeignContentNamespace::Math => return Namespace::Math,
    }

    if ctx.name_is(name, "svg") {
        Namespace::Svg
    } else if ctx.name_is(name, "math") {
        Namespace::Math
    } else {
        Namespace::Html
    }
}

pub fn last_open_svg_integration_point(ctx: &ValidationContext) -> Option<&'static str> {
    let mut insertion = Namespace::Html;
    let mut last: Option<&'static str> = None;
    for name in ctx.open_elements() {
        let name_norm = normalize(ctx, name);
        let el_ns = match insertion {
            Namespace::Html => match name_norm.as_ref() {
                "svg" => Namespace::Svg,
                "math" => Namespace::Math,
                _ => Namespace::Html,
            },
            other => other,
        };

        if el_ns == Namespace::Svg {
            insertion = match name_norm.as_ref() {
                "foreignobject" => {
                    last = Some("foreignobject");
                    Namespace::Html
                }
                "desc" => {
                    last = Some("desc");
                    Namespace::Html
                }
                "title" => {
                    last = Some("title");
                    Namespace::Html
                }
                _ => Namespace::Svg,
            };
        } else {
            insertion = el_ns;
        }
    }
    last
}

fn normalize<'a>(ctx: &ValidationContext, name: &'a str) -> Cow<'a, str> {
    match ctx.format {
        html_inspector_core::InputFormat::Html => {
            html_inspector_core::ascii_lowercase_cow_if_needed(name)
        }
        html_inspector_core::InputFormat::Xhtml => Cow::Borrowed(name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    use html_inspector_core::{
        Config, EventSource, InputFormat, Interest, ParseEvent, Rule, RuleSet, ValidatorError,
        validate_events,
    };

    struct VecSource {
        name: String,
        format: InputFormat,
        events: std::vec::IntoIter<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "vec".to_string(),
                format,
                events: events.into_iter(),
            }
        }
    }

    impl EventSource for VecSource {
        fn source_name(&self) -> &str {
            &self.name
        }

        fn format(&self) -> InputFormat {
            self.format
        }

        fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
            Ok(self.events.next())
        }
    }

    struct NsRecorder {
        out: Arc<Mutex<Vec<(String, Namespace)>>>,
    }

    struct SvgIntegrationPointRecorder {
        out: Arc<Mutex<Vec<(String, Option<&'static str>)>>>,
    }

    impl Rule for NsRecorder {
        fn id(&self) -> &'static str {
            "test.ns_recorder"
        }

        fn interest(&self) -> Interest {
            Interest::START_TAG
        }

        fn on_event(
            &mut self,
            event: &ParseEvent,
            ctx: &mut ValidationContext,
            _out: &mut dyn html_inspector_core::MessageSink,
        ) {
            let ParseEvent::StartTag { name, .. } = event else {
                return;
            };
            let ns = namespace_for_next_start_tag(ctx, name);
            self.out.lock().unwrap().push((name.clone(), ns));
        }
    }

    impl Rule for SvgIntegrationPointRecorder {
        fn id(&self) -> &'static str {
            "test.svg_integration_point_recorder"
        }

        fn interest(&self) -> Interest {
            Interest::START_TAG
        }

        fn on_event(
            &mut self,
            event: &ParseEvent,
            ctx: &mut ValidationContext,
            _out: &mut dyn html_inspector_core::MessageSink,
        ) {
            let ParseEvent::StartTag { name, .. } = event else {
                return;
            };
            let ip = last_open_svg_integration_point(ctx);
            self.out.lock().unwrap().push((name.clone(), ip));
        }
    }

    #[test]
    fn namespace_for_next_start_tag_tracks_svg_integration_points_in_html()
    -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Namespace)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = NsRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "foreignObject".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "math".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "svg".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "math".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "foreignObject".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "svg".to_string(),
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Html, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), Namespace::Svg),
            ("foreignObject".to_string(), Namespace::Svg),
            ("math".to_string(), Namespace::Math),
            ("svg".to_string(), Namespace::Math),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn namespace_for_next_start_tag_is_case_sensitive_in_xhtml() -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Namespace)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = NsRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "SVG".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Xhtml, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("SVG".to_string(), Namespace::Html),
            ("svg".to_string(), Namespace::Svg),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn namespace_for_next_start_tag_does_not_switch_outside_html_insertion()
    -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Namespace)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = NsRecorder { out: out.clone() };

        // When the insertion namespace is SVG, <math> start tags should stay in SVG namespace.
        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "math".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Html, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), Namespace::Svg),
            ("math".to_string(), Namespace::Svg),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn last_open_svg_integration_point_tracks_last_open_in_html() -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Option<&'static str>)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = SvgIntegrationPointRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "foreignObject".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Html, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), None),
            ("foreignObject".to_string(), None),
            ("p".to_string(), Some("foreignobject")),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn last_open_svg_integration_point_recognizes_foreignobject_all_caps_in_html()
    -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Option<&'static str>)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = SvgIntegrationPointRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "FOREIGNOBJECT".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Html, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), None),
            ("FOREIGNOBJECT".to_string(), None),
            ("p".to_string(), Some("foreignobject")),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn last_open_svg_integration_point_is_case_sensitive_in_xhtml() -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Option<&'static str>)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = SvgIntegrationPointRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "foreignObject".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Xhtml, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), None),
            ("foreignObject".to_string(), None),
            ("p".to_string(), None),
        ];
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn last_open_svg_integration_point_recognizes_title_in_html() -> Result<(), ValidatorError> {
        let out: Arc<Mutex<Vec<(String, Option<&'static str>)>>> = Arc::new(Mutex::new(Vec::new()));
        let rule = SvgIntegrationPointRecorder { out: out.clone() };

        let events = vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "TiTlE".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: None,
            },
        ];

        let source = VecSource::new(InputFormat::Html, events);
        let _report = validate_events(source, RuleSet::new().push(rule), Config::default())?;

        let got = out.lock().unwrap().clone();
        let expected = vec![
            ("svg".to_string(), None),
            ("TiTlE".to_string(), None),
            ("p".to_string(), Some("title")),
        ];
        assert_eq!(got, expected);
        Ok(())
    }
}
