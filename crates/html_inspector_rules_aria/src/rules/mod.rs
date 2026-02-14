mod aria_activedescendant_requires_descendant_or_owns_warning;
mod aria_br_wbr_constraints;
mod aria_checked_allowed_role;
mod aria_checked_on_checkbox;
mod aria_deprecated_attributes;
mod aria_disabled_on_a_with_href_warning;
mod aria_disabled_unnecessary_on_disabled_warning;
mod aria_expanded_with_command;
mod aria_expanded_with_popovertarget;
mod aria_global_properties_on_main_warning;
mod aria_haspopup_on_datalist_input_warning;
mod aria_hidden_constraints;
mod aria_idref_exists;
mod aria_multiple_main_warning;
mod aria_naming_prohibited_by_role;
mod aria_placeholder_with_placeholder;
mod aria_pressed_requires_role;
mod aria_properties_supported_by_role;
mod aria_readonly_requires_context;
mod aria_role_hierarchy_constraints;
mod aria_select_role_constraints;
mod aria_selected_on_option;
mod aria_summary_constraints;
mod aria_tabpanel_required_for_active_tab;
mod shared;

use html_inspector::Rule;

pub fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(aria_checked_on_checkbox::AriaCheckedOnCheckbox),
        Box::new(aria_deprecated_attributes::AriaDeprecatedAttributes),
        Box::new(aria_placeholder_with_placeholder::AriaPlaceholderWithPlaceholder),
        Box::new(aria_readonly_requires_context::AriaReadonlyRequiresContext),
        Box::new(aria_expanded_with_command::AriaExpandedWithCommand),
        Box::new(aria_expanded_with_popovertarget::AriaExpandedWithPopoverTarget),
        Box::new(aria_selected_on_option::AriaSelectedOnOption),
        Box::new(aria_pressed_requires_role::AriaPressedRequiresRole),
        Box::new(aria_disabled_on_a_with_href_warning::AriaDisabledOnAWithHrefWarning),
        Box::new(
            aria_activedescendant_requires_descendant_or_owns_warning::AriaActivedescendantRequiresDescendantOrOwnsWarning::default(),
        ),
        Box::new(aria_naming_prohibited_by_role::AriaNamingProhibitedByRole),
        Box::new(aria_checked_allowed_role::AriaCheckedAllowedRole),
        Box::new(aria_idref_exists::AriaIdrefExists::default()),
        Box::new(
            aria_disabled_unnecessary_on_disabled_warning::AriaDisabledUnnecessaryOnDisabledWarning,
        ),
        Box::new(aria_haspopup_on_datalist_input_warning::AriaHaspopupOnDatalistInputWarning),
        Box::new(aria_hidden_constraints::AriaHiddenConstraints),
        Box::new(aria_properties_supported_by_role::AriaPropertiesSupportedByRole),
        Box::new(aria_br_wbr_constraints::AriaBrWbrConstraints),
        Box::new(aria_role_hierarchy_constraints::AriaRoleHierarchyConstraints::default()),
        Box::new(aria_multiple_main_warning::AriaMultipleMainWarning::default()),
        Box::new(aria_global_properties_on_main_warning::AriaGlobalPropertiesOnMainWarning),
        Box::new(aria_select_role_constraints::AriaSelectRoleConstraints),
        Box::new(aria_summary_constraints::AriaSummaryConstraints::default()),
        Box::new(aria_tabpanel_required_for_active_tab::AriaTabpanelRequiredForActiveTab::default()),
    ]
}

#[cfg(test)]
mod tests;
