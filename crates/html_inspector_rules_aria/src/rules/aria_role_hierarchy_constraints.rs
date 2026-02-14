use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct AriaRoleHierarchyConstraints {
    role_stack: Vec<RoleEntry>,
    owned_by_roles: FxHashMap<String, Vec<String>>,
    implicit_list_stack: Vec<String>,
}

#[derive(Clone, Debug)]
struct RoleEntry {
    role_lc: String,
    depth: usize,
}

impl Rule for AriaRoleHierarchyConstraints {
    fn id(&self) -> &'static str {
        "aria.role.hierarchy_constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.role_stack.clear();
        self.owned_by_roles.clear();
        self.implicit_list_stack.clear();
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name,
                attrs,
                self_closing,
                span,
            } => {
                let role_value = ctx.attr_value(attrs, "role");
                let role_lc = role_value
                    .and_then(|v| v.split_ascii_whitespace().next())
                    .map(|t| t.to_ascii_lowercase());
                let has_role_attr = role_value.is_some();

                // Record aria-owns relationships for later containment checks.
                if let (Some(owner_role), Some(owns)) =
                    (role_lc.as_deref(), ctx.attr_value(attrs, "aria-owns"))
                    && is_owner_role(owner_role) {
                        for token in owns.split_ascii_whitespace() {
                            self.owned_by_roles
                                .entry(token.to_string())
                                .or_default()
                                .push(owner_role.to_string());
                        }
                    }

                // Enforce specific role + attribute warnings.
                if role_lc.as_deref() == Some("listitem") && ctx.has_attr(attrs, "aria-level") {
                    out.push(Message::new(
                        "aria.role_listitem.aria_level.discouraged",
                        Severity::Warning,
                        Category::Aria,
                        "The “aria-level” attribute should not be used on any element which has “role=listitem”.",
                        *span,
                    ));
                }

                // Enforce required-owner roles.
                let id = ctx.attr_value(attrs, "id").unwrap_or("");
                if role_lc.as_deref() == Some("cell")
                    && !self.has_ancestor_role("row")
                    && !self.is_owned_by(id, &["row"])
                {
                    out.push(Message::new(
                        "aria.role_cell.requires_row",
                        Severity::Error,
                        Category::Aria,
                        "An element with “role=cell” must be contained in, or owned by, an element with the “role” value “row”.",
                        *span,
                    ));
                }
                if role_lc.as_deref() == Some("option")
                    && !self.has_ancestor_role("listbox")
                    && !self.is_owned_by(id, &["listbox"])
                {
                    out.push(Message::new(
                        "aria.role_option.requires_listbox",
                        Severity::Error,
                        Category::Aria,
                        "An element with “role=option” must be contained in, or owned by, an element with the “role” value “listbox”.",
                        *span,
                    ));
                }
                if role_lc.as_deref() == Some("row")
                    && !(self.has_ancestor_role("treegrid")
                        || self.has_ancestor_role("grid")
                        || self.has_ancestor_role("rowgroup")
                        || self.has_ancestor_role("table"))
                    && !self.is_owned_by(id, &["treegrid", "grid", "rowgroup", "table"])
                {
                    out.push(Message::new(
                        "aria.role_row.requires_table_like",
                        Severity::Error,
                        Category::Aria,
                        "An element with “role=row” must be contained in, or owned by, an element with the “role” value “treegrid”, “grid”, “rowgroup”, or “table”.",
                        *span,
                    ));
                }

                // Enforce constraints for <li> descendants of particular role containers.
                if ctx.name_is(name, "li")
                    && let Some(li_role) = role_lc.as_deref() {
                        if self.has_ancestor_role("listbox")
                            && !(li_role == "group" || li_role == "option")
                        {
                            out.push(Message::new(
                                "aria.li.role.descendant_of_listbox_or_list",
                                Severity::Error,
                                Category::Aria,
                                "An “li” element that is a descendant of a “role=listbox” element or “role=list” element must not have any “role” value other than “group” or “option”.",
                                *span,
                            ));
                        }
                        if (self.has_ancestor_role("menu") || self.has_ancestor_role("menubar"))
                            && !matches!(
                                li_role,
                                "group"
                                    | "menuitem"
                                    | "menuitemcheckbox"
                                    | "menuitemradio"
                                    | "none"
                                    | "presentation"
                                    | "separator"
                            )
                        {
                            out.push(Message::new(
                                "aria.li.role.descendant_of_menu_or_menubar",
                                Severity::Error,
                                Category::Aria,
                                "An “li” element that is a descendant of a “role=menu” element or “role=menubar” element must not have any “role” value other than “group”, “menuitem”, “menuitemcheckbox”, “menuitemradio”, or “separator”.",
                                *span,
                            ));
                        }
                        if self.has_ancestor_role("tablist") && li_role != "tab" {
                            out.push(Message::new(
                                "aria.li.role.descendant_of_tablist",
                                Severity::Error,
                                Category::Aria,
                                "An “li” element that is a descendant of a “role=tablist” element must not have any “role” value other than “tab”.",
                                *span,
                            ));
                        }
                        if self.has_ancestor_role("tree") && li_role != "treeitem" {
                            out.push(Message::new(
                                "aria.li.role.descendant_of_tree",
                                Severity::Error,
                                Category::Aria,
                                "An “li” element that is a descendant of a “role=tree” element must not have any “role” value other than “treeitem”.",
                                *span,
                            ));
                        }

                        if (self.has_ancestor_role("list") || !self.implicit_list_stack.is_empty())
                            && has_role_attr
                        {
                            out.push(Message::new(
                                "aria.li.role.descendant_of_implicit_or_role_list",
                                Severity::Error,
                                Category::Aria,
                                "An “li” element that is a descendant of a “ul”, “ol”, or “menu” element with no explicit “role” value, or a descendant of a “role=list” element, must not have any “role” value other than “listitem”.",
                                *span,
                            ));
                        }
                    }

                // Track ancestor roles we care about for descendant/containment checks.
                let pushes = match ctx.format {
                    html_inspector_core::InputFormat::Html => {
                        !html_inspector_core::is_void_html_element(name)
                    }
                    html_inspector_core::InputFormat::Xhtml => !*self_closing,
                };
                if pushes {
                    if (ctx.name_is(name, "ul")
                        || ctx.name_is(name, "ol")
                        || ctx.name_is(name, "menu"))
                        && !has_role_attr
                    {
                        self.implicit_list_stack.push(name.clone());
                    }
                    if let Some(role) = role_lc
                        && is_tracked_container_role(&role) {
                            self.role_stack.push(RoleEntry {
                                role_lc: role,
                                depth: ctx.open_elements().len() + 1,
                            });
                        }
                }
            }
            ParseEvent::EndTag { name, .. } => self.on_end_tag(name, ctx),
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.role_stack.clear();
        self.owned_by_roles.clear();
        self.implicit_list_stack.clear();
    }
}

