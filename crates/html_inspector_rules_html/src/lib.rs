mod rules;

use html_inspector::RuleSet;

pub fn pack_html_conformance() -> RuleSet {
    RuleSet::new().extend(rules::all())
}
