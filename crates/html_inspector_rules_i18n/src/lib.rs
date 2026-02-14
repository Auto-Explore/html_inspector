mod rules;

use html_inspector::RuleSet;

pub fn pack_i18n() -> RuleSet {
    RuleSet::new().extend(rules::all())
}
