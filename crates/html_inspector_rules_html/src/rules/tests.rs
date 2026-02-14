use html_inspector_core::{
    Attribute, Config, EventSource, InputFormat, ParseEvent, Rule, RuleSet, ValidatorError,
};
use html_inspector_html::HtmlEventSource;

use super::attribute_not_allowed_constraints::AttributeNotAllowedConstraints;
use super::mathml_constraints::MathmlConstraints;
use super::p_disallowed_parent_constraints::PDisallowedParentConstraints;
use super::unknown_element_constraints::UnknownElementConstraints;
use super::video_src_constraints::VideoSrcConstraints;

use super::{
    a_download_constraints::ADownloadConstraints,
    a_href_button_descendant::AHrefButtonDescendant,
    a_href_constraints::AHrefConstraints,
    a_transparent_content_model::ATransparentContentModel,
    accesskey_constraints::AccesskeyConstraints,
    address_constraints::AddressConstraints,
    area_coords_constraints::AreaCoordsConstraints,
    area_map_ancestor::AreaRequiresMapAncestor,
    article_heading_warning::ArticleHeadingWarning,
    audio_src_constraints::AudioSrcConstraints,
    audio_transparent_content_model::AudioTransparentContentModel,
    autocomplete_constraints::AutocompleteConstraints,
    autofocus_constraints::AutofocusConstraints,
    base_element_constraints::BaseElementConstraints,
    base_href_constraints::BaseHrefConstraints,
    base_in_body::BaseInBody,
    bdo_dir::BdoDir,
    blockquote_cite_constraints::BlockquoteCiteConstraints,
    button_formaction_constraints::ButtonFormactionConstraints,
    canvas_transparent_content_model::CanvasTransparentContentModel,
    commandfor_constraints::CommandforConstraints,
    data_attribute_constraints::DataAttributeConstraints,
    del_cite_constraints::DelCiteConstraints,
    del_datetime_constraints::DelDatetimeConstraints,
    details_summary_constraints::DetailsSummaryConstraints,
    dialog_constraints::DialogConstraints,
    dl_child_content::DlChildContent,
    dl_div_group_constraints::DlDivGroupConstraints,
    dl_duplicate_dt_name::DlDuplicateDtName,
    dl_structure_constraints::DlStructureConstraints,
    doctype_required::DoctypeRequired,
    dt_descendant_constraints::DtDescendantConstraints,
    duplicate_id::DuplicateId,
    element_specific_attributes::ElementSpecificAttributes,
    embed_constraints::EmbedConstraints,
    empty_heading_warning::EmptyHeadingWarning,
    enterkeyhint_constraints::EnterkeyhintConstraints,
    figure_figcaption::FigureFigcaption,
    figure_table_caption_warning::FigureTableCaptionWarning,
    footer_constraints::FooterConstraints,
    form_action_constraints::FormActionConstraints,
    form_attribute_constraints::FormAttributeConstraints,
    header_constraints::HeaderConstraints,
    headingoffset_constraints::HeadingoffsetConstraints,
    html5ever_parse_errors::Html5EverParseErrors,
    id_datatype_constraints::IdDatatypeConstraints,
    iframe_constraints::IframeConstraints,
    iframe_sandbox_constraints::IFrameSandboxConstraints,
    img_alt_required::ImgAltRequired,
    img_ismap_anchor_ancestor::ImgIsmapAnchorAncestor,
    img_obsolete_attributes::ImgObsoleteAttributes,
    img_role_constraints::ImgRoleConstraints,
    img_sizes_auto_loading_lazy::ImgSizesAutoRequiresLazyLoading,
    img_sizes_constraints::ImgSizesConstraints,
    img_src_constraints::ImgSrcConstraints,
    img_srcset_constraints::ImgSrcsetConstraints,
    img_srcset_sizes_required::ImgSrcsetSizesRequired,
    img_usemap_constraints::ImgUsemapConstraints,
    implied_p_end_tag::ImpliedPEndTag,
    input_attribute_allowed_types::InputAttributeAllowedTypes,
    input_attribute_disallowed_by_type::InputAttributeDisallowedByType,
    input_checkbox_role_button_aria_pressed::InputCheckboxRoleButtonAriaPressed,
    input_color_constraints::InputColorConstraints,
    input_date_constraints::InputDateConstraints,
    input_datetime_local_constraints::InputDatetimeLocalConstraints,
    input_formaction_constraints::InputFormactionConstraints,
    input_image_src_constraints::InputImageSrcConstraints,
    input_list_constraints::InputListConstraints,
    input_month_constraints::InputMonthConstraints,
    input_name_constraints::InputNameConstraints,
    input_number_constraints::InputNumberConstraints,
    input_range_constraints::InputRangeConstraints,
    input_size_constraints::InputSizeConstraints,
    input_step_constraints::InputStepConstraints,
    input_time_constraints::InputTimeConstraints,
    input_type_constraints::InputTypeConstraints,
    input_url_value_constraints::InputUrlValueConstraints,
    input_week_constraints::InputWeekConstraints,
    ins_cite_constraints::InsCiteConstraints,
    is_attribute_constraints::IsAttributeConstraints,
    label_control_count::LabelControlCount,
    label_for_constraints::LabelForConstraints,
    li_parent_constraints::LiParentConstraints,
    li_value_constraints::LiValueConstraints,
    link_constraints::LinkConstraints,
    link_href_constraints::LinkHrefConstraints,
    main_constraints::MainConstraints,
    map_constraints::MapConstraints,
    math_role_warning::MathRoleWarning,
    meta_element_constraints::MetaElementConstraints,
    meta_refresh_constraints::MetaRefreshConstraints,
    meter_constraints::MeterConstraints,
    microdata_constraints::MicrodataConstraints,
    mimetype_constraints::{LinkMimetypeConstraints, ObjectMimetypeConstraints, is_mime_type},
    non_void_self_closing_syntax::NonVoidSelfClosingSyntax,
    obj_element_constraints::ObjElementConstraints,
    object_data_constraints::ObjectDataConstraints,
    obsolete_elements::ObsoleteElements,
    ol_start_constraints::OlStartConstraints,
    option_constraints::OptionConstraints,
    p_end_tag_scope::PEndTagScope,
    picture_attribute_constraints::PictureAttributeConstraints,
    picture_content_model_constraints::PictureContentModelConstraints,
    picture_parent_constraints::PictureParentConstraints,
    picture_source_media_all_constraints::PictureSourceMediaAllConstraints,
    picture_source_selection_constraints::PictureSourceSelectionConstraints,
    picture_unclosed_end_of_file::PictureUnclosedEndOfFile,
    placeholder_constraints::PlaceholderConstraints,
    popover_constraints::PopoverConstraints,
    progress_constraints::ProgressConstraints,
    q_cite_constraints::QCiteConstraints,
    rel_typo_constraints::RelTypoConstraints,
    ruby_constraints::RubyConstraints,
    script_constraints::ScriptConstraints,
    script_importmap_constraints::ScriptImportmapConstraints,
    script_integrity_constraints::ScriptIntegrityConstraints,
    script_speculationrules_constraints::ScriptSpeculationrulesConstraints,
    select_constraints::SelectConstraints,
    spellcheck_constraints::SpellcheckConstraints,
    style_constraints::StyleConstraints,
    target_browsing_context_constraints::TargetBrowsingContextConstraints,
    textarea_constraints::TextareaConstraints,
    time_datetime_constraints::TimeDatetimeConstraints,
    title_constraints::TitleConstraints,
    tokenizer_parse_errors::TokenizerParseErrors,
    track_constraints::TrackConstraints,
    unnecessary_role_warnings::UnnecessaryRoleWarnings,
    url_constraints::UrlConstraints,
    void_element_end_tag::VoidElementEndTag,
    xml_stylesheet_processing_instruction::XmlStylesheetProcessingInstruction,
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
fn doctype_required_emits_when_missing() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DoctypeRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.doctype.missing")
    );
}

#[test]
fn doctype_required_does_not_double_report_when_parser_already_reported_missing_doctype() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::ParseError {
                code: "html.parser.start_tag_without_doctype".to_string(),
                message:
                    "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”."
                        .to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DoctypeRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.doctype.missing")
    );
}

#[test]
fn html5ever_missing_doctype_is_reported_only_as_parse_error() {
    let src = HtmlEventSource::from_str(
        "t",
        InputFormat::Html,
        "<html><head><title>x</title></head><body></body></html>",
    )
    .unwrap();
    let rules = RuleSet::new()
        .push(TokenizerParseErrors::default())
        .push(DoctypeRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parser.start_tag_without_doctype")
    );
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.doctype.missing")
    );
}

#[test]
fn html5ever_parse_errors_are_reported() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::ParseError {
            code: "html5.parse_error".to_string(),
            message: "Unexpected end tag.".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(Html5EverParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| m.code == "html.parse.error"));
}

#[test]
fn html5ever_parse_errors_ignore_non_matching_events_and_codes() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::ParseError {
                code: "other".to_string(),
                message: "nope".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(Html5EverParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert_eq!(report.messages.len(), 0);
}

#[test]
fn title_constraints_emits_when_title_missing() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(TitleConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.head.title.missing")
    );
}

#[test]
fn title_constraints_emits_when_title_empty() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "title".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: " \n\t".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "title".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(TitleConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| m.code == "html.title.empty"));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.head.title.missing")
    );
}

#[test]
fn tokenizer_parse_errors_are_reported() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::ParseError {
            code: "html.tokenizer.eof_after_lt".to_string(),
            message: "End of file after “<”.".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.tokenizer.eof_after_lt")
    );
}

#[test]
fn tokenizer_parse_errors_skip_doctype_missing_and_non_parse_errors() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::ParseError {
                code: "html.doctype.missing".to_string(),
                message: "Missing doctype.".to_string(),
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert_eq!(report.messages.len(), 0);
}

#[test]
fn unknown_element_constraints_normalizes_names_in_html() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "BODY".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "UnknownHTMLElement".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(UnknownElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.not_allowed"
            && m.message
                == "Element “unknownhtmlelement” not allowed as child of “body” in this context."
    }));
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.completely_unknown"
            && m.message
                == "The “unknownhtmlelement” element is a completely-unknown element that is not allowed anywhere in any HTML content."
    }));
}

#[test]
fn unknown_element_constraints_is_case_sensitive_in_xhtml() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "BODY".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "unknownhtmlelement".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "UnknownHTMLElement".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(UnknownElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.not_allowed"
            && m.message
                == "Element “unknownhtmlelement” not allowed as child of “BODY” in this context."
    }));
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.not_allowed"
            && m.message
                == "Element “UnknownHTMLElement” not allowed as child of “BODY” in this context."
    }));
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.completely_unknown"
            && m.message
                == "The “unknownhtmlelement” element is a completely-unknown element that is not allowed anywhere in any HTML content."
    }));
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.completely_unknown"
            && m.message
                == "The “UnknownHTMLElement” element is a completely-unknown element that is not allowed anywhere in any HTML content."
    }));
    assert!(!report.messages.iter().any(|m| {
        m.code == "html.unknown_element.not_allowed"
            && m.message
                == "Element “unknownhtmlelement” not allowed as child of “body” in this context."
    }));
}

#[test]
fn unknown_element_constraints_does_not_flag_custom_elements() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
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
                name: "my-widget".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(UnknownElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn unknown_element_constraints_is_ignored_inside_template() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "unknownhtmlelement".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(UnknownElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn unknown_element_constraints_in_svg_preserves_parent_name_case() {
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
                name: "linearGradient".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "foo".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new()
        .push(super::unknown_element_constraints::UnknownSvgElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report.messages.iter().any(|m| {
            m.code == "html.unknown_element.not_allowed"
                && m.message
                    == "Element “foo” not allowed as child of “linearGradient” in this context."
        }),
        "unexpected messages: {:?}",
        report.messages
    );
    assert!(report.messages.iter().any(|m| {
        m.code == "html.unknown_element.completely_unknown"
            && m.message
                == "The “foo” element is a completely-unknown element that is not allowed anywhere in any SVG content."
    }));
}

#[test]
fn picture_unclosed_on_finish_emits_error_for_html_only() {
    let html_src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "picture".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        html_src,
        RuleSet::new().push(PictureUnclosedEndOfFile::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.eof.open_elements")
    );

    let no_picture = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        no_picture,
        RuleSet::new().push(PictureUnclosedEndOfFile::default()),
        Config::default(),
    )
    .unwrap();
    assert_eq!(report.messages.len(), 0);

    let xhtml_src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "picture".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        xhtml_src,
        RuleSet::new().push(PictureUnclosedEndOfFile::default()),
        Config::default(),
    )
    .unwrap();
    assert_eq!(report.messages.len(), 0);
}

#[test]
fn video_src_empty_in_xhtml_emits_error() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "video".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(VideoSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.video.src.empty")
    );
}

