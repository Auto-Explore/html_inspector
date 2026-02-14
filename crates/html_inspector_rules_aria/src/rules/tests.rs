use html_inspector::{
    Attribute, Config, EventSource, InputFormat, Message, MessageSink, ParseEvent, Rule, RuleSet,
    ValidationContext, ValidatorError,
};

use super::{
    aria_br_wbr_constraints::AriaBrWbrConstraints,
    aria_checked_allowed_role::AriaCheckedAllowedRole,
    aria_checked_on_checkbox::AriaCheckedOnCheckbox,
    aria_deprecated_attributes::AriaDeprecatedAttributes,
    aria_disabled_unnecessary_on_disabled_warning::AriaDisabledUnnecessaryOnDisabledWarning,
    aria_expanded_with_command::AriaExpandedWithCommand,
    aria_expanded_with_popovertarget::AriaExpandedWithPopoverTarget,
    aria_global_properties_on_main_warning::AriaGlobalPropertiesOnMainWarning,
    aria_haspopup_on_datalist_input_warning::AriaHaspopupOnDatalistInputWarning,
    aria_hidden_constraints::AriaHiddenConstraints, aria_idref_exists::AriaIdrefExists,
    aria_multiple_main_warning::AriaMultipleMainWarning,
    aria_naming_prohibited_by_role::AriaNamingProhibitedByRole,
    aria_placeholder_with_placeholder::AriaPlaceholderWithPlaceholder,
    aria_pressed_requires_role::AriaPressedRequiresRole,
    aria_properties_supported_by_role::AriaPropertiesSupportedByRole,
    aria_readonly_requires_context::AriaReadonlyRequiresContext,
    aria_role_hierarchy_constraints::AriaRoleHierarchyConstraints,
    aria_select_role_constraints::AriaSelectRoleConstraints,
    aria_selected_on_option::AriaSelectedOnOption,
    aria_summary_constraints::AriaSummaryConstraints,
    aria_tabpanel_required_for_active_tab::AriaTabpanelRequiredForActiveTab,
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
fn aria_checked_disallowed_on_checkbox() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("checkbox".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-checked".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaCheckedOnCheckbox);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_checked.disallowed_on_input_checkbox"
            && m.severity == html_inspector::Severity::Error
    }));
}

#[test]
fn aria_hidden_true_on_body_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "body".to_string(),
            attrs: vec![Attribute {
                name: "aria-hidden".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaHiddenConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.hidden.disallowed_on_body"
            && m.severity == html_inspector::Severity::Error
            && m.message == "“aria-hidden=true” must not be used on the “body” element."
    }));
}

#[test]
fn aria_hidden_true_conflicts_with_hidden_until_found_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "aria-hidden".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
                Attribute {
                    name: "hidden".to_string(),
                    value: Some("UNTIL-FOUND".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaHiddenConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.hidden.until_found.conflict")
    );
}

#[test]
fn aria_disabled_on_role_main_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("main".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-disabled".to_string(),
                    value: Some("false".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaGlobalPropertiesOnMainWarning);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.role.main.aria_disabled.discouraged"
            && m.severity == html_inspector::Severity::Warning
            && m.message
                == "The “aria-disabled” attribute should not be used on any element which has “role=main”."
    }));
}

#[test]
fn aria_expanded_not_allowed_on_listbox_role_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("listbox".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-expanded".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new()
        .push(super::aria_properties_supported_by_role::AriaPropertiesSupportedByRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria-expanded.not_allowed" && m.severity == html_inspector::Severity::Error
    }));
}

#[test]
fn aria_idref_missing_emits_error_on_finish() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "aria-labelledby".to_string(),
                value: Some("missing".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaIdrefExists::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.idref.missing.aria-labelledby"
            && m.severity == html_inspector::Severity::Error
    }));
}

#[test]
fn aria_idref_is_ok_when_target_id_appears_later() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "aria-labelledby".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaIdrefExists::default());
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code.starts_with("aria.idref.missing."))
    );
}

