mod a_download_constraints;
mod a_href_button_descendant;
mod a_href_constraints;
mod a_transparent_content_model;
mod accesskey_constraints;
mod address_constraints;
mod area_coords_constraints;
mod area_href_constraints;
mod area_map_ancestor;
mod aria_multiselectable_on_select_warning;
mod article_heading_warning;
mod attribute_not_allowed_constraints;
mod audio_src_constraints;
mod audio_transparent_content_model;
mod autocomplete_constraints;
mod autofocus_constraints;
mod base_element_constraints;
mod base_href_constraints;
mod base_in_body;
mod bdo_dir;
mod blockquote_cite_constraints;
mod button_formaction_constraints;
mod canvas_transparent_content_model;
mod commandfor_constraints;
mod content_security_policy_warnings;
mod csp;
mod data_attribute_constraints;
mod del_cite_constraints;
mod del_datetime_constraints;
mod del_ins_transparent_content_model;
mod details_summary_constraints;
mod dialog_constraints;
mod dl_child_content;
mod dl_div_child_content;
mod dl_div_group_constraints;
mod dl_duplicate_dt_name;
mod dl_structure_constraints;
mod doctype_required;
mod dt_descendant_constraints;
mod duplicate_id;
mod element_specific_attributes;
mod embed_constraints;
mod empty_heading_warning;
mod enterkeyhint_constraints;
mod figure_figcaption;
mod figure_table_caption_warning;
mod footer_constraints;
mod foreign_content;
mod form_action_constraints;
mod form_attribute_constraints;
mod h1_top_level_heading_warning;
mod header_constraints;
mod heading_skip_level_error;
mod headingoffset_constraints;
mod html5ever_parse_errors;
mod id_datatype_constraints;
mod iframe_constraints;
mod iframe_sandbox_constraints;
mod iframe_text_content_model;
mod img_alt_required;
mod img_dimension_constraints;
mod img_ismap_anchor_ancestor;
mod img_obsolete_attributes;
mod img_role_constraints;
mod img_sizes_auto_loading_lazy;
mod img_sizes_constraints;
mod img_src_constraints;
mod img_srcset_constraints;
mod img_srcset_sizes_required;
mod img_usemap_anchor_descendant;
mod img_usemap_constraints;
mod implied_p_end_tag;
mod input_attribute_allowed_types;
mod input_attribute_disallowed_by_type;
mod input_checkbox_role_button_aria_pressed;
mod input_color_constraints;
mod input_date_constraints;
mod input_datetime_local_constraints;
mod input_formaction_constraints;
mod input_image_src_constraints;
mod input_list_constraints;
mod input_month_constraints;
mod input_name_constraints;
mod input_number_constraints;
mod input_range_constraints;
mod input_size_constraints;
mod input_step_constraints;
mod input_time_constraints;
mod input_type_constraints;
mod input_url_value_constraints;
mod input_week_constraints;
mod ins_cite_constraints;
mod is_attribute_constraints;
mod label_control_count;
mod label_for_constraints;
mod li_parent_constraints;
mod li_value_constraints;
mod link_constraints;
mod link_href_constraints;
mod link_media_constraints;
mod main_constraints;
mod map_constraints;
mod math_role_warning;
mod mathml_constraints;
mod media_query;
mod menu_constraints;
mod meta_element_constraints;
mod meta_refresh_constraints;
mod meter_constraints;
mod microdata_constraints;
mod microdata_itemid_constraints;
mod microdata_itemref_constraints;
mod microdata_itemtype_constraints;
mod mimetype_constraints;
mod non_void_self_closing_syntax;
mod obj_element_constraints;
mod object_data_constraints;
mod obsolete_attribute_constraints;
mod obsolete_elements;
mod ol_start_constraints;
mod option_constraints;
mod p_disallowed_parent_constraints;
mod p_end_tag_scope;
mod phrasing_parent_child_constraints;
mod picture_attribute_constraints;
mod picture_content_model_constraints;
mod picture_parent_constraints;
mod picture_source_img_constraints;
mod picture_source_media_all_constraints;
mod picture_source_selection_constraints;
mod picture_unclosed_end_of_file;
mod placeholder_constraints;
mod popover_constraints;
mod progress_constraints;
mod q_cite_constraints;
mod rdfa_lite_constraints;
mod rel_typo_constraints;
mod ruby_constraints;
mod script_constraints;
mod script_importmap_constraints;
mod script_integrity_constraints;
mod script_speculationrules_constraints;
mod script_src_constraints;
mod script_text_content_constraints;
mod section_heading_warning;
mod select_constraints;
mod shared;
mod source_attribute_constraints;
mod source_media_constraints;
mod source_src_constraints;
mod spellcheck_constraints;
mod srcset_microsyntax;
mod style_constraints;
mod svg_image_srcset_constraints;
mod svg_suite_constraints;
mod svg_xmlns_constraints;
mod table_constraints;
mod target_browsing_context_constraints;
mod td_role_constraints;
mod textarea_constraints;
mod time_datetime_constraints;
mod time_text_content_constraints;
mod title_constraints;
mod tokenizer_parse_errors;
mod track_constraints;
mod track_src_constraints;
mod unchecked_subtree_warnings;
mod unknown_element_constraints;
mod unnecessary_role_warnings;
mod url_attr;
mod url_constraints;
mod video_poster_constraints;
mod video_src_constraints;
mod video_transparent_content_model;
mod void_element_end_tag;
mod xml_stylesheet_processing_instruction;