#[test]
fn input_datetime_local_invalid_and_valid_paths() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("datetime-local".to_string()),
                    span: None,
                },
                Attribute {
                    name: "value".to_string(),
                    value: Some("2020-01-01-02T00:00".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
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

    // Cover leap-year and time parsing success paths too.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("datetime-local".to_string()),
                    span: None,
                },
                Attribute {
                    name: "value".to_string(),
                    value: Some("2020-02-29T23:59:59".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(InputDatetimeLocalConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn script_importmap_constraints_cover_scopes_and_url_like_checks() {
    let run = |format: InputFormat, attrs: Vec<Attribute>, text: &str| {
        let src = VecSource::new(
            format,
            vec![
                ParseEvent::StartTag {
                    name: "script".to_string(),
                    attrs,
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: text.to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "script".to_string(),
                    span: None,
                },
            ],
        );
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptImportmapConstraints::default()),
            Config::default(),
        )
        .unwrap()
    };

    // scopes must be an object.
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("importmap".to_string()),
            span: None,
        }],
        r#"{"scopes":1}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.values.object")
    );

    // scopes keys must be valid URL-like specifiers.
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("importmap".to_string()),
            span: None,
        }],
        r#"{"scopes":{"":{}}}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.keys.url")
    );

    // scopes values must be objects.
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("importmap".to_string()),
            span: None,
        }],
        r#"{"scopes":{"https://example.com/":1}}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.values.object")
    );

    // scope specifier-map values must be strings / valid URL-like specifiers.
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("importmap".to_string()),
            span: None,
        }],
        r#"{"scopes":{"https://example.com/":{"x":1}}}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.values.url")
    );

    // XHTML attribute matching: early return when `src` is present.
    let report = run(
        InputFormat::Xhtml,
        vec![
            Attribute {
                name: "type".to_string(),
                value: Some("importmap".to_string()),
                span: None,
            },
            Attribute {
                name: "src".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ],
        r#"{"imports":{}}"#,
    );
    assert!(report.messages.is_empty());
}

#[test]
fn script_importmap_constraints_ignore_uninterested_events() {
    struct Sink(Vec<html_inspector_core::Message>);
    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }

    let mut ctx = html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
    let mut sink = Sink(Vec::new());
    let mut rule = ScriptImportmapConstraints::default();
    rule.on_event(
        &ParseEvent::Comment {
            text: "x".to_string(),
            span: None,
        },
        &mut ctx,
        &mut sink,
    );
    assert!(sink.0.is_empty());
    html_inspector_core::MessageSink::push(
        &mut sink,
        html_inspector_core::Message::new(
            "test.dummy",
            html_inspector_core::Severity::Info,
            html_inspector_core::Category::Html,
            "x".to_string(),
            None,
        ),
    );
    assert_eq!(sink.0.len(), 1);
}

#[test]
fn script_speculationrules_constraints_cover_predicate_validation_edges() {
    let run = |format: InputFormat, attrs: Vec<Attribute>, text: &str| {
        let src = VecSource::new(
            format,
            vec![
                ParseEvent::StartTag {
                    name: "script".to_string(),
                    attrs,
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: text.to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "script".to_string(),
                    span: None,
                },
            ],
        );
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
            Config::default(),
        )
        .unwrap()
    };

    // Unknown sources are rejected.
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"source":"unknown","urls":["/a"]}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    // Predicate children must be objects (and/or arrays must contain objects).
    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"where":{"and":[1]}}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"where":{"or":{}}}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"where":{"or":[1]}}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"where":{"or":[{"href_matches":""}]}}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    let report = run(
        InputFormat::Html,
        vec![Attribute {
            name: "type".to_string(),
            value: Some("speculationrules".to_string()),
            span: None,
        }],
        r#"{"prefetch":[{"where":{"selector_matches":1}}]}"#,
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    // XHTML attribute matching: early return when `src` is present.
    let report = run(
        InputFormat::Xhtml,
        vec![
            Attribute {
                name: "type".to_string(),
                value: Some("speculationrules".to_string()),
                span: None,
            },
            Attribute {
                name: "src".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ],
        r#"{"prefetch":[{"urls":["/a"]}]}"#,
    );
    assert!(report.messages.is_empty());

    // Cover the `_ => {}` match branch by calling the rule directly.
    struct Sink(Vec<html_inspector_core::Message>);
    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }
    let mut ctx = html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
    let mut sink = Sink(Vec::new());
    let mut rule = ScriptSpeculationrulesConstraints::default();
    rule.on_event(
        &ParseEvent::Comment {
            text: "x".to_string(),
            span: None,
        },
        &mut ctx,
        &mut sink,
    );
    assert!(sink.0.is_empty());
    html_inspector_core::MessageSink::push(
        &mut sink,
        html_inspector_core::Message::new(
            "test.dummy",
            html_inspector_core::Severity::Info,
            html_inspector_core::Category::Html,
            "x".to_string(),
            None,
        ),
    );
    assert_eq!(sink.0.len(), 1);
}

#[test]
fn link_constraints_cover_xhtml_rdfa_as_and_imagesrcset_width_descriptor() {
    // XHTML RDFa attributes count as sufficient context for allowing link without href/imagesrcset.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "about".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(LinkConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.link.href.required")
    );

    // XHTML `as` attribute without preload/modulepreload triggers the error (and covers XHTML attr matching).
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("/x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "as".to_string(),
                    value: Some("script".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(LinkConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.as.requires_preload")
    );

    // Width descriptors in imagesrcset require imagesizes.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("/x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "rel".to_string(),
                    value: Some("preload".to_string()),
                    span: None,
                },
                Attribute {
                    name: "as".to_string(),
                    value: Some("image".to_string()),
                    span: None,
                },
                Attribute {
                    name: "imagesrcset".to_string(),
                    value: Some(", a 1w".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(LinkConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.link.imagesrcset.width_descriptor_requires_imagesizes"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn unnecessary_role_searchbox_on_input_type_search_emits_warning() {
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
                    name: "role".to_string(),
                    value: Some("searchbox".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UnnecessaryRoleWarnings::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.role.unnecessary"
            && m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "The “searchbox” role is unnecessary for an “input” element that has no “list” attribute and whose type is “search”."
    }));
}

#[test]
fn svg_title_does_not_trigger_html_title_empty_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "title".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "title".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "title".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "title".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "svg".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(TitleConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(!report.messages.iter().any(|m| m.code == "html.title.empty"));
}

#[test]
fn svg_style_does_not_trigger_html_style_not_allowed_here_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "title".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "title".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "defs".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(StyleConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.style.not_allowed_here")
    );
}

#[test]
fn html_style_inside_svg_desc_is_allowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "title".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "title".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "desc".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(StyleConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.style.not_allowed_here")
    );
}

#[test]
fn svg_font_does_not_trigger_html_obsolete_font_error() {
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
                name: "font".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    let mut report = report;
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.element.font.obsolete")
    );
}

#[test]
fn svg_xmlns_prefix_other_than_xlink_is_disallowed() {
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
                name: "g".to_string(),
                attrs: vec![Attribute {
                    name: "xmlns:bd".to_string(),
                    value: Some("http://example.org/ExampleBusinessData".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::svg_xmlns_constraints::SvgXmlnsConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.xmlns.prefix.disallowed"
            && m.severity == html_inspector_core::Severity::Warning
            && m.message == "Attribute “xmlns:bd” not allowed here."
    }));
}

#[test]
fn svg_xmlns_xlink_must_match_expected_namespace() {
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
                name: "g".to_string(),
                attrs: vec![Attribute {
                    name: "xmlns:xlink".to_string(),
                    value: Some("http://example.net/bar".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::svg_xmlns_constraints::SvgXmlnsConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.xmlns.xlink.bad_value"
            && m.severity == html_inspector_core::Severity::Error
            && m.message
                == "Bad value “http://example.net/bar” for the attribute “xmlns:link” (only “http://www.w3.org/1999/xlink” permitted here)."
    }));
}

#[test]
fn svg_xmlns_default_must_match_svg_namespace() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "svg".to_string(),
            attrs: vec![Attribute {
                name: "xmlns".to_string(),
                value: Some("http://www.example.org/notsvg".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(super::svg_xmlns_constraints::SvgXmlnsConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.xmlns.default.bad_value"
            && m.severity == html_inspector_core::Severity::Error
            && m.message
                == "Bad value “http://www.example.org/notsvg” for the attribute “xmlns” (only “http://www.w3.org/2000/svg” permitted here)."
    }));
}

#[test]
fn svg_xml_id_attribute_is_disallowed() {
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
                name: "rect".to_string(),
                attrs: vec![Attribute {
                    name: "xml:id".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::svg_suite_constraints::SvgSuiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.attr.xml_id.disallowed"
            && m.severity == html_inspector_core::Severity::Warning
            && m.message == "Attribute “xml:id” not allowed on element “rect” at this point."
    }));
}

#[test]
fn svg_font_requires_missing_glyph_child() {
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
                name: "font".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "font".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::svg_suite_constraints::SvgSuiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.element.font.missing_missing_glyph"
            && m.severity == html_inspector_core::Severity::Warning
            && m.message
                == "Element “font” is missing a required instance of child element “missing-glyph”."
    }));
}

#[test]
fn svg_a_must_not_be_nested_in_another_svg_a() {
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
                name: "a".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::svg_suite_constraints::SvgSuiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.svg.a.nested_in_a"
            && m.severity == html_inspector_core::Severity::Error
            && m.message
                == "The SVG element “a” must not appear as a descendant of another SVG element “a”."
    }));
}

#[test]
fn duplicate_id_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
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
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.id.duplicate")
    );
}

#[test]
fn duplicate_id_is_ignored_inside_template() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
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
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn duplicate_id_in_template_does_not_affect_outside_ids() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
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
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.code == "html.id.duplicate")
            .count(),
        1
    );
}

#[test]
fn duplicate_id_xhtml_requires_exact_id_attribute_name() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "ID".to_string(),
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
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.id.duplicate")
    );
}

#[test]
fn img_alt_required_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgAltRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.alt.missing")
    );
}

#[test]
fn img_role_none_disallowed_with_non_empty_alt() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "alt".to_string(),
                    value: Some("Has alt".to_string()),
                    span: None,
                },
                Attribute {
                    name: "role".to_string(),
                    value: Some("none".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgRoleConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.img.role.invalid_for_non_empty_alt"
            && m.severity == html_inspector_core::Severity::Error
    }));
}

#[test]
fn img_role_invalid_token_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "alt".to_string(),
                    value: Some("".to_string()),
                    span: None,
                },
                Attribute {
                    name: "role".to_string(),
                    value: Some("invalid".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgRoleConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.role.invalid")
    );
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.img.role.alt_empty")
    );
}

#[test]
fn img_role_with_empty_alt_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "alt".to_string(),
                    value: Some("".to_string()),
                    span: None,
                },
                Attribute {
                    name: "role".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgRoleConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.role.alt_empty")
    );
}

#[test]
fn img_border_attribute_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "border".to_string(),
                value: Some("0".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgObsoleteAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.img.border.obsolete" && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn img_sizes_auto_requires_loading_lazy() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "sizes".to_string(),
                value: Some("auto".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSizesAutoRequiresLazyLoading::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes_auto.requires_loading_lazy")
    );
}

#[test]
fn img_sizes_without_srcset_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "src".to_string(),
                    value: Some("image.jpg".to_string()),
                    span: None,
                },
                Attribute {
                    name: "sizes".to_string(),
                    value: Some("100vw".to_string()),
                    span: None,
                },
                Attribute {
                    name: "alt".to_string(),
                    value: Some("Image".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSizesConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes.requires_srcset")
    );
}

#[test]
fn img_sizes_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "src".to_string(),
                    value: Some("default.jpg".to_string()),
                    span: None,
                },
                Attribute {
                    name: "srcset".to_string(),
                    value: Some("small.jpg 320w, large.jpg 800w".to_string()),
                    span: None,
                },
                Attribute {
                    name: "sizes".to_string(),
                    value: Some("(min-width:) 800px, 320px".to_string()),
                    span: None,
                },
                Attribute {
                    name: "alt".to_string(),
                    value: Some("test".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSizesConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes.invalid")
    );
}

#[test]
fn img_src_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "src".to_string(),
                    value: Some("#\\".to_string()),
                    span: None,
                },
                Attribute {
                    name: "alt".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.src.invalid")
    );
}