#[test]
fn aria_br_role_separator_is_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "BR".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("separator".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaBrWbrConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    let msg = report
        .messages
        .iter()
        .find(|m| m.code == "aria.role.separator.disallowed_on_br_wbr")
        .expect("expected separator disallowed message");
    assert_eq!(msg.severity, html_inspector::Severity::Error);
    assert_eq!(
        msg.message,
        "Bad value “separator” for attribute “role” on element “br”."
    );
}

#[test]
fn aria_atomic_is_disallowed_on_wbr() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "wbr".to_string(),
            attrs: vec![Attribute {
                name: "aria-atomic".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaBrWbrConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_atomic.disallowed_on_br_wbr"
            && m.severity == html_inspector::Severity::Error
    }));
}

#[test]
fn aria_range_properties_on_input_number_with_native_min_max_emit_errors() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("number".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("0".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-valuemin".to_string(),
                    value: Some("0".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPropertiesSupportedByRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_valuemin.input_number.with_min")
    );
}

#[test]
fn aria_properties_supported_by_role_progress_with_max_disallows_aria_valuemax() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "progress".to_string(),
            attrs: vec![
                Attribute {
                    name: "max".to_string(),
                    value: Some("10".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-valuemax".to_string(),
                    value: Some("10".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPropertiesSupportedByRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_valuemax.progress.with_max")
    );
}

#[test]
fn aria_properties_supported_by_role_meter_discourages_aria_valuemin() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meter".to_string(),
            attrs: vec![Attribute {
                name: "aria-valuemin".to_string(),
                value: Some("0".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPropertiesSupportedByRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_valuemin.meter.discouraged"
            && m.severity == html_inspector::Severity::Warning
    }));
}

#[test]
fn aria_select_role_button_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("button".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.select.role.button.invalid")
    );
}

#[test]
fn aria_select_role_button_is_invalid_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("BUTTON".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.select.role.button.invalid")
    );
}

#[test]
fn aria_select_role_combobox_requires_aria_expanded() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("combobox".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.select.role.combobox.missing_aria_expanded")
    );
}

#[test]
fn aria_select_role_combobox_requires_aria_expanded_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("COMBOBOX".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.select.role.combobox.missing_aria_expanded")
    );
}

#[test]
fn aria_tabpanel_is_required_for_active_tab_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("TAB".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-selected".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaTabpanelRequiredForActiveTab::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.tabpanel.required_for_active_tab")
    );
}

#[test]
fn aria_tabpanel_requirement_is_satisfied_when_tabpanel_present_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("TAB".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-selected".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("TaBpAnEl".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaTabpanelRequiredForActiveTab::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.tabpanel.required_for_active_tab")
    );
}

#[test]
fn aria_multiple_visible_main_warns_on_second_role_main_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("MAIN".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("main".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaMultipleMainWarning::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.main.multiple_visible")
    );
}

#[test]
fn aria_select_role_listbox_requires_multiple_or_size_gt_one() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("listbox".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| { m.code == "aria.select.role.listbox.disallowed_without_multiple_or_size" })
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("listbox".to_string()),
                    span: None,
                },
                Attribute {
                    name: "size".to_string(),
                    value: Some("2".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectRoleConstraints);
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.select.role.listbox.disallowed_without_multiple_or_size")
    );
}

#[test]
fn aria_summary_constraints_apply_only_for_details_summary() {
    // summary as first child of details: role is disallowed.
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "details".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "summary".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaSummaryConstraints::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.summary.details_summary.disallowed_attribute.role")
    );

    // summary outside details: role is allowed for this rule.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "summary".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("button".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSummaryConstraints::default());
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.summary.details_summary.disallowed_attribute.role")
    );
}