use html_inspector_core::Rule;

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(tokenizer_parse_errors::TokenizerParseErrors),
        Box::new(html5ever_parse_errors::Html5EverParseErrors),
        Box::new(
            xml_stylesheet_processing_instruction::XmlStylesheetProcessingInstruction::default(),
        ),
        Box::new(doctype_required::DoctypeRequired::default()),
        Box::new(duplicate_id::DuplicateId::default()),
        Box::new(obsolete_attribute_constraints::ObsoleteAttributeConstraints),
        Box::new(attribute_not_allowed_constraints::AttributeNotAllowedConstraints),
        Box::new(unknown_element_constraints::UnknownElementConstraints),
        Box::new(obj_element_constraints::ObjElementConstraints),
        Box::new(unnecessary_role_warnings::UnnecessaryRoleWarnings),
        Box::new(img_usemap_constraints::ImgUsemapConstraints::default()),
        Box::new(img_usemap_anchor_descendant::ImgUsemapAnchorDescendant::default()),
        Box::new(img_dimension_constraints::ImgDimensionConstraints),
        Box::new(img_ismap_anchor_ancestor::ImgIsmapAnchorAncestor::default()),
        Box::new(img_obsolete_attributes::ImgObsoleteAttributes),
        Box::new(img_role_constraints::ImgRoleConstraints),
        Box::new(img_sizes_auto_loading_lazy::ImgSizesAutoRequiresLazyLoading),
        Box::new(img_sizes_constraints::ImgSizesConstraints),
        Box::new(img_src_constraints::ImgSrcConstraints),
        Box::new(img_srcset_constraints::ImgSrcsetConstraints),
        Box::new(img_srcset_sizes_required::ImgSrcsetSizesRequired),
        Box::new(img_alt_required::ImgAltRequired),
        Box::new(area_coords_constraints::AreaCoordsConstraints),
        Box::new(area_href_constraints::AreaHrefConstraints),
        Box::new(area_map_ancestor::AreaRequiresMapAncestor),
        Box::new(accesskey_constraints::AccesskeyConstraints),
        Box::new(autocomplete_constraints::AutocompleteConstraints),
        Box::new(autofocus_constraints::AutofocusConstraints::default()),
        Box::new(a_download_constraints::ADownloadConstraints),
        Box::new(a_href_constraints::AHrefConstraints),
        Box::new(a_href_button_descendant::AHrefButtonDescendant),
        Box::new(a_transparent_content_model::ATransparentContentModel::default()),
        Box::new(address_constraints::AddressConstraints),
        Box::new(article_heading_warning::ArticleHeadingWarning::default()),
        Box::new(section_heading_warning::SectionHeadingWarning::default()),
        Box::new(h1_top_level_heading_warning::H1TopLevelHeadingWarning::default()),
        Box::new(audio_src_constraints::AudioSrcConstraints),
        Box::new(audio_transparent_content_model::AudioTransparentContentModel::default()),
        Box::new(video_poster_constraints::VideoPosterConstraints),
        Box::new(video_src_constraints::VideoSrcConstraints),
        Box::new(video_transparent_content_model::VideoTransparentContentModel::default()),
        Box::new(base_in_body::BaseInBody),
        Box::new(base_element_constraints::BaseElementConstraints::default()),
        Box::new(base_href_constraints::BaseHrefConstraints),
        Box::new(bdo_dir::BdoDir),
        Box::new(button_formaction_constraints::ButtonFormactionConstraints),
        Box::new(blockquote_cite_constraints::BlockquoteCiteConstraints),
        Box::new(q_cite_constraints::QCiteConstraints),
        Box::new(canvas_transparent_content_model::CanvasTransparentContentModel::default()),
        Box::new(commandfor_constraints::CommandforConstraints::default()),
        Box::new(data_attribute_constraints::DataAttributeConstraints),
        Box::new(del_cite_constraints::DelCiteConstraints),
        Box::new(del_datetime_constraints::DelDatetimeConstraints),
        Box::new(ins_cite_constraints::InsCiteConstraints),
        Box::new(del_ins_transparent_content_model::DelInsTransparentContentModel::default()),
        Box::new(details_summary_constraints::DetailsSummaryConstraints::default()),
        Box::new(dialog_constraints::DialogConstraints),
        Box::new(dt_descendant_constraints::DtDescendantConstraints::default()),
        Box::new(element_specific_attributes::ElementSpecificAttributes),
        Box::new(empty_heading_warning::EmptyHeadingWarning::default()),
        Box::new(enterkeyhint_constraints::EnterkeyhintConstraints),
        Box::new(embed_constraints::EmbedConstraints),
        Box::new(figure_figcaption::FigureFigcaption::default()),
        Box::new(figure_table_caption_warning::FigureTableCaptionWarning::default()),
        Box::new(form_attribute_constraints::FormAttributeConstraints::default()),
        Box::new(form_action_constraints::FormActionConstraints),
        Box::new(footer_constraints::FooterConstraints),
        Box::new(header_constraints::HeaderConstraints),
        Box::new(heading_skip_level_error::HeadingSkipLevelError::default()),
        Box::new(headingoffset_constraints::HeadingoffsetConstraints),
        Box::new(input_attribute_allowed_types::InputAttributeAllowedTypes),
        Box::new(input_attribute_disallowed_by_type::InputAttributeDisallowedByType),
        Box::new(input_checkbox_role_button_aria_pressed::InputCheckboxRoleButtonAriaPressed),
        Box::new(input_list_constraints::InputListConstraints::default()),
        Box::new(input_number_constraints::InputNumberConstraints),
        Box::new(input_range_constraints::InputRangeConstraints),
        Box::new(input_formaction_constraints::InputFormactionConstraints),
        Box::new(input_image_src_constraints::InputImageSrcConstraints),
        Box::new(input_url_value_constraints::InputUrlValueConstraints),
        Box::new(input_type_constraints::InputTypeConstraints),
        Box::new(input_color_constraints::InputColorConstraints),
        Box::new(input_date_constraints::InputDateConstraints),
        Box::new(input_datetime_local_constraints::InputDatetimeLocalConstraints),
        Box::new(input_month_constraints::InputMonthConstraints),
        Box::new(input_time_constraints::InputTimeConstraints),
        Box::new(input_week_constraints::InputWeekConstraints),
        Box::new(input_name_constraints::InputNameConstraints),
        Box::new(is_attribute_constraints::IsAttributeConstraints),
        Box::new(id_datatype_constraints::IdDatatypeConstraints),
        Box::new(iframe_constraints::IframeConstraints),
        Box::new(iframe_sandbox_constraints::IFrameSandboxConstraints),
        Box::new(iframe_text_content_model::IframeTextContentModel),
        Box::new(implied_p_end_tag::ImpliedPEndTag),
        Box::new(non_void_self_closing_syntax::NonVoidSelfClosingSyntax),
        Box::new(mimetype_constraints::ObjectMimetypeConstraints),
        Box::new(mimetype_constraints::LinkMimetypeConstraints),
        Box::new(object_data_constraints::ObjectDataConstraints),
        Box::new(placeholder_constraints::PlaceholderConstraints),
        Box::new(p_disallowed_parent_constraints::PDisallowedParentConstraints),
        Box::new(phrasing_parent_child_constraints::PhrasingParentChildConstraints),
        Box::new(label_control_count::LabelControlCount::default()),
        Box::new(label_for_constraints::LabelForConstraints::default()),
        Box::new(li_parent_constraints::LiParentConstraints),
        Box::new(li_value_constraints::LiValueConstraints),
        Box::new(link_media_constraints::LinkMediaConstraints),
        Box::new(link_constraints::LinkConstraints),
        Box::new(link_href_constraints::LinkHrefConstraints),
        Box::new(main_constraints::MainConstraints::default()),
        Box::new(map_constraints::MapConstraints),
        Box::new(math_role_warning::MathRoleWarning),
        Box::new(mathml_constraints::MathmlConstraints::default()),
        Box::new(menu_constraints::MenuConstraints::default()),
        Box::new(meter_constraints::MeterConstraints),
        Box::new(meta_refresh_constraints::MetaRefreshConstraints),
        Box::new(meta_element_constraints::MetaElementConstraints::default()),
        Box::new(title_constraints::TitleConstraints::default()),
        Box::new(content_security_policy_warnings::ContentSecurityPolicyWarnings::default()),
        Box::new(ol_start_constraints::OlStartConstraints),
        Box::new(option_constraints::OptionConstraints::default()),
        Box::new(source_attribute_constraints::SourceAttributeConstraints),
        Box::new(svg_image_srcset_constraints::SvgImageSrcsetConstraints),
        Box::new(svg_xmlns_constraints::SvgXmlnsConstraints),
        Box::new(unknown_element_constraints::UnknownSvgElementConstraints),
        Box::new(svg_suite_constraints::SvgSuiteConstraints::default()),
        Box::new(picture_content_model_constraints::PictureContentModelConstraints::default()),
        Box::new(picture_parent_constraints::PictureParentConstraints),
        Box::new(picture_attribute_constraints::PictureAttributeConstraints),
        Box::new(picture_source_img_constraints::PictureSourceImgConstraints::default()),
        Box::new(
            picture_source_selection_constraints::PictureSourceSelectionConstraints::default(),
        ),
        Box::new(picture_source_media_all_constraints::PictureSourceMediaAllConstraints::default()),
        Box::new(picture_unclosed_end_of_file::PictureUnclosedEndOfFile),
        Box::new(progress_constraints::ProgressConstraints),
        Box::new(popover_constraints::PopoverConstraints),
        Box::new(rel_typo_constraints::RelTypoConstraints),
        Box::new(rdfa_lite_constraints::RdfaLiteConstraints),
        Box::new(script_integrity_constraints::ScriptIntegrityConstraints),
        Box::new(script_constraints::ScriptConstraints),
        Box::new(script_importmap_constraints::ScriptImportmapConstraints::default()),
        Box::new(script_speculationrules_constraints::ScriptSpeculationrulesConstraints::default()),
        Box::new(script_src_constraints::ScriptSrcConstraints),
        Box::new(script_text_content_constraints::ScriptTextContentConstraints::default()),
        Box::new(select_constraints::SelectConstraints::default()),
        Box::new(aria_multiselectable_on_select_warning::AriaMultiselectableOnSelectWarning),
        Box::new(source_media_constraints::SourceMediaConstraints),
        Box::new(source_src_constraints::SourceSrcConstraints),
        Box::new(spellcheck_constraints::SpellcheckConstraints),
        Box::new(style_constraints::StyleConstraints),
        Box::new(target_browsing_context_constraints::TargetBrowsingContextConstraints),
        Box::new(input_size_constraints::InputSizeConstraints),
        Box::new(input_step_constraints::InputStepConstraints),
        Box::new(textarea_constraints::TextareaConstraints),
        Box::new(td_role_constraints::TdRoleConstraints::default()),
        Box::new(table_constraints::TableConstraints::default()),
        Box::new(time_datetime_constraints::TimeDatetimeConstraints),
        Box::new(time_text_content_constraints::TimeTextContentConstraints::default()),
        Box::new(url_constraints::UrlConstraints),
        Box::new(void_element_end_tag::VoidElementEndTag),
        Box::new(track_src_constraints::TrackSrcConstraints),
        Box::new(track_constraints::TrackConstraints::default()),
        Box::new(ruby_constraints::RubyConstraints::default()),
        Box::new(microdata_itemid_constraints::MicrodataItemidConstraints),
        Box::new(microdata_itemref_constraints::MicrodataItemrefConstraints::default()),
        Box::new(microdata_itemtype_constraints::MicrodataItemtypeConstraints),
        Box::new(microdata_constraints::MicrodataConstraints),
        Box::new(dl_child_content::DlChildContent),
        Box::new(dl_div_child_content::DlDivChildContent::default()),
        Box::new(dl_div_group_constraints::DlDivGroupConstraints::default()),
        Box::new(dl_duplicate_dt_name::DlDuplicateDtName::default()),
        Box::new(dl_structure_constraints::DlStructureConstraints::default()),
        Box::new(p_end_tag_scope::PEndTagScope::default()),
        Box::new(obsolete_elements::ObsoleteElements),
        Box::new(unchecked_subtree_warnings::UncheckedSubtreeWarnings::default()),
    ]
}

#[cfg(test)]
mod tests;