#[test]
fn img_srcset_width_descriptor_requires_sizes() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "srcset".to_string(),
                    value: Some("x 320w, y 640w".to_string()),
                    span: None,
                },
                Attribute {
                    name: "alt".to_string(),
                    value: Some("Image".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSrcsetSizesRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.img.srcset.width_descriptor_requires_sizes"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn img_srcset_width_descriptor_without_sizes_allowed_with_lazy_and_dimensions() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "srcset".to_string(),
                    value: Some("x 100w, y 200w".to_string()),
                    span: None,
                },
                Attribute {
                    name: "loading".to_string(),
                    value: Some("lazy".to_string()),
                    span: None,
                },
                Attribute {
                    name: "width".to_string(),
                    value: Some("100".to_string()),
                    span: None,
                },
                Attribute {
                    name: "height".to_string(),
                    value: Some("100".to_string()),
                    span: None,
                },
                Attribute {
                    name: "alt".to_string(),
                    value: Some("".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSrcsetSizesRequired::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.img.srcset.width_descriptor_requires_sizes")
    );
}

#[test]
fn img_usemap_hash_only_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "usemap".to_string(),
                value: Some("#".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgUsemapConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.usemap.bad_value")
    );
}

#[test]
fn img_usemap_missing_map_name_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "usemap".to_string(),
                value: Some("#nonexistent".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgUsemapConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.usemap.missing_map_name")
    );
}

#[test]
fn autocomplete_unknown_field_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "autocomplete".to_string(),
                value: Some("qwerty".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AutocompleteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.autocomplete.invalid")
    );
}

#[test]
fn autocomplete_on_hidden_input_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("hidden".to_string()),
                    span: None,
                },
                Attribute {
                    name: "autocomplete".to_string(),
                    value: Some("on".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AutocompleteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.autocomplete.on_off_disallowed_on_hidden")
    );
}

#[test]
fn img_ismap_requires_anchor_href_ancestor() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "ismap".to_string(),
                value: None,
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgIsmapAnchorAncestor::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.ismap.requires_anchor_href")
    );
}

#[test]
fn area_requires_map_ancestor_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "area".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AreaRequiresMapAncestor::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.area.map_ancestor.missing")
    );
}

#[test]
fn a_href_allows_empty_port_after_colon() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("http://f:/c".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AHrefConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.invalid")
    );
}

#[test]
fn a_href_allows_legacy_file_drive_authority_pipe() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("file://C|/foo/bar".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AHrefConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.invalid")
    );
}

#[test]
fn a_href_rejects_non_ascii_in_userinfo() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("http://💩:foo@example.com".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AHrefConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.invalid")
    );
}

#[test]
fn a_name_emits_obsolete_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "name".to_string(),
                value: Some("anchor".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.name.obsolete"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn a_ping_without_href_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "ping".to_string(),
                value: Some("https://example.com/track".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.ping.requires_href")
    );
}

#[test]
fn a_transparent_model_disallows_flow_when_a_in_phrasing_context() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "a".to_string(),
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
        ],
    );
    let rules = RuleSet::new().push(ATransparentContentModel::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.transparent.disallowed_child_in_phrasing")
    );
}

#[test]
fn a_with_href_inside_button_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "button".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![Attribute {
                    name: "href".to_string(),
                    value: Some("/link".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AHrefButtonDescendant::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.button_descendant")
    );
}

#[test]
fn a_with_href_inside_button_inside_template_is_ignored() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "button".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![Attribute {
                    name: "href".to_string(),
                    value: Some("/link".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AHrefButtonDescendant::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.button_descendant")
    );
}

#[test]
fn address_must_not_be_descendant_of_address() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "address".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "address".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AddressConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.address.descendant.disallowed")
    );
}

#[test]
fn obsolete_acronym_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "acronym".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.acronym.obsolete")
    );
}

#[test]
fn obsolete_frameset_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "frameset".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.frameset.obsolete")
    );
}

#[test]
fn obsolete_keygen_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "keygen".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.keygen.obsolete")
    );
}

#[test]
fn obsolete_marquee_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "marquee".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.marquee.obsolete")
    );
}

#[test]
fn implied_p_end_tag_with_open_elements_emits_parse_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "abbr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "ul".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(ImpliedPEndTag::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.p.end_tag_implied_open_elements")
    );
}

#[test]
fn area_download_without_coords_or_shape_does_not_require_href() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "area".to_string(),
            attrs: vec![Attribute {
                name: "download".to_string(),
                value: None,
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ADownloadConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.area.href.required_for_download")
    );
}

#[test]
fn area_download_with_coords_requires_href() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "area".to_string(),
            attrs: vec![
                Attribute {
                    name: "download".to_string(),
                    value: Some("file.pdf".to_string()),
                    span: None,
                },
                Attribute {
                    name: "coords".to_string(),
                    value: Some("0,0,1,1".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ADownloadConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.area.href.required_for_download")
    );
}

#[test]
fn area_shape_default_disallows_coords() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "area".to_string(),
            attrs: vec![
                Attribute {
                    name: "shape".to_string(),
                    value: Some("default".to_string()),
                    span: None,
                },
                Attribute {
                    name: "coords".to_string(),
                    value: Some("1,2".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AreaCoordsConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.area.coords.disallowed_for_default")
    );
}

#[test]
fn article_without_h2_h6_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "article".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "article".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(ArticleHeadingWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.article.lacks_heading"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn empty_heading_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "h1".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "h1".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(EmptyHeadingWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.heading.empty"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn empty_heading_warns_for_unclosed_nested_headings() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "h1".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "h2".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "h1".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(EmptyHeadingWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.code == "html.heading.empty")
            .count(),
        1
    );
}

#[test]
fn empty_heading_matches_tag_name_case_insensitively_in_html() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "H1".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "h1".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(EmptyHeadingWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.heading.empty")
    );
}

#[test]
fn empty_heading_matches_tag_name_case_sensitively_in_xhtml() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "H1".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "h1".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(EmptyHeadingWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.heading.empty")
    );
}

#[test]
fn non_empty_heading_does_not_warn() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "h1".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "h1".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(EmptyHeadingWarning::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.heading.empty")
    );
}

#[test]
fn audio_transparent_model_disallows_flow_when_in_phrasing_context() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "audio".to_string(),
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
        ],
    );
    let rules = RuleSet::new().push(AudioTransparentContentModel::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.audio.transparent.disallowed_child_in_phrasing")
    );
}

#[test]
fn audio_src_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "audio".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AudioSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.audio.src.invalid")
    );
}

#[test]
fn audio_src_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "audio".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AudioSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.audio.src.empty")
    );
}

#[test]
fn audio_src_attr_name_matching_is_case_insensitive_in_html() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "audio".to_string(),
            attrs: vec![Attribute {
                name: "SRC".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AudioSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.audio.src.empty")
    );
}

#[test]
fn audio_src_attr_name_matching_is_case_sensitive_in_xhtml() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "audio".to_string(),
            attrs: vec![Attribute {
                name: "SRC".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AudioSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn autonomous_custom_element_must_not_have_is_attribute() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "my-element".to_string(),
            attrs: vec![Attribute {
                name: "is".to_string(),
                value: Some("other-element".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IsAttributeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.is_attribute.autonomous_custom_element_forbidden")
    );
}

#[test]
fn base_href_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "base".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("http://".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(BaseHrefConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.base.href.invalid")
    );
}

#[test]
fn base_missing_href_and_target_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "base".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(BaseElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.base.missing_href_or_target")
    );
}

#[test]
fn base_after_link_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "link".to_string(),
                attrs: vec![Attribute {
                    name: "href".to_string(),
                    value: Some("foo".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "base".to_string(),
                attrs: vec![Attribute {
                    name: "href".to_string(),
                    value: Some("bar".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(BaseElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.base.must_come_before_link_or_script")
    );
}

#[test]
fn blockquote_cite_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "blockquote".to_string(),
            attrs: vec![Attribute {
                name: "cite".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(BlockquoteCiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.blockquote.cite.invalid")
    );
}

#[test]
fn void_element_end_tag_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::EndTag {
            name: "br".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(VoidElementEndTag::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.void_element.end_tag")
    );
}

#[test]
fn void_element_end_tag_emits_error_on_parse_error_event() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::ParseError {
            code: "html.void_element.end_tag".to_string(),
            message: "End tag “br”.".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.void_element.end_tag")
    );
}

#[test]
fn void_element_end_tag_detected_with_html5ever_backend() {
    let src = HtmlEventSource::from_str("t", InputFormat::Html, "</br>").unwrap();
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.void_element.end_tag")
    );
}

#[test]
fn foreign_object_start_tag_span_matches_input_bytes_with_html5ever_backend() {
    let html = "<svg><foreignObject></foreignObject></svg>";
    let expected_start = html.find("<foreignObject").unwrap();

    let mut src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
    let mut got_start = None;
    while let Some(ev) = src.next_event().unwrap() {
        if let ParseEvent::StartTag {
            name,
            span: Some(span),
            ..
        } = ev
        {
            if name.eq_ignore_ascii_case("foreignobject") {
                got_start = Some(span.byte_start);
                break;
            }
        }
    }

    assert_eq!(got_start, Some(expected_start));
}

#[test]
fn button_formaction_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![Attribute {
                name: "formaction".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ButtonFormactionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.button.formaction.invalid")
    );
}

#[test]
fn button_formaction_matches_case_insensitively_in_html() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "BUTTON".to_string(),
            attrs: vec![Attribute {
                name: "FORMACTION".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ButtonFormactionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.button.formaction.invalid")
    );
}

#[test]
fn button_formaction_is_case_sensitive_in_xhtml() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "BUTTON".to_string(),
            attrs: vec![Attribute {
                name: "FORMACTION".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ButtonFormactionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn button_formaction_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![Attribute {
                name: "formaction".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ButtonFormactionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.button.formaction.empty")
    );
}

#[test]
fn canvas_transparent_model_disallows_flow_when_in_phrasing_context() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "canvas".to_string(),
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
        ],
    );
    let rules = RuleSet::new().push(CanvasTransparentContentModel::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.canvas.transparent.disallowed_child_in_phrasing")
    );
}

#[test]
fn colgroup_span_exceeds_max_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "colgroup".to_string(),
                attrs: vec![Attribute {
                    name: "span".to_string(),
                    value: Some("10001".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::table_constraints::TableConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.colgroup.span.max")
    );
}

#[test]
fn colgroup_with_span_and_col_child_warns_span_ignored() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "colgroup".to_string(),
                attrs: vec![Attribute {
                    name: "span".to_string(),
                    value: Some("3".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "col".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(super::table_constraints::TableConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.colgroup.span.ignored")
    );
}

#[test]
fn del_cite_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "cite".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelCiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.del.cite.invalid")
    );
}

#[test]
fn ins_cite_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "ins".to_string(),
            attrs: vec![Attribute {
                name: "cite".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InsCiteConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.ins.cite.invalid")
    );
}

#[test]
fn del_datetime_out_of_range_year_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some("0004-02-29".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.del.datetime.warn" && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn del_datetime_invalid_date_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some("2014-02-29".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.invalid")
    );
}

#[test]
fn del_datetime_accepts_global_datetime_with_z() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some("2011-11-12T14:54Z".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn del_datetime_timezone_minutes_15_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some("2011-11-12T00:00:00+08:15".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.warn"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn del_datetime_leading_whitespace_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "del".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some(" 2002-09-29".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DelDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.del.datetime.invalid")
    );
}

#[test]
fn details_disallows_multiple_summary_children() {
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
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "summary".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "summary".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DetailsSummaryConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.details.multiple_summary")
    );
}

#[test]
fn details_requires_summary_as_first_child() {
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
                name: "p".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "details".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DetailsSummaryConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.details.missing_summary")
    );
}

#[test]
fn dialog_disallows_dt_as_child() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dialog".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DialogConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dialog.disallowed_dt_dd")
    );
}

#[test]
fn dt_disallows_article_descendants() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "article".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DtDescendantConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dt.disallowed_descendant")
    );
}

#[test]
fn dl_requires_dd_even_if_dd_only_in_template() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dd".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dl".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlStructureConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.missing_dd")
    );
}

#[test]
fn dl_disallows_text_nodes() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlStructureConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.text.disallowed")
    );
}

#[test]
fn dl_disallows_nested_dl_at_dl_level() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlStructureConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.child.dl.disallowed")
    );
}

#[test]
fn p_end_tag_after_implicit_close_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "p".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PEndTagScope::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.p.end_tag_without_scope")
    );
}

#[test]
fn p_end_tag_after_implicit_close_by_ul_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "ul".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "p".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PEndTagScope::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.p.end_tag_without_scope")
    );
}