#[test]
fn aria_summary_disallows_non_global_aria_attributes_on_details_summary() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "details".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "summary".to_string(),
                attrs: vec![Attribute {
                    name: "aria-expanded".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaSummaryConstraints::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.summary.details_summary.disallowed_attribute.aria_expanded")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "details".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "summary".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaSummaryConstraints::default());
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(!report.messages.iter().any(|m| {
        m.code
            .starts_with("aria.summary.details_summary.disallowed_attribute.")
    }));
}

#[test]
fn aria_haspopup_on_datalist_text_input_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "list".to_string(),
                    value: Some("d".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-haspopup".to_string(),
                    value: Some("listbox".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaHaspopupOnDatalistInputWarning);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.haspopup.datalist_input" && m.severity == html_inspector::Severity::Warning
    }));
}

#[test]
fn aria_haspopup_on_datalist_input_does_not_warn_for_non_text_types() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "list".to_string(),
                    value: Some("d".to_string()),
                    span: None,
                },
                Attribute {
                    name: "type".to_string(),
                    value: Some("email".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-haspopup".to_string(),
                    value: Some("listbox".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaHaspopupOnDatalistInputWarning);
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.haspopup.datalist_input")
    );
}

#[test]
fn aria_disabled_is_unnecessary_on_disabled_elements() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![
                Attribute {
                    name: "disabled".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "aria-disabled".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaDisabledUnnecessaryOnDisabledWarning),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_disabled.unnecessary_on_disabled"
            && m.severity == html_inspector::Severity::Warning
    }));

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![
                Attribute {
                    name: "disabled".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "aria-disabled".to_string(),
                    value: Some("false".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaDisabledUnnecessaryOnDisabledWarning),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_disabled.unnecessary_on_disabled")
    );
}

#[test]
fn aria_checked_allowed_role_rejects_elements_without_checked_roles() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "aria-checked".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaCheckedAllowedRole),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_checked.not_allowed")
    );
}

#[test]
fn aria_checked_allowed_role_allows_role_checkbox() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("checkbox".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-checked".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaCheckedAllowedRole),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_checked.not_allowed")
    );
}

#[test]
fn aria_checked_allowed_role_allows_role_checkbox_case_insensitive() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("CHECKBOX".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-checked".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaCheckedAllowedRole),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_checked.not_allowed")
    );
}

#[test]
fn aria_checked_allowed_role_skips_native_checkbox_inputs() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("checkbox".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-checked".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaCheckedAllowedRole),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_checked.not_allowed")
    );
}

#[test]
fn aria_naming_prohibited_by_computed_role_emits_errors() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "aria-label".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_label.prohibited_on_role.paragraph")
    );

    // <a href> computes role=link, which is not prohibited.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("https://example.com/".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code.contains("prohibited_on_role"))
    );
}

#[test]
fn aria_naming_prohibited_by_role_covers_other_attributes_and_roles() {
    // <a> without href computes role=generic, which is prohibited.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "aria-label".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_label.prohibited_on_role.generic")
    );

    // aria-labelledby and aria-braillelabel behave the same way.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![
                Attribute {
                    name: "aria-labelledby".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-braillelabel".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.insert(
        0,
        Message::new(
            "test.dummy",
            html_inspector::Severity::Info,
            html_inspector::Category::Aria,
            "x".to_string(),
            None,
        ),
    );
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_labelledby.prohibited_on_role.paragraph"
            || m.code == "aria.aria_braillelabel.prohibited_on_role.paragraph"
    }));

    // Explicit role tokens: first token wins.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("presentation somethingelse".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_label.prohibited_on_role.presentation"
            && m.severity == html_inspector::Severity::Error
    }));

    // Elements whose implicit role depends on accessible name / attributes should avoid false positives.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "section".to_string(),
            attrs: vec![Attribute {
                name: "aria-label".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.is_empty());

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "select".to_string(),
            attrs: vec![
                Attribute {
                    name: "multiple".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.is_empty());

    // Input implicit role mapping should accept common types.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("search".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn aria_readonly_requires_context_covers_xhtml_and_non_start_tag_branch() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "aria-readonly".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaReadonlyRequiresContext),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_readonly.requires_role_or_state"
            && m.severity == html_inspector::Severity::Error
    }));

    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
    let mut sink = Sink(Vec::new());
    let mut rule = AriaReadonlyRequiresContext;
    rule.on_event(
        &ParseEvent::Text {
            text: "x".to_string(),
            span: None,
        },
        &mut ctx,
        &mut sink,
    );
    assert!(sink.0.is_empty());
    sink.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert_eq!(sink.0.len(), 1);
}

