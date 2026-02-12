mod rules;

use html_inspector_core::RuleSet;

pub fn pack_i18n() -> RuleSet {
    RuleSet::new().extend(rules::all())
}