#[test]
fn ol_start_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "ol".to_string(),
            attrs: vec![Attribute {
                name: "start".to_string(),
                value: Some("abc".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(OlStartConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.ol.start.invalid")
    );
}

#[test]
fn option_without_label_must_not_be_empty() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "option".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(OptionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label")
    );
}

#[test]
fn option_label_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "option".to_string(),
            attrs: vec![Attribute {
                name: "label".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(OptionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.option.label.empty")
    );
}

#[test]
fn picture_source_media_all_with_following_source_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "source".to_string(),
                attrs: vec![
                    Attribute {
                        name: "srcset".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "media".to_string(),
                        value: Some("all".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "source".to_string(),
                attrs: vec![Attribute {
                    name: "srcset".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "picture".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureSourceMediaAllConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.source.media_all.disallowed")
    );
}

#[test]
fn picture_source_media_all_without_following_source_is_allowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "source".to_string(),
                attrs: vec![
                    Attribute {
                        name: "srcset".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "media".to_string(),
                        value: Some("all".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "picture".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureSourceMediaAllConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn picture_source_media_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "source".to_string(),
                attrs: vec![
                    Attribute {
                        name: "srcset".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "media".to_string(),
                        value: None,
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureSourceSelectionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.source.media.empty")
    );
}

#[test]
fn picture_source_always_matching_without_media_or_type_emits_error_when_followed_by_img_srcset() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "source".to_string(),
                attrs: vec![Attribute {
                    name: "srcset".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![Attribute {
                    name: "srcset".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureSourceSelectionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.source.always_matching.disallowed")
    );
}

#[test]
fn non_void_self_closing_syntax_emits_parse_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "picture".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(NonVoidSelfClosingSyntax::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.self_closing.non_void")
    );
}

#[test]
fn non_void_self_closing_syntax_emits_parse_error_on_parse_error_event() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::ParseError {
            code: "html.parse.self_closing.non_void".to_string(),
            message: "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.".to_string(),
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.self_closing.non_void")
    );
}

#[test]
fn non_void_self_closing_syntax_detected_with_html5ever_backend() {
    let src = HtmlEventSource::from_str("t", InputFormat::Html, "<div/>").unwrap();
    let rules = RuleSet::new().push(TokenizerParseErrors::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.self_closing.non_void")
    );
}

#[test]
fn picture_unclosed_at_eof_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "picture".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(PictureUnclosedEndOfFile::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.parse.eof.open_elements")
    );
}

#[test]
fn audio_srcset_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "audio".to_string(),
            attrs: vec![Attribute {
                name: "srcset".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.audio.srcset.disallowed")
    );
}

#[test]
fn img_missing_src_and_srcset_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.src_or_srcset.missing")
    );
}

#[test]
fn img_srcset_without_descriptor_with_sizes_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "srcset".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "sizes".to_string(),
                    value: Some("50vw".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgSrcsetConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.srcset.invalid")
    );
}

#[test]
fn img_type_attribute_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "type".to_string(),
                value: Some("image/gif".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.type.disallowed")
    );
}

#[test]
fn input_srcset_attribute_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "srcset".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputAttributeDisallowedByType::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.srcset.disallowed")
    );
}

#[test]
fn picture_disallows_br_child() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "br".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "picture".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureContentModelConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.message
            .contains("Element “br” not allowed as child of “picture”")
    }));
}

#[test]
fn picture_disallows_non_whitespace_text_child() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureContentModelConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.text.disallowed")
    );
}

#[test]
fn picture_disallows_picture_child() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureContentModelConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.child.picture.disallowed")
    );
}

#[test]
fn link_srcset_attribute_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "srcset".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.srcset.disallowed")
    );
}

#[test]
fn object_srcset_attribute_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "object".to_string(),
            attrs: vec![Attribute {
                name: "srcset".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.object.srcset.disallowed")
    );
}

#[test]
fn picture_disallowed_in_hgroup_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "hgroup".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.parent.hgroup.disallowed")
    );
}

#[test]
fn picture_disallowed_in_rp_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "rp".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.parent.rp.disallowed")
    );
}

#[test]
fn picture_in_noscript_in_head_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "noscript".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.in_noscript_in_head.disallowed")
    );
}

#[test]
fn picture_disallowed_attribute_align_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "picture".to_string(),
            attrs: vec![Attribute {
                name: "align".to_string(),
                value: Some("left".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(PictureAttributeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.attr.align.disallowed")
    );
}

#[test]
fn picture_disallowed_in_ul_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ul".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "picture".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(PictureParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.picture.parent.ul.disallowed")
    );
}

#[test]
fn param_element_is_obsolete() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "param".to_string(),
            attrs: vec![],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObsoleteElements::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.param.obsolete")
    );
}

#[test]
fn dl_div_child_role_must_be_presentation_or_none() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("group".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDivGroupConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.div.role.disallowed")
    );
}

#[test]
fn dl_div_group_requires_dt_and_dd() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDivGroupConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.div.missing_dd")
    );
}

#[test]
fn dl_div_group_constraints_are_ignored_inside_template() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("group".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dl".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDivGroupConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn dl_div_group_disallows_dt_after_dd() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dd".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDivGroupConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.div.dt.after_dd")
    );
}

#[test]
fn dl_duplicate_dt_name_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "Term".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dt".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "dd".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "Term".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dt".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dl".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDuplicateDtName::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.dl.duplicate_dt_name"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn dl_duplicate_dt_name_emits_warning_inside_template() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "template".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dl".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "Term".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dt".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "dd".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "Term".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dt".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "dl".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "template".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DlDuplicateDtName::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.dl.duplicate_dt_name"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn embed_height_percent_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "embed".to_string(),
            attrs: vec![Attribute {
                name: "height".to_string(),
                value: Some("20%".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EmbedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.embed.height.invalid")
    );
}

#[test]
fn embed_src_empty_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "embed".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EmbedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.embed.src.empty")
    );
}

#[test]
fn embed_src_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "embed".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EmbedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.embed.src.invalid")
    );
}

#[test]
fn embed_width_percent_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "embed".to_string(),
            attrs: vec![Attribute {
                name: "width".to_string(),
                value: Some("20%".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EmbedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.embed.width.invalid")
    );
}

#[test]
fn embed_type_without_slash_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "embed".to_string(),
            attrs: vec![Attribute {
                name: "type".to_string(),
                value: Some("foo".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EmbedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.embed.type.invalid")
    );
}

#[test]
fn figure_with_role_img_and_figcaption_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("img".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureFigcaption::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.figure.role.with_figcaption")
    );
}

#[test]
fn figure_with_role_doc_example_and_figcaption_does_not_emit_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("doc-example".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureFigcaption::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.figure.role.with_figcaption")
    );
}

#[test]
fn figure_with_role_doc_example_case_insensitive_and_figcaption_does_not_emit_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("DOC-EXAMPLE".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureFigcaption::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.figure.role.with_figcaption")
    );
}

#[test]
fn figure_with_role_figure_case_insensitive_and_figcaption_does_not_emit_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("FIGURE".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureFigcaption::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.figure.role.with_figcaption")
    );
}

#[test]
fn figure_table_caption_prefers_figcaption_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "caption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "table".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "figure".to_string(),
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureTableCaptionWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.figure.table_caption.prefers_figcaption"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn footer_disallows_footer_and_header_descendants() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "footer".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "header".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "footer".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FooterConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.footer.descendant.header")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.footer.descendant.footer")
    );
}

#[test]
fn header_disallows_footer_and_header_descendants() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "header".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "footer".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "header".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(HeaderConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.header.descendant.footer")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.header.descendant.header")
    );
}

#[test]
fn form_accept_charset_only_allows_utf8() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "form".to_string(),
            attrs: vec![Attribute {
                name: "accept-charset".to_string(),
                value: Some("iso-8859-1".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(FormActionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.form.accept_charset.only_utf8")
    );
}

#[test]
fn form_action_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "form".to_string(),
            attrs: vec![Attribute {
                name: "action".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(FormActionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.form.action.invalid")
    );
}

#[test]
fn form_action_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "form".to_string(),
            attrs: vec![Attribute {
                name: "action".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(FormActionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.form.action.empty")
    );
}

#[test]
fn base_in_body_emits_error_after_body_content() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "base".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(BaseInBody::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.base.not_in_body")
    );
}

#[test]
fn bdo_dir_missing_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "bdo".to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(BdoDir::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.bdo.dir.missing")
    );
}

#[test]
fn figure_multiple_figcaption_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "figure".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "figcaption".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "figcaption".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FigureFigcaption::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.figure.figcaption.multiple")
    );
}

#[test]
fn input_type_constraints_emits_errors() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "type".to_string(),
                        value: Some("button".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "value".to_string(),
                        value: Some("".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "type".to_string(),
                        value: Some("file".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "value".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(InputTypeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.button.value.nonempty")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.file.value.disallowed")
    );
}

#[test]
fn microdata_itemid_requires_itemtype() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![
                Attribute {
                    name: "itemscope".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "itemid".to_string(),
                    value: Some("http://example.com/item".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MicrodataConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.microdata.itemid.requires_itemscope_itemtype")
    );
}

#[test]
fn dl_child_content_rejects_p() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dl".to_string(),
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
        ],
    );
    let rules = RuleSet::new().push(DlChildContent::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.dl.child.invalid")
    );
}

#[test]
fn label_multiple_controls_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LabelControlCount::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.label.multiple_controls")
    );
}

#[test]
fn label_aria_hidden_with_labelable_descendant_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![Attribute {
                    name: "aria-hidden".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LabelForConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.label.aria_hidden.with_labelable_descendant"
            && m.severity == html_inspector_core::Severity::Error
    }));
}

#[test]
fn label_for_requires_descendant_input_id_match() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![Attribute {
                    name: "for".to_string(),
                    value: Some("a".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LabelForConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.label.for.descendant_input_id_mismatch")
    );
}

#[test]
fn label_for_must_reference_non_hidden_form_control() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "label".to_string(),
            attrs: vec![Attribute {
                name: "for".to_string(),
                value: Some("missing".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LabelForConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.label.for.must_reference_non_hidden_control")
    );
}

#[test]
fn role_button_disallows_input_descendant() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LabelForConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.role_button.descendant_input")
    );
}

#[test]
fn li_outside_list_parent_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LiParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.li.parent.disallowed")
    );
}

#[test]
fn li_value_disallowed_outside_ol() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ul".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "value".to_string(),
                    value: Some("5".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LiValueConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.li.value.disallowed")
    );
}

#[test]
fn li_value_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ol".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "li".to_string(),
                attrs: vec![Attribute {
                    name: "value".to_string(),
                    value: Some("abc".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(LiValueConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.li.value.invalid")
    );
}

#[test]
fn link_missing_href_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "rel".to_string(),
                value: Some("icon".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.href.required")
    );
}

#[test]
fn link_as_requires_preload_or_modulepreload() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "rel".to_string(),
                    value: Some("dns-prefetch".to_string()),
                    span: None,
                },
                Attribute {
                    name: "as".to_string(),
                    value: Some("script".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.as.requires_preload")
    );
}

#[test]
fn link_alternate_stylesheet_requires_title() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "rel".to_string(),
                    value: Some("alternate stylesheet".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.alternate_stylesheet.title.required")
    );
}

#[test]
fn link_blocking_requires_rel_stylesheet_only() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![
                Attribute {
                    name: "href".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "rel".to_string(),
                    value: Some("preload".to_string()),
                    span: None,
                },
                Attribute {
                    name: "blocking".to_string(),
                    value: Some("render".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.blocking.requires_stylesheet")
    );
}

#[test]
fn link_href_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkHrefConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.href.invalid")
    );
}

#[test]
fn main_disallowed_descendant_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "article".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "main".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MainConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.main.disallowed_descendant")
    );
}

#[test]
fn multiple_visible_main_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "main".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "main".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "main".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MainConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.main.visible.multiple")
    );
}

#[test]
fn map_id_name_mismatch_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "map".to_string(),
            attrs: vec![
                Attribute {
                    name: "id".to_string(),
                    value: Some("foo".to_string()),
                    span: None,
                },
                Attribute {
                    name: "name".to_string(),
                    value: Some("bar".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MapConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.map.id_name.mismatch")
    );
}

#[test]
fn math_role_attribute_emits_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "math".to_string(),
            attrs: vec![Attribute {
                name: "role".to_string(),
                value: Some("math".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MathRoleWarning::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.math.role.unnecessary"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn meter_value_exceeds_max_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meter".to_string(),
            attrs: vec![
                Attribute {
                    name: "value".to_string(),
                    value: Some("10".to_string()),
                    span: None,
                },
                Attribute {
                    name: "max".to_string(),
                    value: Some("5".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MeterConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meter.value_le_max")
    );
}

#[test]
fn script_importmap_src_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                },
                Attribute {
                    name: "src".to_string(),
                    value: Some("map.json".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ScriptConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.src.disallowed")
    );
}

#[test]
fn script_type_matching_is_ascii_case_insensitive() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![
                    Attribute {
                        name: "type".to_string(),
                        value: Some("MODULE".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "defer".to_string(),
                        value: None,
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![
                    Attribute {
                        name: "type".to_string(),
                        value: Some("ImportMap".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "src".to_string(),
                        value: Some("map.json".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("TEXT/JAVASCRIPT".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.module.defer.disallowed")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.src.disallowed")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.type.unnecessary")
    );
}

#[test]
fn script_charset_requires_src() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "charset".to_string(),
                value: Some("utf-8".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.charset.requires_src")
    );
}

#[test]
fn script_charset_must_be_utf8_when_src_present() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "src".to_string(),
                    value: Some("x.js".to_string()),
                    span: None,
                },
                Attribute {
                    name: "charset".to_string(),
                    value: Some("iso-8859-1".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.charset.utf8_only")
    );
}

#[test]
fn script_language_is_obsolete() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "language".to_string(),
                value: Some("JavaScript".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.language.obsolete")
    );
}

#[test]
fn script_language_javascript_with_non_javascript_type_emits_mismatch_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "language".to_string(),
                    value: Some("JavaScript".to_string()),
                    span: None,
                },
                Attribute {
                    name: "type".to_string(),
                    value: Some("text/vbscript".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.language.javascript.type_mismatch")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.language.obsolete")
    );
}

#[test]
fn script_type_is_unnecessary_for_javascript_mime_types() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "type".to_string(),
                value: Some("text/javascript1.5".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.type.unnecessary")
    );
}

#[test]
fn script_module_defer_is_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("module".to_string()),
                    span: None,
                },
                Attribute {
                    name: "defer".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.module.defer.disallowed")
    );
}

#[test]
fn script_module_with_nomodule_is_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("module".to_string()),
                    span: None,
                },
                Attribute {
                    name: "nomodule".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.module.nomodule.disallowed")
    );
}

