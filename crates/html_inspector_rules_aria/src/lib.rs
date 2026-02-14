mod rules;

use html_inspector::RuleSet;

pub fn pack_aria() -> RuleSet {
    RuleSet::new().extend(rules::all())
}