#[test]
fn aria_hidden_disallowed_on_meta_and_conflicts_with_hidden_until_found() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![Attribute {
                name: "aria-hidden".to_string(),
                value: Some("false".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaHiddenConstraints),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.hidden.disallowed_on_meta")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "aria-hidden".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
                Attribute {
                    name: "hidden".to_string(),
                    value: Some("until-found".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaHiddenConstraints),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.hidden.until_found.conflict")
    );
}

#[test]
fn aria_role_hierarchy_requires_listbox_for_option_unless_owned() {
    // Without a listbox ancestor, role=option is an error.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("option".to_string()),
                    span: None,
                },
                Attribute {
                    name: "id".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaRoleHierarchyConstraints::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.role_option.requires_listbox")
    );

    // If a listbox owns the option via aria-owns, it's allowed even without containment.
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("listbox".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-owns".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("option".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "id".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AriaRoleHierarchyConstraints::default());
    let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(Message::new(
        "test.dummy",
        html_inspector::Severity::Info,
        html_inspector::Category::Aria,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.role_option.requires_listbox")
    );
}

#[test]
fn aria_role_hierarchy_warns_on_listitem_with_aria_level() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "role".to_string(),
                    value: Some("listitem".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-level".to_string(),
                    value: Some("1".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaRoleHierarchyConstraints::default());
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.role_listitem.aria_level.discouraged"
            && m.severity == html_inspector::Severity::Warning
    }));
}

#[test]
fn aria_placeholder_disallowed_with_placeholder() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "placeholder".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-placeholder".to_string(),
                    value: Some("y".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPlaceholderWithPlaceholder);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_placeholder.disallowed_with_placeholder"
            && m.severity == html_inspector::Severity::Error
    }));
}

#[test]
fn aria_readonly_requires_role_or_state() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "aria-readonly".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaReadonlyRequiresContext);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_readonly.requires_role_or_state")
    );
}

#[test]
fn aria_selected_discouraged_on_option_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "option".to_string(),
            attrs: vec![Attribute {
                name: "aria-selected".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaSelectedOnOption);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "aria.aria_selected.option.discouraged"
            && m.severity == html_inspector::Severity::Warning
    }));
}

#[test]
fn aria_pressed_on_output_without_role_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "output".to_string(),
            attrs: vec![Attribute {
                name: "aria-pressed".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPressedRequiresRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.element.output.missing_role")
    );
}

#[test]
fn aria_pressed_on_summary_without_role_is_allowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "summary".to_string(),
            attrs: vec![Attribute {
                name: "aria-pressed".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaPressedRequiresRole);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.element.output.missing_role")
    );
}

#[test]
fn aria_dropeffect_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "aria-dropeffect".to_string(),
                value: Some("copy".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaDeprecatedAttributes);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.severity == html_inspector::Severity::Warning)
    );
}