#[test]
fn script_speculationrules_disallows_async() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                },
                Attribute {
                    name: "async".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.async.disallowed")
    );
}

#[test]
fn inline_classic_script_disallows_async_and_defer() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "async".to_string(),
                value: None,
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.inline_classic.async.disallowed")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "defer".to_string(),
                value: None,
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.inline.defer.disallowed")
    );
}

#[test]
fn script_data_block_disallows_async_and_src() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("text/plain".to_string()),
                    span: None,
                },
                Attribute {
                    name: "async".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.datablock.async.disallowed")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("text/plain".to_string()),
                    span: None,
                },
                Attribute {
                    name: "src".to_string(),
                    value: Some("x.txt".to_string()),
                    span: None,
                },
            ],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.datablock.src.disallowed")
    );
}

#[test]
fn select_without_multiple_rejects_two_selected() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![Attribute {
                    name: "selected".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![Attribute {
                    name: "selected".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(SelectConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.select.selected.multiple_without_multiple")
    );
}

#[test]
fn track_empty_label_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "track".to_string(),
            attrs: vec![Attribute {
                name: "label".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TrackConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.track.label.non_empty")
    );
}

#[test]
fn accesskey_duplicate_tokens_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "accesskey".to_string(),
                value: Some("a b ぬ c ぬ".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AccesskeyConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.accesskey.duplicate")
    );
}

#[test]
fn accesskey_multi_character_token_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "accesskey".to_string(),
                value: Some("a b ほげ".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AccesskeyConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.accesskey.duplicate")
    );
}

#[test]
fn autofocus_multiple_in_dialog_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "dialog".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "button".to_string(),
                attrs: vec![Attribute {
                    name: "autofocus".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "button".to_string(),
                attrs: vec![Attribute {
                    name: "autofocus".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(AutofocusConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.autofocus.multiple_in_scoping_root")
    );
}

#[test]
fn commandfor_missing_id_emits_error_on_finish() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![Attribute {
                name: "commandfor".to_string(),
                value: Some("missing".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(CommandforConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.commandfor.idref.missing")
    );
}

#[test]
fn data_attribute_invalid_suffix_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "data-z:foo".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(DataAttributeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.data_attribute.not_xml_serializable")
    );
}

#[test]
fn div_name_attribute_disallowed_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "name".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ElementSpecificAttributes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.div.name.disallowed")
    );
}

#[test]
fn enterkeyhint_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "enterkeyhint".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EnterkeyhintConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.enterkeyhint.value.invalid")
    );
}

#[test]
fn enterkeyhint_missing_value_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "enterkeyhint".to_string(),
                value: None,
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(EnterkeyhintConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.enterkeyhint.value.invalid")
    );
}

#[test]
fn form_attribute_must_refer_to_form() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("notaform".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![Attribute {
                    name: "form".to_string(),
                    value: Some("notaform".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(FormAttributeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.form_attribute.not_form")
    );
}

#[test]
fn headingoffset_out_of_range_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "article".to_string(),
            attrs: vec![Attribute {
                name: "headingoffset".to_string(),
                value: Some("9".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(HeadingoffsetConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.headingoffset.range")
    );
}

#[test]
fn is_attribute_requires_hyphen_and_lowercase() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "button".to_string(),
            attrs: vec![Attribute {
                name: "is".to_string(),
                value: Some("fancybutton".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IsAttributeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.is_attribute.invalid")
    );
}

#[test]
fn popover_invalid_value_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "popover".to_string(),
                value: Some("invalid".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(PopoverConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.popover.value.invalid")
    );
}

#[test]
fn rel_typo_emits_info() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "rel".to_string(),
                value: Some("alternat".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(RelTypoConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.severity == html_inspector_core::Severity::Info)
    );
}

#[test]
fn spellcheck_bad_value_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "spellcheck".to_string(),
                value: Some("badvalue".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(SpellcheckConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.spellcheck.invalid")
    );
}

#[test]
fn spellcheck_missing_value_is_ok() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "p".to_string(),
            attrs: vec![Attribute {
                name: "spellcheck".to_string(),
                value: None,
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(SpellcheckConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn target_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "target".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TargetBrowsingContextConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.target.empty")
    );
}

#[test]
fn target_reserved_names_allowed_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "target".to_string(),
                value: Some("_BLANK".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TargetBrowsingContextConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn target_underscore_names_reject_unknown_token_case_insensitively() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "target".to_string(),
                value: Some("_NOPE".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TargetBrowsingContextConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.target.underscore_disallowed")
    );
}

#[test]
fn input_color_invalid_value_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("color".to_string()),
                    span: None,
                },
                Attribute {
                    name: "value".to_string(),
                    value: Some("#gggggg".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputColorConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.color.value.invalid")
    );
}

#[test]
fn input_date_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("date".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("2024-02-30".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputDateConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.date.invalid")
    );
}

#[test]
fn input_datetime_local_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("datetime-local".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("2024-13-01T12:00".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputDatetimeLocalConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.datetime_local.invalid")
    );
}

#[test]
fn time_datetime_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "time".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some("badvalue".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TimeDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.time.datetime.invalid")
    );
}

#[test]
fn time_datetime_accepts_common_valid_formats() {
    let values = [
        "2024-12-31",
        "12:30",
        "2024-12-31T12:30:00",
        "2024-12-31T12:30:00Z",
        "2024-12-31T12:30:00+09:00",
        "P1D",
        "2024-W52",
        "2024-12",
    ];
    let events = values
        .into_iter()
        .map(|v| ParseEvent::StartTag {
            name: "time".to_string(),
            attrs: vec![Attribute {
                name: "datetime".to_string(),
                value: Some(v.to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        })
        .collect::<Vec<_>>();

    let src = VecSource::new(InputFormat::Html, events);
    let rules = RuleSet::new().push(TimeDatetimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert_eq!(report.messages.len(), 0);
}

#[test]
fn id_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "id".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IdDatatypeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| m.code == "html.id.invalid"));
}

#[test]
fn size_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "size".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputSizeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.size.invalid")
    );
}

#[test]
fn size_zero_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "size".to_string(),
                value: Some("0".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputSizeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.size.invalid")
    );
}

#[test]
fn size_allowed_for_mixed_case_text_type() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("TeXt".to_string()),
                    span: None,
                },
                Attribute {
                    name: "size".to_string(),
                    value: Some("1".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputSizeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn size_disallowed_for_number_type() {
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
                    name: "size".to_string(),
                    value: Some("1".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputSizeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.size.disallowed_for_type")
    );
}

#[test]
fn input_readonly_disallowed_for_button_type() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                },
                Attribute {
                    name: "readonly".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputAttributeAllowedTypes::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.readonly.disallowed_for_type")
    );
}

#[test]
fn checkbox_role_button_requires_aria_pressed() {
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
                    name: "role".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputCheckboxRoleButtonAriaPressed::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.input.checkbox.role_button.missing_aria_pressed"
            && m.severity == html_inspector_core::Severity::Error
    }));
}

#[test]
fn input_image_requires_alt() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("image".to_string()),
                    span: None,
                },
                Attribute {
                    name: "src".to_string(),
                    value: Some("button.png".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputTypeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.image.alt.missing")
    );
}

#[test]
fn input_list_disallowed_for_checkbox_type() {
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
                    name: "list".to_string(),
                    value: Some("test".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputListConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.list.disallowed_for_type")
    );
}

#[test]
fn input_list_must_refer_to_datalist() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "datalist".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("foo".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "type".to_string(),
                        value: Some("text".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "list".to_string(),
                        value: Some("bar".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(InputListConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.list.must_refer_to_datalist")
    );
}

#[test]
fn input_name_isindex_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "name".to_string(),
                value: Some("isindex".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputNameConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.name.isindex.disallowed")
    );
}

#[test]
fn input_number_invalid_value_emits_error() {
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
                    name: "value".to_string(),
                    value: Some("abc".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputNumberConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.number.value.invalid")
    );
}

#[test]
fn input_number_multiple_disallowed() {
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
                    name: "multiple".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputNumberConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.number.multiple.disallowed")
    );
}

#[test]
fn input_range_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("range".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("abc".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputRangeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.range.min.invalid")
    );
}

#[test]
fn input_tel_disallows_max_min_step() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("tel".to_string()),
                    span: None,
                },
                Attribute {
                    name: "max".to_string(),
                    value: Some("100".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("0".to_string()),
                    span: None,
                },
                Attribute {
                    name: "step".to_string(),
                    value: Some("1".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputAttributeDisallowedByType::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.max.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.min.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.step.disallowed_for_type")
    );
}

#[test]
fn input_text_disallows_accept_checked_src_width_height() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("text".to_string()),
                    span: None,
                },
                Attribute {
                    name: "accept".to_string(),
                    value: Some("text/plain".to_string()),
                    span: None,
                },
                Attribute {
                    name: "checked".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "src".to_string(),
                    value: Some("image.png".to_string()),
                    span: None,
                },
                Attribute {
                    name: "width".to_string(),
                    value: Some("200".to_string()),
                    span: None,
                },
                Attribute {
                    name: "height".to_string(),
                    value: Some("50".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputAttributeDisallowedByType::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.accept.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.checked.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.src.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.width.disallowed_for_type")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.height.disallowed_for_type")
    );
}

#[test]
fn input_hidden_disallows_aria_and_placeholder() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("hidden".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
                Attribute {
                    name: "placeholder".to_string(),
                    value: Some("y".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputAttributeDisallowedByType::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.hidden.aria.disallowed")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.hidden.placeholder.disallowed")
    );
}

#[test]
fn input_image_formaction_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("image".to_string()),
                    span: None,
                },
                Attribute {
                    name: "formaction".to_string(),
                    value: Some("#\\".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputFormactionConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.formaction.invalid")
    );
}

#[test]
fn input_image_src_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("image".to_string()),
                    span: None,
                },
                Attribute {
                    name: "src".to_string(),
                    value: Some("#\\".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputImageSrcConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.image.src.invalid")
    );
}

#[test]
fn input_url_value_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("url".to_string()),
                    span: None,
                },
                Attribute {
                    name: "value".to_string(),
                    value: Some("#\\".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputUrlValueConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.url.value.invalid")
    );
}

#[test]
fn step_non_number_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "step".to_string(),
                value: Some("abc".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputStepConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.step.invalid")
    );
}