impl AriaRoleHierarchyConstraints {
    fn has_ancestor_role(&self, role_lc: &str) -> bool {
        self.role_stack.iter().any(|e| e.role_lc == role_lc)
    }

    fn is_owned_by(&self, id: &str, owner_roles: &[&str]) -> bool {
        if id.is_empty() {
            return false;
        }
        let Some(owners) = self.owned_by_roles.get(id) else {
            return false;
        };
        owners.iter().any(|r| owner_roles.contains(&r.as_str()))
    }

    fn on_end_tag(&mut self, name: &str, ctx: &ValidationContext) {
        let closed_depth = closed_stack_depth(name, ctx);
        self.role_stack.retain(|e| e.depth <= closed_depth);
        let pos = match ctx.format {
            html_inspector_core::InputFormat::Html => self
                .implicit_list_stack
                .iter()
                .rposition(|n| n.eq_ignore_ascii_case(name)),
            html_inspector_core::InputFormat::Xhtml => {
                self.implicit_list_stack.iter().rposition(|n| n == name)
            }
        };
        if let Some(pos) = pos {
            self.implicit_list_stack.truncate(pos);
        }
    }
}

fn closed_stack_depth(name: &str, ctx: &ValidationContext) -> usize {
    let depth = ctx.open_elements().len();
    let pos = match ctx.format {
        html_inspector_core::InputFormat::Html => ctx
            .open_elements()
            .iter()
            .rposition(|n| n.eq_ignore_ascii_case(name)),
        html_inspector_core::InputFormat::Xhtml => {
            ctx.open_elements().iter().rposition(|n| n == name)
        }
    };
    pos.unwrap_or(depth)
}

fn is_owner_role(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "listbox" | "row" | "treegrid" | "grid" | "rowgroup" | "table"
    )
}

fn is_tracked_container_role(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "listbox"
            | "list"
            | "menu"
            | "menubar"
            | "tablist"
            | "tree"
            | "row"
            | "treegrid"
            | "grid"
            | "rowgroup"
            | "table"
    )
}

#[cfg(test)]
mod tests {
    use super::AriaRoleHierarchyConstraints;
    use html_inspector_core::{
        validate_events, Attribute, Config, EventSource, InputFormat, ParseEvent, RuleSet, Span,
        ValidatorError,
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
    fn role_value_uses_first_token_when_whitespace_separated() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![
                    Attribute {
                        name: "role".to_string(),
                        value: Some("  listitem other  ".to_string()),
                        span: None,
                    },
                    Attribute {
                        name: "aria-level".to_string(),
                        value: Some("2".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span,
            }],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(AriaRoleHierarchyConstraints::default()),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["aria.role_listitem.aria_level.discouraged"]);
        assert_eq!(report.messages[0].span, span);
    }

    #[test]
    fn is_owned_by_rejects_empty_or_unknown_ids() {
        let c = AriaRoleHierarchyConstraints::default();
        assert!(!c.is_owned_by("", &["row"]));
        assert!(!c.is_owned_by("missing", &["row"]));
    }

    #[test]
    fn is_owned_by_matches_any_allowed_owner_role() {
        let mut c = AriaRoleHierarchyConstraints::default();
        c.owned_by_roles.insert(
            "cell1".to_string(),
            vec!["row".to_string(), "grid".to_string()],
        );

        assert!(c.is_owned_by("cell1", &["grid"]));
        assert!(c.is_owned_by("cell1", &["table", "grid"]));
        assert!(!c.is_owned_by("cell1", &["table"]));
    }
}
