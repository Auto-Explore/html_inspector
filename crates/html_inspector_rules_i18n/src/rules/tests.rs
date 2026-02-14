use html_inspector_core::{
    Attribute, Config, EventSource, InputFormat, ParseEvent, RuleSet, ValidatorError,
};

use super::meta_http_equiv_charset::MetaHttpEquivCharset;
use super::{
    lang_constraints::LangConstraints, lang_detect_warnings::LangDetectWarnings,
    meta_charset::MetaCharsetUtf8,
    unicode_normalization_nfc_warning::UnicodeNormalizationNfcWarning,
    xml_lang_consistency::XmlLangConsistency,
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

#[test]
fn xml_lang_requires_lang() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![Attribute {
                name: "xml:lang".to_string(),
                value: Some("fr".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(XmlLangConsistency);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.xml_lang.requires_lang"));
}

#[test]
fn xml_lang_consistency_is_not_checked_for_xhtml() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![Attribute {
                name: "xml:lang".to_string(),
                value: Some("fr".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(XmlLangConsistency);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.lang.xml_lang.")));
}

#[test]
fn xml_lang_consistency_is_not_checked_in_foreign_content() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            // Inside SVG insertion mode, xml:lang should not trigger the HTML-only constraint.
            ParseEvent::StartTag {
                name: "foreignObject".to_string(),
                attrs: vec![Attribute {
                    name: "xml:lang".to_string(),
                    value: Some("fr".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(XmlLangConsistency);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.lang.xml_lang.")));
}

#[test]
fn xml_lang_consistency_is_checked_inside_svg_foreignobject_children() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "foreignObject".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            // Within the HTML integration point, xml:lang should trigger the constraint again.
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "xml:lang".to_string(),
                    value: Some("fr".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(XmlLangConsistency);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.xml_lang.requires_lang"));
}

#[test]
fn xml_lang_mismatch_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![
                Attribute {
                    name: "xml:lang".to_string(),
                    value: Some("en".to_string()),
                    span: None,
                },
                Attribute {
                    name: "lang".to_string(),
                    value: Some("fr".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(XmlLangConsistency);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.xml_lang.mismatch"));
}

#[test]
fn meta_charset_non_utf8_emits_warning_in_risk_profile() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![Attribute {
                name: "charset".to_string(),
                value: Some("iso-8859-1".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaCharsetUtf8::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.meta.charset.mismatch"
            && m.severity == html_inspector_core::Severity::Warning));
}

#[test]
fn meta_charset_multiple_emits_error_on_second() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![Attribute {
                    name: "charset".to_string(),
                    value: Some("utf-8".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![Attribute {
                    name: "charset".to_string(),
                    value: Some("utf-8".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MetaCharsetUtf8::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.meta.charset.multiple"));
}

#[test]
fn xhtml_meta_charset_is_case_sensitive_for_tag_and_attribute_names() {
    // Tag name is case-sensitive in XHTML: "META" should not match.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "META".to_string(),
            attrs: vec![Attribute {
                name: "charset".to_string(),
                value: Some("iso-8859-1".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaCharsetUtf8::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());

    // Attribute name is case-sensitive in XHTML: "CHARSET" should not match.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![Attribute {
                name: "CHARSET".to_string(),
                value: Some("iso-8859-1".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaCharsetUtf8::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn lang_double_hyphen_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![Attribute {
                name: "lang".to_string(),
                value: Some("en--US".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LangConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.severity == html_inspector_core::Severity::Error));
}

#[test]
fn xhtml_lang_is_case_sensitive_on_html_element() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "HTML".to_string(),
            attrs: vec![Attribute {
                name: "lang".to_string(),
                value: Some("en".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LangConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.missing"));
}

#[test]
fn normalization_warning_emits_for_greek_extended_eta_with_oxia() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: "\u{1F75}".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "Text run is not in Unicode Normalization Form C. Should instead be “ή”. (Copy and paste that into your source document to replace the un-normalized text.)"
    }));
}

#[test]
fn normalization_warning_emits_for_vietnamese_decomposed_text() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: "Tại sao họ không thể chỉ nói tiếng Việt ?".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "Text run is not in Unicode Normalization Form C. Should instead be “Tại sao họ không thể chỉ nói tiếng Việt ”. (Copy and paste that into your source document to replace the un-normalized text.)"
    }));
}

#[test]
fn normalization_warning_suggests_normalized_prefix_excluding_last_non_composing_char() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: "e\u{301}x".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "Text run is not in Unicode Normalization Form C. Should instead be “é”. (Copy and paste that into your source document to replace the un-normalized text.)"
    }));
}

#[test]
fn normalization_warning_on_finish_includes_trailing_combining_sequence() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: "a\u{301}".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "Text run is not in Unicode Normalization Form C. Should instead be “á”. (Copy and paste that into your source document to replace the un-normalized text.)"
    }));
}

#[test]
fn normalization_warns_when_text_run_starts_with_composing_character() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: "\u{0301}a".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.message == "Text run starts with a composing character."));
}

#[test]
fn normalization_warns_when_text_run_starts_with_nfc_qc_maybe_character() {
    use unicode_normalization::{is_nfc_quick, IsNormalized};

    let Some(qc_maybe_starter) = (0u32..=0x10FFFF).find_map(|cp| {
        let c = char::from_u32(cp)?;
        if unicode_normalization::char::canonical_combining_class(c) != 0 {
            return None;
        }
        matches!(is_nfc_quick(std::iter::once(c)), IsNormalized::Maybe).then_some(c)
    }) else {
        panic!("expected to find at least one NFC_QC=maybe starter in Unicode range");
    };

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::Text {
            text: format!("{qc_maybe_starter}a"),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.message == "Text run starts with a composing character."));
}

#[test]
fn normalization_warns_for_non_nfc_attribute_values() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "data-x".to_string(),
                value: Some("\u{1F75}".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.message
            == "The value of attribute “data-x” on element “p” is not in Unicode Normalization Form C."
    }));
}