#[test]
fn progress_max_negative_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "progress".to_string(),
            attrs: vec![Attribute {
                name: "max".to_string(),
                value: Some("-5".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ProgressConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.progress.max.positive")
    );
}

#[test]
fn textarea_rows_zero_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "textarea".to_string(),
            attrs: vec![Attribute {
                name: "rows".to_string(),
                value: Some("0".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(TextareaConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.textarea.rows.positive")
    );
}

#[test]
fn script_integrity_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "script".to_string(),
            attrs: vec![Attribute {
                name: "integrity".to_string(),
                value: Some("md5-abc123".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ScriptIntegrityConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.integrity.invalid")
    );
}

#[test]
fn object_type_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "object".to_string(),
            attrs: vec![Attribute {
                name: "type".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObjectMimetypeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.object.type.invalid")
    );
}

#[test]
fn mime_type_syntax_matches_vnu_suite_examples() {
    let valid = [
        "text/html",
        "TEXT/HTML",
        "text/html;charset=utf-8",
        "text/html ;charset=utf-8",
        "text/html; charset=utf-8",
        "text/html  ;  charset=utf-8",
        "text/html;charset=\"utf-8\"",
        "text/html;charset=\"\\utf-8\"",
        "text/html;charset=\"u\\t\\f\\-\\8\"",
        "application/auth-policy+xml",
        "application/vnd.apple.installer+xml",
    ];
    for v in valid {
        assert!(is_mime_type(v), "expected valid MIME type: {v:?}");
    }

    let invalid = [
        "text/html ",
        " TEXT/HTML",
        "text/html;charset=",
        "text/html ;charset",
        "text/html;",
        "text/html; ",
        "text/html ;",
        "text/html;charset=\"utf-8",
        "text/html;charset=\"u\\",
        "application",
        "application/",
    ];
    for v in invalid {
        assert!(!is_mime_type(v), "expected invalid MIME type: {v:?}");
    }
}

#[test]
fn link_type_invalid_mime_type_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "type".to_string(),
                value: Some("text/html ".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(LinkMimetypeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.link.type.invalid")
    );
}

#[test]
fn object_data_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "object".to_string(),
            attrs: vec![Attribute {
                name: "data".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ObjectDataConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.object.data.invalid")
    );
}

#[test]
fn input_month_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("month".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("2024-00".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputMonthConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.month.invalid")
    );
}

#[test]
fn input_name_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "name".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputNameConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.name.empty")
    );
}

#[test]
fn meta_refresh_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("refresh".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaRefreshConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.refresh.empty")
    );
}

#[test]
fn meta_refresh_valid_accepts_url_after_semicolon_with_whitespace() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("refresh".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("0; URL=http://example.com/".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaRefreshConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.is_empty());
}

#[test]
fn meta_refresh_missing_whitespace_after_semicolon_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("refresh".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("0;url=http://example.com/".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaRefreshConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.refresh.invalid")
    );
}

#[test]
fn meta_refresh_quoted_url_is_invalid() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "http-equiv".to_string(),
                    value: Some("refresh".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("0; url='http://example.com/'".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaRefreshConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.refresh.invalid")
    );
}

#[test]
fn meta_charset_with_content_disallowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "meta".to_string(),
            attrs: vec![
                Attribute {
                    name: "charset".to_string(),
                    value: Some("utf-8".to_string()),
                    span: None,
                },
                Attribute {
                    name: "content".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MetaElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.charset.content.disallowed")
    );
}

#[test]
fn meta_charset_and_content_type_disallowed() {
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
                    name: "http-equiv".to_string(),
                    value: Some("content-type".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MetaElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.charset_and_content_type.disallowed")
    );
}

#[test]
fn meta_csp_invalid_directive_warns_and_invalid_keyword_errors() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-security-policy".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("default-src 'self'; invalid-directive 'none'".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-security-policy".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("default-src 'invalid-keyword'".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MetaElementConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.csp.invalid"
                && m.severity == html_inspector_core::Severity::Warning)
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.meta.csp.invalid"
                && m.severity == html_inspector_core::Severity::Error)
    );
}

#[test]
fn iframe_sandbox_duplicate_token_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![Attribute {
                name: "sandbox".to_string(),
                value: Some("allow-scripts allow-scripts".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IFrameSandboxConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.sandbox.duplicate_token"
                && m.severity == html_inspector_core::Severity::Info)
    );
}

#[test]
fn iframe_sandbox_invalid_token_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![Attribute {
                name: "sandbox".to_string(),
                value: Some("allow-scripts not-a-token".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IFrameSandboxConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.sandbox.invalid_token"
                && m.severity == html_inspector_core::Severity::Error)
    );
}

#[test]
fn iframe_sandbox_scripts_and_same_origin_warns() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![Attribute {
                name: "sandbox".to_string(),
                value: Some("allow-scripts allow-same-origin".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IFrameSandboxConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.sandbox.scripts_and_same_origin"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn iframe_disallows_allowpaymentrequest_and_seamless() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![
                Attribute {
                    name: "allowpaymentrequest".to_string(),
                    value: None,
                    span: None,
                },
                Attribute {
                    name: "seamless".to_string(),
                    value: None,
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IframeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.allowpaymentrequest.disallowed")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.seamless.disallowed")
    );
}

#[test]
fn iframe_src_invalid_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("#\\".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IframeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.src.invalid"
                && m.severity == html_inspector_core::Severity::Warning)
    );
}

#[test]
fn iframe_src_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "iframe".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(IframeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.src.empty")
    );
}

#[test]
fn placeholder_with_linebreak_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![Attribute {
                name: "placeholder".to_string(),
                value: Some("line1\nline2".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(PlaceholderConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.placeholder.linebreak")
    );
}

#[test]
fn input_time_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("time".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("25:00".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputTimeConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.time.invalid")
    );
}

#[test]
fn input_week_invalid_min_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "input".to_string(),
            attrs: vec![
                Attribute {
                    name: "type".to_string(),
                    value: Some("week".to_string()),
                    span: None,
                },
                Attribute {
                    name: "min".to_string(),
                    value: Some("2024-W54".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(InputWeekConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.input.week.invalid")
    );
}

#[test]
fn link_href_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "link".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(UrlConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| m.code == "html.url.empty"));
}

#[test]
fn a_download_without_href_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "download".to_string(),
                value: None,
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ADownloadConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.required_for_download")
    );
}

#[test]
fn a_href_invalid_host_empty_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("http://".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AHrefConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.a.href.invalid")
    );
}

#[test]
fn microdata_itemtype_requires_itemscope() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "itemtype".to_string(),
                value: Some("http://schema.org/Thing".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(MicrodataConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.microdata.itemtype.requires_itemscope")
    );
}

#[test]
fn attribute_href_on_div_emits_not_allowed_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "href".to_string(),
                value: Some("mailto:x@example.com".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(AttributeNotAllowedConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    let msg = report
        .messages
        .iter()
        .find(|m| m.code == "html.attr.href.not_allowed")
        .expect("expected not-allowed message");
    assert_eq!(msg.severity, html_inspector_core::Severity::Warning);
}

#[test]
fn p_as_child_of_strong_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "strong".to_string(),
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
        ],
    );
    let rules = RuleSet::new().push(PDisallowedParentConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.p.parent_strong")
    );
}

#[test]
fn mathml_html_tag_in_math_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "math".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MathmlConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "html.mathml.html_start_tag_in_foreign_namespace"
            && m.message == "HTML start tag “div” in a foreign namespace context."
    }));
}

#[test]
fn mathml_annotation_xml_allows_html_children() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "math".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "annotation-xml".to_string(),
                attrs: vec![Attribute {
                    name: "encoding".to_string(),
                    value: Some("application/xhtml+xml".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(MathmlConstraints::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.mathml.html_start_tag_in_foreign_namespace")
    );
}

#[test]
fn html_tag_name_matching_is_case_insensitive_but_xhtml_is_case_sensitive() {
    // HTML: <MAP><AREA> should satisfy the map-ancestor check.
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "MAP".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "AREA".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AreaRequiresMapAncestor::default()),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.area.map_ancestor.missing")
    );

    // XHTML: <MAP><area> does not satisfy the map-ancestor check (case-sensitive).
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "MAP".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "area".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AreaRequiresMapAncestor::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.area.map_ancestor.missing")
    );
}

#[test]
fn html_attribute_name_matching_is_case_insensitive_but_xhtml_is_case_sensitive() {
    // HTML: ALT should be treated as alt.
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "ALT".to_string(),
                value: Some("ok".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ImgAltRequired::default()),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.img.alt.missing")
    );

    // XHTML: ALT should not be treated as alt.
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "ALT".to_string(),
                value: Some("ok".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ImgAltRequired::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.alt.missing")
    );
}

#[test]
fn duplicate_id_trims_whitespace_and_ignores_empty_ids() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("  ".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some(" x ".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
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
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.id.duplicate")
    );
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.code == "html.id.duplicate")
            .count(),
        1
    );
}

#[test]
fn img_alt_required_allows_aria_label_as_accessible_name() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![
                Attribute {
                    name: "aria-label".to_string(),
                    value: Some("label".to_string()),
                    span: None,
                },
                Attribute {
                    name: "aria-busy".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                },
            ],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgAltRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.img.aria.accessible_name.missing")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.alt.missing")
    );
}

#[test]
fn duplicate_id_is_case_sensitive() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "id".to_string(),
                    value: Some("X".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DuplicateId::default());
    let mut report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.id.duplicate")
    );
}

#[test]
fn duplicate_id_xhtml_attribute_name_is_case_sensitive() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "ID".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "ID".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
        ],
    );
    let rules = RuleSet::new().push(DuplicateId::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.id.duplicate")
    );
}

#[test]
fn img_alt_required_emits_accessible_name_missing_when_aria_attrs_present_without_name() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "img".to_string(),
            attrs: vec![Attribute {
                name: "aria-busy".to_string(),
                value: Some("true".to_string()),
                span: None,
            }],
            self_closing: true,
            span: None,
        }],
    );
    let rules = RuleSet::new().push(ImgAltRequired::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.aria.accessible_name.missing")
    );
}

#[test]
fn style_in_body_emits_not_allowed_here_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(StyleConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.style.not_allowed_here")
    );
}

#[test]
fn style_scoped_is_disallowed_in_head() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![Attribute {
                    name: "scoped".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(StyleConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.style.scoped.disallowed")
    );
}

#[test]
fn style_type_text_css_emits_unnecessary_warning() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text/css".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(StyleConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.style.type.unnecessary")
    );
}

#[test]
fn style_type_other_than_text_css_emits_error() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("text/css; charset=utf-8".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(StyleConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.style.type.text_css_only")
    );
}

#[test]
fn style_inside_svg_desc_integration_point_is_allowed_in_body() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "desc".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "style".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(StyleConstraints::default()),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code == "html.style.not_allowed_here")
    );
}

#[test]
fn script_importmap_requires_valid_json() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.json.invalid")
    );
}

#[test]
fn script_importmap_requires_top_level_object() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "[]".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.top_level.object")
    );
}

#[test]
fn script_importmap_disallows_unknown_top_level_properties() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"foo\": {}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.top_level.properties")
    );
}

#[test]
fn script_importmap_imports_requires_object() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"imports\": []}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.imports.object")
    );
}

#[test]
fn script_importmap_imports_keys_must_be_non_empty() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"imports\": {\"\": \"./x\"}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.imports.keys.non_empty")
    );
}

#[test]
fn script_importmap_imports_values_must_be_strings() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"imports\": {\"x\": 1}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.imports.values.string")
    );
}

#[test]
fn script_importmap_imports_requires_trailing_slash_match() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"imports\": {\"x/\": \"y\"}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.imports.slash_match")
    );
}

#[test]
fn script_importmap_scopes_keys_must_be_valid_urls() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"scopes\": {\"http://exa mple/\": {\"x\": \"./y\"}}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.keys.url")
    );
}

#[test]
fn script_importmap_scopes_values_must_be_url_strings() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("importmap".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"scopes\": {\"https://example.com/\": {\"x\": \"http://exa mple/\"}}}"
                    .to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptImportmapConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.importmap.scopes.values.url")
    );
}

#[test]
fn obj_is_not_allowed_in_p() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "obj".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ObjElementConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.element.obj.not_allowed_in_p")
    );
}

#[test]
fn xhtml_xml_stylesheet_pi_must_appear_before_elements() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "href=\"a.css\"".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "xhtml.xml_stylesheet.after_element")
    );
}

#[test]
fn xhtml_xml_stylesheet_pi_requires_href() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::ProcessingInstruction {
            target: "xml-stylesheet".to_string(),
            data: "type=\"text/css\"".to_string(),
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "xhtml.xml_stylesheet.missing_href")
    );
}

#[test]
fn xhtml_xml_stylesheet_pi_alternate_yes_requires_non_empty_title() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::ProcessingInstruction {
            target: "xml-stylesheet".to_string(),
            data: "href=\"a.css\" alternate=\"yes\"".to_string(),
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| { m.code == "xhtml.xml_stylesheet.alternate_yes.requires_title" })
    );
}

#[test]
fn xhtml_xml_stylesheet_pi_charset_utf8_emits_warning() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::ProcessingInstruction {
            target: "xml-stylesheet".to_string(),
            data: "href=\"a.css\" charset=\"utf-8\"".to_string(),
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.iter().any(|m| {
        m.code == "xhtml.xml_stylesheet.charset.ignored_warning"
            && m.severity == html_inspector_core::Severity::Warning
    }));
}

#[test]
fn xhtml_xml_stylesheet_pi_type_with_invalid_mime_emits_error() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![ParseEvent::ProcessingInstruction {
            target: "xml-stylesheet".to_string(),
            data: "href=\"a.css\" type=\"text\"".to_string(),
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "xhtml.xml_stylesheet.type.bad_value")
    );
}

#[test]
fn speculationrules_requires_valid_json_and_object() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.json.invalid")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "[]".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.json.object")
    );
}

#[test]
fn speculationrules_requires_prefetch_or_prerender_and_disallows_unknown_props() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.top_level.missing")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"prefetch\": [], \"extra\": 1}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.top_level.properties")
    );
}