#[test]
fn aria_expanded_disallowed_with_command() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![
                Attribute {
                    name: "command".to_string(),
                    value: Some("toggle-popover".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-expanded".to_string(),
                    value: Some("false".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaExpandedWithCommand);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_expanded.disallowed_with_command")
    );
}

#[test]
fn aria_expanded_disallowed_with_popovertarget() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![
                Attribute {
                    name: "popovertarget".to_string(),
                    value: Some("f".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-expanded".to_string(),
                    value: Some("false".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AriaExpandedWithPopoverTarget);
    let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_expanded.disallowed_with_popovertarget")
    );
}

#[test]
fn aria_role_hierarchy_required_owner_roles_can_be_satisfied_by_aria_owns() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("row".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-owns".to_string(),
                        value: Some("owned_cell".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("owned_cell".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "role".to_string(),
                        value: Some("cell".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            // Missing owner => error.
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("cell".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("listbox".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-owns".to_string(),
                        value: Some("owned_option".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("owned_option".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "role".to_string(),
                        value: Some("option".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("option".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("table".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-owns".to_string(),
                        value: Some("owned_row".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("owned_row".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "role".to_string(),
                        value: Some("row".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("row".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaRoleHierarchyConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "aria.role_cell.requires_row",
        "aria.role_option.requires_listbox",
        "aria.role_row.requires_table_like",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn aria_role_hierarchy_li_descendant_constraints_and_implicit_lists() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("listitem".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-level".to_string(),
                        value: Some("2".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("listbox".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("menuitem".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("menu".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("option".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("tablist".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("treeitem".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("tree".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("tab".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            // Implicit list container without explicit role.
            ParseEvent::StartTag {
                name: "ul".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("tab".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "li".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "ul".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaRoleHierarchyConstraints::default()),
        Config::default(),
    )
    .unwrap();

    for code in [
        "aria.role_listitem.aria_level.discouraged",
        "aria.li.role.descendant_of_listbox_or_list",
        "aria.li.role.descendant_of_menu_or_menubar",
        "aria.li.role.descendant_of_tablist",
        "aria.li.role.descendant_of_tree",
        "aria.li.role.descendant_of_implicit_or_role_list",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn aria_properties_supported_by_role_emits_not_allowed_and_exercises_exceptions() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            // No role => most of these are unsupported.
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "aria-multiselectable".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-placeholder".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-selected".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-readonly".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            // <select> without role: aria-multiselectable is exempt from the no-role check.
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![Attribute {
                    name: "aria-multiselectable".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            // <option> without role: aria-selected is exempt from the no-role check.
            ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![Attribute {
                    name: "aria-selected".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            // With role, enforce supported-by-role lists.
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("button".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-placeholder".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-valuemin".to_string(),
                        value: Some("0".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            // Xhtml path in normalize_name() inside emit_not_allowed().
            ParseEvent::StartTag {
                name: "DIV".to_string(),
                attrs: vec![Attribute {
                    name: "aria-readonly".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );

    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaPropertiesSupportedByRole),
        Config::default(),
    )
    .unwrap();

    for code in [
        "aria.aria-multiselectable.not_allowed",
        "aria.aria-placeholder.not_allowed",
        "aria.aria-selected.not_allowed",
        "aria.aria-readonly.not_allowed",
        "aria.aria-valuemin.not_allowed",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn aria_naming_prohibited_by_role_emits_for_implicit_and_explicit_prohibited_roles() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("presentation".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-labelledby".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-braillelabel".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            // <a> without href is implicitly "generic" (prohibited); with href is "link" (allowed).
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![
                    Attribute {
                        name: "href".to_string(),
                        value: Some("a".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-label".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
        ],
    );

    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();

    for code in [
        "aria.aria_label.prohibited_on_role.paragraph",
        "aria.aria_labelledby.prohibited_on_role.presentation",
        "aria.aria_braillelabel.prohibited_on_role.presentation",
        "aria.aria_label.prohibited_on_role.generic",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "aria.aria_label.prohibited_on_role.link")
    );
}

#[test]
fn aria_naming_prohibited_by_role_covers_more_implicit_prohibited_roles() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "code".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "strong".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "sub".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "sup".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "del".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "caption".to_string(),
                attrs: vec![Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector::validate_events(
        src,
        RuleSet::new().push(AriaNamingProhibitedByRole),
        Config::default(),
    )
    .unwrap();
    for code in [
        "aria.aria_label.prohibited_on_role.code",
        "aria.aria_label.prohibited_on_role.strong",
        "aria.aria_label.prohibited_on_role.subscript",
        "aria.aria_label.prohibited_on_role.superscript",
        "aria.aria_label.prohibited_on_role.deletion",
        "aria.aria_label.prohibited_on_role.caption",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}