#[test]
fn normalization_warns_for_attribute_values_that_start_with_composing_character() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "data-x".to_string(),
                value: Some("\u{0301}a".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.message
            == "The value of attribute “data-x” on element “p” starts with a composing character."
    }));
}

#[test]
fn normalization_warns_for_non_nfc_processing_instruction_data() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::ProcessingInstruction {
            target: "xml-stylesheet".to_string(),
            data: "e\u{301}".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnicodeNormalizationNfcWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.message == "Processing instruction data in not in Unicode Normalization Form C."
    }));
}

#[test]
fn lang_deprecated_mo_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "body".to_string(),
            attrs: vec![Attribute {
                name: "lang".to_string(),
                value: Some("mo".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LangConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.severity == html_inspector_core::Severity::Warning));
}

#[test]
fn missing_lang_emits_warning_when_not_ignored() {
    let src = VecSource::new(InputFormat::Html, vec![]);
    let cfg = Config {
        ignore_missing_lang: false,
        ..Default::default()
    };
    let rules = RuleSet::new().push(LangConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, cfg).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.missing"
            && m.severity == html_inspector_core::Severity::Warning));
}

#[test]
fn lang_default_script_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "span".to_string(),
            attrs: vec![Attribute {
                name: "lang".to_string(),
                value: Some("ja-Jpan".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LangConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.severity == html_inspector_core::Severity::Warning));
}

#[test]
fn http_equiv_charset_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.meta.http_equiv_charset.empty"));
}

#[test]
fn http_equiv_charset_empty_with_trailing_params_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=; foo=bar".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.meta.http_equiv_charset.empty"));
}

#[test]
fn http_equiv_charset_parsing_is_case_insensitive_for_charset_and_value() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; CHARSET=UtF-8".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.meta.http_equiv_charset.")));
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.charset.unsupported"));
}

#[test]
fn http_equiv_charset_strips_leading_quote_in_label() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=\"windows-1251".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.charset.unsupported"));
}

#[test]
fn http_equiv_charset_ignores_parameters_after_semicolon() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=utf-8; foo=bar".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.meta.http_equiv_charset.")));
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.charset.unsupported"));
}

#[test]
fn http_equiv_charset_unsupported_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=madeup-123".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.charset.unsupported"));
}

#[test]
fn xhtml_http_equiv_charset_is_case_sensitive_for_tag_and_attribute_names() {
    // Tag name is case-sensitive in XHTML: "META" should not match.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "META".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());

    // Attribute name is case-sensitive in XHTML: "HTTP-EQUIV" should not match.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "HTTP-EQUIV".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("text/html; charset=".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaHttpEquivCharset);
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn lang_detect_warns_for_zh_hant_when_lang_is_en() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "觀".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.zh_hant.mismatch"));
}

#[test]
fn lang_detect_does_not_warn_for_zh_hant_when_lang_has_zh_hant_prefix_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![Attribute {
                    name: "lang".to_string(),
                    value: Some("ZH-HANT".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "觀".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.zh_hant.mismatch"));
}

#[test]
fn lang_detect_warns_for_hebrew_dir_when_lang_he_dir_ltr() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("he".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "א".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.dir_ltr"));
}

#[test]
fn lang_detect_prefers_lang_mismatch_over_dir_warning_for_hebrew() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "א".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.mismatch"));
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.dir_ltr"));
}

#[test]
fn lang_detect_does_not_warn_for_hebrew_when_lang_has_he_prefix_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![Attribute {
                    name: "lang".to_string(),
                    value: Some("HE-IL".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "א".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.mismatch"));
}

#[test]
fn lang_detect_warns_for_arabic_lang_mismatch() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "ا".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.ar.mismatch"));
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.ar.dir_ltr"));
}

#[test]
fn lang_detect_warns_for_arabic_dir_when_lang_ar_dir_ltr() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("ar".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "ا".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.ar.mismatch"));
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.ar.dir_ltr"));
}

#[test]
fn lang_detect_does_not_warn_for_arabic_when_lang_has_ar_prefix_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![Attribute {
                    name: "lang".to_string(),
                    value: Some("AR-EG".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "ا".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.lang.detect.ar.")));
}

#[test]
fn lang_detect_in_xhtml_does_not_treat_uppercase_html_tag_name_as_html() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "HTML".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "א".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code.starts_with("i18n.lang.detect.")));
}

#[test]
fn lang_detect_in_xhtml_warns_on_hebrew_lang_mismatch() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![
                    Attribute {
                        name: "lang".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "dir".to_string(),
                        value: Some("ltr".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "א".repeat(30),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LangDetectWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.mismatch"));
    assert!(!report
        .messages
        .iter()
        .any(|m| m.code == "i18n.lang.detect.he.dir_ltr"));
}