#[test]
fn speculationrules_prefetch_must_be_array_of_rules_with_valid_urls() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"prefetch\": {}}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"prefetch\": [{\"source\": \"list\", \"urls\": [\"\"]}]}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid")
    );
}

#[test]
fn speculationrules_document_rules_validate_where_predicates() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "script".to_string(),
                attrs: vec![Attribute {
                    name: "type".to_string(),
                    value: Some("speculationrules".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "{\"prerender\": [{\"where\": {}}]}".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "script".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prerender.invalid")
    );
}

#[test]
fn html_attr_src_is_not_allowed_on_a() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "a".to_string(),
            attrs: vec![Attribute {
                name: "src".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AttributeNotAllowedConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.src.not_allowed_on_a")
    );
}

#[test]
fn html_attr_xml_base_is_not_allowed_on_html() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "html".to_string(),
            attrs: vec![Attribute {
                name: "xml:base".to_string(),
                value: Some("x".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AttributeNotAllowedConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.xml_base.not_allowed")
    );
}

#[test]
fn html_attr_prefix_value_is_validated() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "prefix".to_string(),
                value: Some("foo: https://example.com/".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AttributeNotAllowedConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(report.messages.is_empty());

    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "prefix".to_string(),
                value: Some("foo https://example.com/".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AttributeNotAllowedConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.prefix.bad_value")
    );
}

#[test]
fn html_attr_xmlns_prefix_is_not_allowed() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "xmlns:foo".to_string(),
                value: Some("bar".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(AttributeNotAllowedConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.xmlns.not_allowed")
    );
}

#[test]
fn q_cite_invalid_value_emits_message() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![ParseEvent::StartTag {
            name: "q".to_string(),
            attrs: vec![Attribute {
                name: "cite".to_string(),
                value: Some("http://exa mple/".to_string()),
                span: None,
            }],
            self_closing: false,
            span: None,
        }],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(QCiteConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.q.cite.invalid")
    );
}

#[test]
fn ruby_requires_rt_and_rp_children() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ruby".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "ruby".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(RubyConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.ruby.missing.rp_rt")
    );

    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ruby".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "rp".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "rp".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "ruby".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(RubyConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.ruby.missing.rt")
    );
}

#[test]
fn ruby_rt_with_non_whitespace_content_satisfies_constraint() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "ruby".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "rt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "rt".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "ruby".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(RubyConstraints::default()),
        Config::default(),
    )
    .unwrap();
    let mut report = report;
    report.messages.push(html_inspector_core::Message::new(
        "test.dummy",
        html_inspector_core::Severity::Info,
        html_inspector_core::Category::Html,
        "x".to_string(),
        None,
    ));
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.code.starts_with("html.ruby.missing"))
    );
}

#[test]
fn img_sizes_requires_srcset_when_sizes_present() {
    let src = HtmlEventSource::from_str("t", InputFormat::Html, "<img sizes=\"100vw\">").unwrap();
    let rules = RuleSet::new().push(ImgSizesConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes.requires_srcset")
    );
}

#[test]
fn img_sizes_auto_requires_lazy_loading() {
    let src = HtmlEventSource::from_str(
        "t",
        InputFormat::Html,
        "<img srcset=\"a 1x\" sizes=\"auto, 100vw\">",
    )
    .unwrap();
    let rules = RuleSet::new().push(ImgSizesConstraints::default());
    let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes.auto_requires_lazy")
    );
}

