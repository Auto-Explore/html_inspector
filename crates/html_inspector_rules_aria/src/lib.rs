mod rules;

use html_inspector_core::RuleSet;

pub fn pack_aria() -> RuleSet {
    RuleSet::new().extend(rules::all())
}