#[test]
fn img_sizes_source_size_list_validation_covers_edge_cases() {
    for (sizes, expected_ok) in [
        ("100vw", true),
        ("auto", true),
        ("auto, 100vw", true),
        ("(min-width: 600px) 50vw, 100vw", true),
        ("(min-width: 600px) calc(50vw - 10px), 100vw", true),
        ("", false),
        ("/* unclosed", false),
        (", 100vw", false),
        ("(min-width:) 100vw", false),
        ("(min-width: 600px) 50vw, 100vw, 50vw", false), // default must be last
        ("100%", false),
        ("10 unknown", false),
        ("calc(10px", false),
        ("default", false),
        ("inherit", false),
        ("-1px", false),
    ] {
        let html = format!("<img srcset=\"a 1x\" sizes=\"{sizes}\" loading=\"lazy\">");
        let src = HtmlEventSource::from_str("t", InputFormat::Html, &html).unwrap();
        let rules = RuleSet::new().push(ImgSizesConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        let ok = !report
            .messages
            .iter()
            .any(|m| m.code == "html.img.sizes.invalid");
        assert_eq!(ok, expected_ok, "sizes={sizes:?}");
    }
}

#[test]
fn table_constraints_reports_headers_missing_th() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![Attribute {
                    name: "headers".to_string(),
                    value: Some("missing".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "table".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::table_constraints::TableConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.headers.missing_th")
    );
}

#[test]
fn table_constraints_detects_overlaps_missing_starts_and_rowgroup_span() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "tbody".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            // Row 1: cell in col0, and a rowspan cell in col1.
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![Attribute {
                    name: "colspan".to_string(),
                    value: Some("2".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            // Row 2: empty row (no starting cells).
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "tbody".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "table".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::table_constraints::TableConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.row.no_cells")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.column.no_starting_cell"
                || m.code == "html.table.columns.no_starting_cells")
    );
}

#[test]
fn table_constraints_enforces_limits_and_col_markup_mismatch() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "col".to_string(),
                attrs: vec![Attribute {
                    name: "span".to_string(),
                    value: Some("1001".to_string()),
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![
                    Attribute {
                        name: "colspan".to_string(),
                        value: Some("0".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "rowspan".to_string(),
                        value: Some("70000".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![Attribute {
                    name: "colspan".to_string(),
                    value: Some("1001".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "table".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::table_constraints::TableConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.table.col.span.max",
        "html.table.cell.colspan.zero",
        "html.table.cell.rowspan.max",
        "html.table.row.width.less_than_col_markup",
        "html.table.cell.colspan.max",
        "html.table.row.width.exceeds_col_markup",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn table_constraints_xhtml_disallows_col_direct_child_of_table() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "col".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::table_constraints::TableConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.table.col.parent_table")
    );
}

#[test]
fn table_constraints_reports_overlap_and_spans_past_row_group() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "table".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "tbody".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            // Row 1: col0 cell, and a rowspan cell in col1 that spans beyond the group.
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![Attribute {
                    name: "rowspan".to_string(),
                    value: Some("3".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            // Row 2: a spanning cell overlaps the carried rowspan column.
            ParseEvent::StartTag {
                name: "tr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "td".to_string(),
                attrs: vec![Attribute {
                    name: "colspan".to_string(),
                    value: Some("2".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "tr".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "tbody".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "table".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::table_constraints::TableConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.table.cell.overlapped",
        "html.table.cell.spans_past_row_group",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn link_constraints_cover_common_attribute_interactions() {
    let html = r##"
<!doctype html>
<html>
  <head>
    <link rel="stylesheet">
    <link href="a">
    <link href="a" imagesizes="100vw">
    <link href="a" rel="preload">
    <link href="a" rel="modulepreload" as="script">
    <link href="a" as="script">
    <link href="a" rel="alternate stylesheet">
    <link href="a" rel="preload" blocking="render" as="script">
    <link href="a" rel="preload" disabled as="script">
    <link href="a" rel="icon" color="#fff">
    <link href="a" rel="icon" integrity="x">
    <link href="a" rel="preload" as="image" imagesrcset="a 100w">
    <link href="a" imagesrcset="a 100w" as="script">
    <link href="a" sizes="32x32" rel="preload" as="image">
  </head>
  <body>
    <link href="a" rel="icon">
    <link href=\"a\" property=\"x\">
    <link href=\"a\" itemprop=\"x\" rel=\"stylesheet\">
  </body>
</html>
"##;
    let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(LinkConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.link.href.required",
        "html.link.imagesizes.requires_imagesrcset",
        "html.link.as.requires_preload",
        "html.link.preload.requires_as",
        "html.link.integrity.requires_stylesheet_or_preload",
        "html.link.alternate_stylesheet.title.required",
        "html.link.blocking.requires_stylesheet",
        "html.link.disabled.requires_stylesheet",
        "html.link.color.requires_mask_icon",
        "html.link.imagesrcset.requires_preload",
        "html.link.imagesrcset.requires_as_image",
        "html.link.imagesrcset.width_descriptor_requires_imagesizes",
        "html.link.sizes.requires_icon_rel",
        "html.link.in_body.disallowed_rel",
        "html.link.missing_rel_or_itemprop_or_property",
        "html.link.itemprop.rel.disallowed",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn svg_suite_constraints_cover_more_branches() {
    let html = r#"
<!doctype html>
<svg xml:id="x" xml:base="y" baseProfile="basic" contentScriptType="x" externalResourcesRequired="true">
  <clipPath x="0"></clipPath>
  <filter filterPrimitiveUnits="userSpaceOnUse"></filter>
  <g marker="x"></g>
  <rect stop-color="red"></rect>
  <feConvolveMatrix></feConvolveMatrix>
  <feComponentTransfer><feFuncR></feFuncR></feComponentTransfer>
  <a><tspan></tspan><a></a></a>
  <use><use></use></use>
  <defs><stop></stop></defs>
  <path d="M 20 100 H 40#90"></path>
  <font></font>
  <font horiz-adv-x="1"></font>
</svg>
"#;
    let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(super::svg_suite_constraints::SvgSuiteConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.svg.attr.xml_id.disallowed",
        "html.svg.attr.xml_base.disallowed",
        "html.svg.attr.x.disallowed_on_clippath",
        "html.svg.attr.filterprimitiveunits.disallowed",
        "html.svg.attr.marker.disallowed_on_g",
        "html.svg.attr.stop_color.disallowed_on_rect",
        "html.svg.attr.contentscripttype.disallowed_on_svg",
        "html.svg.attr.externalresourcesrequired.disallowed_on_svg",
        "html.svg.element.feconvolvematrix.missing_order",
        "html.svg.child.fefuncr.disallowed_in_fecomponenttransfer",
        "html.svg.child.tspan.disallowed_in_a",
        "html.svg.a.nested_in_a",
        "html.svg.use.nested_in_use",
        "html.svg.stop.child_of_defs",
        "html.svg.attr.d.bad_value",
        "html.svg.element.font.missing_horiz_adv_x",
        "html.svg.element.font.missing_missing_glyph",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn mathml_constraints_validate_math_attributes_and_missing_children() {
    let html = r#"
<!doctype html>
<math display="bad" overflow="nope">
  <mfrac><mi></mi></mfrac>
</math>
"#;
    let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MathmlConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.mathml.math.display.bad_value",
        "html.mathml.math.overflow.bad_value",
        "html.mathml.missing_children",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn mathml_constraints_rejects_html_start_tags_in_math_namespace() {
    let src =
        HtmlEventSource::from_str("t", InputFormat::Html, "<math><div></div></math>").unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MathmlConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.mathml.html_start_tag_in_foreign_namespace")
    );
}

#[test]
fn mathml_constraints_reports_unknown_mathml_elements_in_math_namespace() {
    let src =
        HtmlEventSource::from_str("t", InputFormat::Html, "<math><munknown></munknown></math>")
            .unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MathmlConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.mathml.unknown_element")
    );
}

#[test]
fn mathml_constraints_enforces_context_sensitive_suite_rules() {
    let src = HtmlEventSource::from_str(
        "t",
        InputFormat::Html,
        "<math><annotation></annotation><mprescripts></mprescripts><mtr></mtr><mtd></mtd></math>",
    )
    .unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MathmlConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.mathml.annotation.outside_semantics",
        "html.mathml.mprescripts.outside_mmultiscripts",
        "html.mathml.mtr.parent",
        "html.mathml.mtd.parent",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn mathml_constraints_reports_mathml_elements_outside_math() {
    let src = HtmlEventSource::from_str("t", InputFormat::Html, "<mrow></mrow>").unwrap();
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MathmlConstraints::default()),
        Config::default(),
    )
    .unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.code == "html.mathml.element.outside_math")
    );
}

#[test]
fn obsolete_elements_reports_expected_messages() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![Attribute {
                    name: "profile".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "basefont".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "head".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "body".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "acronym".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "applet".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "basefont".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "big".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "blink".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "center".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "font".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "dir".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "frameset".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "noframes".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "keygen".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "marquee".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "menu".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "menuitem".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "menu".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "menuitem".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "nobr".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "param".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "strike".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "tt".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(ObsoleteElements::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.attribute.profile.obsolete",
        "html.element.basefont.disallowed_in_head",
        "html.element.acronym.obsolete",
        "html.element.applet.obsolete",
        "html.element.basefont.obsolete",
        "html.element.big.obsolete",
        "html.element.center.obsolete",
        "html.element.font.obsolete",
        "html.element.dir.obsolete",
        "html.element.frameset.obsolete",
        "html.element.noframes.obsolete",
        "html.element.keygen.obsolete",
        "html.element.marquee.obsolete",
        "html.element.nobr.obsolete",
        "html.element.param.obsolete",
        "html.element.strike.obsolete",
        "html.element.tt.obsolete",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn script_constraints_cover_more_combinations() {
    let cases = [
        (
            "<script charset=\"utf-16\"></script>",
            "html.script.charset.requires_src",
        ),
        (
            "<script charset=\"utf-16\" src=\"a\"></script>",
            "html.script.charset.utf8_only",
        ),
        (
            "<script language=\"JavaScript\" type=\"application/json\"></script>",
            "html.script.language.javascript.type_mismatch",
        ),
        (
            "<script type=\"text/javascript\"></script>",
            "html.script.type.unnecessary",
        ),
        (
            "<script type=\"module\" defer></script>",
            "html.script.module.defer.disallowed",
        ),
        (
            "<script type=\"module\" nomodule></script>",
            "html.script.module.nomodule.disallowed",
        ),
        (
            "<script type=\"module\" integrity=\"x\">x</script>",
            "html.script.module.inline.integrity.disallowed",
        ),
        (
            "<script type=\"importmap\" async></script>",
            "html.script.importmap.async.disallowed",
        ),
        (
            "<script type=\"importmap\" src=\"a\"></script>",
            "html.script.importmap.src.disallowed",
        ),
        (
            "<script type=\"speculationrules\" crossorigin></script>",
            "html.script.speculationrules.crossorigin.disallowed",
        ),
        (
            "<script type=\"speculationrules\" src=\"a\"></script>",
            "html.script.speculationrules.src.disallowed",
        ),
        (
            "<script async>var x</script>",
            "html.script.inline_classic.async.disallowed",
        ),
        (
            "<script defer>var x</script>",
            "html.script.inline.defer.disallowed",
        ),
        (
            "<script blocking=\"render\">var x</script>",
            "html.script.inline_classic.blocking.disallowed",
        ),
        (
            "<script type=\"application/json\" async></script>",
            "html.script.datablock.async.disallowed",
        ),
        (
            "<script type=\"application/json\" src=\"a\"></script>",
            "html.script.datablock.src.disallowed",
        ),
        (
            "<script type=\"application/json\" nomodule></script>",
            "html.script.datablock.nomodule.disallowed",
        ),
    ];

    for (html, code) in cases {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code} for {html}"
        );
    }
}

#[test]
fn speculationrules_script_json_validation_exercises_more_paths() {
    let cases = [
        (
            "<script type=\"speculationrules\"></script>",
            "html.script.speculationrules.json.invalid",
        ),
        (
            "<script type=\"speculationrules\">[]</script>",
            "html.script.speculationrules.json.object",
        ),
        (
            "<script type=\"speculationrules\">{\"x\":1}</script>",
            "html.script.speculationrules.top_level.missing",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[],\"extra\":1}</script>",
            "html.script.speculationrules.top_level.properties",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":{}}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[1]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":1}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"nope\"}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[]}]} </script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"\"]}]} </script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"where\":{}}]} </script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"href_matches\":0}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"and\":[]}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"and\":[{\"or\":[]}]} } ]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"href_matches\":\"\"}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"selector_matches\":true}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"urls\":[\"a\"]}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"source\":\"list\",\"where\":{}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
    ];

    for (html, code) in cases {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code} for {html}"
        );
    }
}

#[test]
fn script_constraints_cover_more_disallowed_attributes() {
    let cases = [
        (
            "<script type=\"importmap\" crossorigin></script>",
            "html.script.importmap.crossorigin.disallowed",
        ),
        (
            "<script type=\"importmap\" defer></script>",
            "html.script.importmap.defer.disallowed",
        ),
        (
            "<script type=\"importmap\" blocking=\"render\"></script>",
            "html.script.importmap.blocking.disallowed",
        ),
        (
            "<script type=\"importmap\" fetchpriority=\"high\"></script>",
            "html.script.importmap.fetchpriority.disallowed",
        ),
        (
            "<script type=\"importmap\" integrity=\"x\"></script>",
            "html.script.importmap.integrity.disallowed",
        ),
        (
            "<script type=\"importmap\" nomodule></script>",
            "html.script.importmap.nomodule.disallowed",
        ),
        (
            "<script type=\"importmap\" referrerpolicy=\"no-referrer\"></script>",
            "html.script.importmap.referrerpolicy.disallowed",
        ),
        (
            "<script type=\"speculationrules\" blocking=\"render\"></script>",
            "html.script.speculationrules.blocking.disallowed",
        ),
        (
            "<script type=\"speculationrules\" defer></script>",
            "html.script.speculationrules.defer.disallowed",
        ),
        (
            "<script type=\"speculationrules\" fetchpriority=\"high\"></script>",
            "html.script.speculationrules.fetchpriority.disallowed",
        ),
        (
            "<script type=\"speculationrules\" integrity=\"x\"></script>",
            "html.script.speculationrules.integrity.disallowed",
        ),
        (
            "<script type=\"speculationrules\" nomodule></script>",
            "html.script.speculationrules.nomodule.disallowed",
        ),
        (
            "<script type=\"speculationrules\" referrerpolicy=\"no-referrer\"></script>",
            "html.script.speculationrules.referrerpolicy.disallowed",
        ),
        (
            "<script type=\"application/json\" integrity=\"x\"></script>",
            "html.script.datablock.integrity.disallowed",
        ),
        (
            "<script type=\"application/json\" fetchpriority=\"high\"></script>",
            "html.script.datablock.fetchpriority.disallowed",
        ),
        (
            "<script type=\"application/json\" crossorigin></script>",
            "html.script.datablock.crossorigin.disallowed",
        ),
        (
            "<script type=\"application/json\" referrerpolicy=\"no-referrer\"></script>",
            "html.script.datablock.referrerpolicy.disallowed",
        ),
        (
            "<script type=\"module\" blocking=\"render\">x</script>",
            "html.script.module.inline.blocking.disallowed",
        ),
        (
            "<script type=\"module\" fetchpriority=\"high\">x</script>",
            "html.script.module.inline.fetchpriority.disallowed",
        ),
        (
            "<script fetchpriority=\"high\">x</script>",
            "html.script.inline_classic.fetchpriority.disallowed",
        ),
        (
            "<script integrity=\"x\">x</script>",
            "html.script.inline_classic.integrity.disallowed",
        ),
    ];

    for (html, code) in cases {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code} for {html}"
        );
    }
}

#[test]
fn speculationrules_script_json_validation_covers_more_rule_shapes() {
    let cases = [
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"a\"],\"eagerness\":1}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"a\"],\"eagerness\":\"fast\"}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"a\"],\"foo\":1}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"a\"],\"where\":{}}]}</script>",
            "html.script.speculationrules.prefetch.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"href_matches\":\"a\",\"selector_matches\":\"b\"}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"not\":1}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
        (
            "<script type=\"speculationrules\">{\"prerender\":[{\"where\":{\"selector_matches\":\"\"}}]}</script>",
            "html.script.speculationrules.prerender.invalid",
        ),
    ];

    for (html, code) in cases {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code} for {html}"
        );
    }
}

#[test]
fn meta_element_constraints_cover_common_document_level_rules() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "charset".to_string(),
                        value: Some("utf-8".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "itemprop".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "name".to_string(),
                        value: Some("y".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            // charset + content-type across the same document.
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-type".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("text/html; charset=utf-8".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-language".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("en".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "name".to_string(),
                        value: Some("description".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("a".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "name".to_string(),
                        value: Some("description".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("b".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "media".to_string(),
                        value: Some("screen".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "name".to_string(),
                        value: Some("not-theme-color".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "name".to_string(),
                        value: Some("viewport".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("width=device-width,user-scalable=no".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("X-UA-Compatible".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("IE=9".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            // CSP: unknown directive => warning; invalid nonce => error.
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-security-policy".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("unknown-src 'self'".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "meta".to_string(),
                attrs: vec![
                    Attribute {
                        name: "http-equiv".to_string(),
                        value: Some("content-security-policy".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "content".to_string(),
                        value: Some("script-src 'nonce-'".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(MetaElementConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.meta.charset.content.disallowed",
        "html.meta.itemprop.disallowed_with_name",
        "html.meta.charset_and_content_type.disallowed",
        "html.meta.http_equiv.content_language.obsolete",
        "html.meta.description.multiple.disallowed",
        "html.meta.media.requires_theme_color",
        "html.meta.viewport.user_scalable_no",
        "html.meta.x_ua_compatible.requires_ie_edge",
        "html.meta.csp.invalid",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn select_constraints_cover_required_placeholder_and_autocomplete() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![Attribute {
                    name: "required".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "select".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![Attribute {
                    name: "required".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![Attribute {
                    name: "value".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::Text {
                text: "Pick".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "option".to_string(),
                span: None,
            },
            ParseEvent::EndTag {
                name: "select".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![Attribute {
                    name: "autocomplete".to_string(),
                    value: Some("section-foo webauthn".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "select".to_string(),
                span: None,
            },
            ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![Attribute {
                    name: "size".to_string(),
                    value: Some("0".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::EndTag {
                name: "select".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(SelectConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.select.required.must_have_option",
        "html.select.required.first_option.placeholder",
        "html.select.autocomplete.webauthn.disallowed",
        "html.select.size.nonzero",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn xhtml_xml_stylesheet_pseudo_attribute_syntax_errors_are_reported() {
    let src = VecSource::new(
        InputFormat::Xhtml,
        vec![
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "href".to_string(),
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "bad=\"x\" href=\"a.css\"".to_string(),
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "href=\"a.css\" href=\"b.css\"".to_string(),
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "alternate=\"maybe\" href=\"a.css\"".to_string(),
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "href=\"a css\"".to_string(),
                span: None,
            },
            ParseEvent::ProcessingInstruction {
                target: "xml-stylesheet".to_string(),
                data: "href=\"a.css\" type=\"text/plain\"".to_string(),
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(XmlStylesheetProcessingInstruction::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "xhtml.xml_stylesheet.pseudo_attribute.syntax",
        "xhtml.xml_stylesheet.alternate.bad_value",
        "xhtml.xml_stylesheet.href.bad_url_space",
        "xhtml.xml_stylesheet.type.unsupported_warning",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn label_for_constraints_cover_descendant_and_association_checks() {
    let src = VecSource::new(
        InputFormat::Html,
        vec![
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "role".to_string(),
                    value: Some("button".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            // Label with a "for" value and labelable descendants triggers descendant checks.
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![
                    Attribute {
                        name: "for".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-hidden".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-label".to_string(),
                        value: Some("lbl".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "role".to_string(),
                        value: Some("button".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("y".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "type".to_string(),
                        value: Some("text".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::EndTag {
                name: "label".to_string(),
                span: None,
            },
            // Controls to satisfy (and violate) label "for" associations at finish time.
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "type".to_string(),
                        value: Some("text".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    Attribute {
                        name: "id".to_string(),
                        value: Some("hidden".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "type".to_string(),
                        value: Some("hidden".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![Attribute {
                    name: "for".to_string(),
                    value: Some("hidden".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            ParseEvent::StartTag {
                name: "label".to_string(),
                attrs: vec![Attribute {
                    name: "for".to_string(),
                    value: Some("missing".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
        ],
    );
    let report = html_inspector_core::validate_events(
        src,
        RuleSet::new().push(LabelForConstraints::default()),
        Config::default(),
    )
    .unwrap();
    for code in [
        "html.role_button.descendant_input",
        "html.label.for.descendant_input_id_mismatch",
        "html.label.aria_hidden.with_labelable_descendant",
        "html.label.role.with_labelable_descendant",
        "html.label.aria_label.associated_with_labelable",
        "html.label.role.associated_with_labelable",
        "html.label.for.must_reference_non_hidden_control",
    ] {
        assert!(
            report.messages.iter().any(|m| m.code == code),
            "missing {code}"
        );
    }
}

#[test]
fn progress_constraints_cover_value_validation_paths() {
    for (attrs, expected_code) in [
        (
            vec![Attribute {
                name: "value".to_string(),
                value: Some("nope".to_string()),
                span: None,
            }],
            "html.progress.value.invalid",
        ),
        (
            vec![Attribute {
                name: "value".to_string(),
                value: Some("-1".to_string()),
                span: None,
            }],
            "html.progress.value.non_negative",
        ),
        (
            vec![
                Attribute {
                    name: "max".to_string(),
                    value: Some("2".to_string()),
                    span: None,
                },
                Attribute {
                    name: "value".to_string(),
                    value: Some("3".to_string()),
                    span: None,
                },
            ],
            "html.progress.value.exceeds_max",
        ),
        (
            vec![Attribute {
                name: "value".to_string(),
                value: Some("2".to_string()),
                span: None,
            }],
            "html.progress.value.exceeds_one",
        ),
    ] {
        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "progress".to_string(),
                attrs,
                self_closing: false,
                span: None,
            }],
        );
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ProgressConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report.messages.iter().any(|m| m.code == expected_code),
            "missing {expected_code}"
        );
    }
}
