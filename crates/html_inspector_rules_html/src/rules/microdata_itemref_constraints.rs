use std::collections::VecDeque;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};
use rustc_hash::{FxHashMap, FxHashSet};

use super::foreign_content::{namespace_for_next_start_tag, Namespace};

#[derive(Default)]
pub struct MicrodataItemrefConstraints {
    nodes: Vec<Node>,
    id_to_node: FxHashMap<String, usize>,
    items: Vec<usize>,
    properties: FxHashSet<usize>,
    open_relevant: Vec<OpenFrame>,
}

#[derive(Debug)]
struct Node {
    span: Option<Span>,
    item_prop: bool,
    item_scope: bool,
    item_ref: Option<Vec<String>>,
    children: Vec<usize>,
}

#[derive(Debug)]
struct OpenFrame {
    name: String,
    node: usize,
}

impl Rule for MicrodataItemrefConstraints {
    fn id(&self) -> &'static str {
        "html.microdata.itemref.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.nodes.clear();
        self.id_to_node.clear();
        self.items.clear();
        self.properties.clear();
        self.open_relevant.clear();
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name,
                attrs,
                self_closing,
                span,
            } => {
                if namespace_for_next_start_tag(ctx, name) != Namespace::Html {
                    return;
                }

                let id = ctx
                    .attr_value(attrs, "id")
                    .map(str::trim)
                    .filter(|s| !s.is_empty());
                let item_prop = ctx.has_attr(attrs, "itemprop");
                let item_scope = ctx.has_attr(attrs, "itemscope");
                let item_ref = ctx
                    .attr_value(attrs, "itemref")
                    .map(|s| s.split_ascii_whitespace().map(|t| t.to_string()).collect());

                let relevant = id.is_some() || item_prop || item_scope;
                if !relevant {
                    return;
                }

                let node = self.nodes.len();
                if let Some(parent) = self.open_relevant.last().map(|f| f.node) {
                    self.nodes[parent].children.push(node);
                }

                self.nodes.push(Node {
                    span: *span,
                    item_prop,
                    item_scope,
                    item_ref,
                    children: Vec::new(),
                });

                if item_prop {
                    self.properties.insert(node);
                } else if item_scope {
                    self.items.push(node);
                }

                if let Some(id) = id {
                    self.id_to_node.entry(id.to_string()).or_insert(node);
                }

                let pushes = match ctx.format {
                    html_inspector_core::InputFormat::Html => {
                        !html_inspector_core::is_void_html_element(name)
                    }
                    html_inspector_core::InputFormat::Xhtml => !*self_closing,
                };
                if pushes {
                    self.open_relevant.push(OpenFrame {
                        name: normalize_name(ctx, name),
                        node,
                    });
                }
            }
            ParseEvent::EndTag { name, .. } => {
                let Some(pos) = self
                    .open_relevant
                    .iter()
                    .rposition(|f| ctx.name_is(&f.name, name))
                else {
                    return;
                };
                self.open_relevant.truncate(pos);
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        let mut parents = Vec::with_capacity(self.items.len());
        let items = self.items.clone();
        for item in items {
            self.check_item(item, &mut parents, out);
        }

        // Emit errors for unreferenced properties (matches Java MicrodataChecker behavior).
        let mut props: Vec<usize> = self.properties.iter().copied().collect();
        props.sort_unstable();
        for prop in props {
            let span = self.nodes.get(prop).and_then(|n| n.span);
            out.push(Message::new(
                "html.microdata.itemprop.not_in_item",
                Severity::Error,
                Category::Html,
                "The “itemprop” attribute was specified, but the element is not a property of any item.",
                span,
            ));
        }
    }
}

impl MicrodataItemrefConstraints {
    fn check_item(&mut self, root: usize, parents: &mut Vec<usize>, out: &mut dyn MessageSink) {
        let initial_capacity = self.nodes.get(root).map_or(0, |node| {
            node.children.len() + node.item_ref.as_ref().map_or(0, |v| v.len())
        });
        let mut pending: VecDeque<usize> = VecDeque::with_capacity(initial_capacity);
        let mut memory: FxHashSet<usize> = FxHashSet::default();
        memory.insert(root);

        if let Some(node) = self.nodes.get(root) {
            for &child in &node.children {
                pending.push_back(child);
            }
            if let Some(item_ref) = node.item_ref.as_ref() {
                for id in item_ref {
                    if let Some(&ref_node) = self.id_to_node.get(id) {
                        pending.push_back(ref_node);
                    } else {
                        out.push(Message::new(
                            "html.microdata.itemref.missing_id",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "The “itemref” attribute referenced “{id}”, but there is no element with an “id” attribute with that value."
                            ),
                            node.span,
                        ));
                    }
                }
            }
        }

        let mut memory_error = false;
        while let Some(current) = pending.pop_back() {
            if !memory.insert(current) {
                memory_error = true;
                continue;
            }

            let (item_prop, item_scope, children, span) = match self.nodes.get(current) {
                Some(n) => (n.item_prop, n.item_scope, n.children.clone(), n.span),
                None => continue,
            };

            if !item_scope {
                for child in children {
                    pending.push_back(child);
                }
            }

            if item_prop {
                self.properties.remove(&current);
                if item_scope {
                    if parents.contains(&current) {
                        out.push(Message::new(
                            "html.microdata.itemref.circular",
                            Severity::Error,
                            Category::Html,
                            "The “itemref” attribute created a circular reference with another item.",
                            span,
                        ));
                    } else {
                        parents.push(root);
                        self.check_item(current, parents, out);
                        parents.pop();
                    }
                }
            }
        }

        if memory_error {
            let span = self.nodes.get(root).and_then(|n| n.span);
            out.push(Message::new(
                "html.microdata.itemref.redundant",
                Severity::Error,
                Category::Html,
                "The “itemref” attribute contained redundant references.",
                span,
            ));
        }
    }
}

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector_core::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector_core::InputFormat::Xhtml => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn itemref_reports_missing_ids_for_all_tokens() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![
                        Attribute {
                            name: "itemscope".to_string(),
                            value: None,
                            span: None,
                        },
                        Attribute {
                            name: "itemref".to_string(),
                            value: Some("a b".to_string()),
                            span: None,
                        },
                    ],
                    self_closing: false,
                    span,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(MicrodataItemrefConstraints::default()),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "html.microdata.itemref.missing_id",
                "html.microdata.itemref.missing_id"
            ]
        );
        assert!(report.messages[0].message.contains("referenced “a”"));
        assert!(report.messages[1].message.contains("referenced “b”"));
    }

    #[test]
    fn itemref_whitespace_only_value_is_ok() {
        let span = Some(Span::new(1, 2, 1, 1));
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
                        name: "itemref".to_string(),
                        value: Some(" \t\r\n ".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span,
            }],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(MicrodataItemrefConstraints::default()),
            Config::default(),
        )
        .unwrap();

        assert!(report.messages.is_empty());
    }

    #[test]
    fn itemref_redundant_when_referencing_descendant_property() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![
                        Attribute {
                            name: "itemscope".to_string(),
                            value: None,
                            span: None,
                        },
                        Attribute {
                            name: "itemref".to_string(),
                            value: Some("x".to_string()),
                            span: None,
                        },
                    ],
                    self_closing: false,
                    span,
                },
                ParseEvent::StartTag {
                    name: "span".to_string(),
                    attrs: vec![
                        Attribute {
                            name: "id".to_string(),
                            value: Some("x".to_string()),
                            span: None,
                        },
                        Attribute {
                            name: "itemprop".to_string(),
                            value: Some("p".to_string()),
                            span: None,
                        },
                    ],
                    self_closing: false,
                    span,
                },
                ParseEvent::EndTag {
                    name: "span".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(MicrodataItemrefConstraints::default()),
            Config::default(),
        )
        .unwrap();

        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.microdata.itemref.redundant"));
    }

    #[test]
    fn itemref_cycle_between_items_is_reported() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![Attribute {
                        name: "itemscope".to_string(),
                        value: None,
                        span: None,
                    }],
                    self_closing: false,
                    span,
                },
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![
                        Attribute {
                            name: "id".to_string(),
                            value: Some("a".to_string()),
                            span: None,
                        },
                        Attribute {
                            name: "itemscope".to_string(),
                            value: None,
                            span: None,
                        },
                        Attribute {
                            name: "itemprop".to_string(),
                            value: Some("p".to_string()),
                            span: None,
                        },
                        Attribute {
                            name: "itemref".to_string(),
                            value: Some("b".to_string()),
                            span: None,
                        },
                    ],
                    self_closing: false,
                    span,
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
                            value: Some("b".to_string()),
                            span: None,
                        },
                        Attribute {
                            name: "itemscope".to_string(),
                            value: None,
                            span: None,
                        },
                        Attribute {
                            name: "itemprop".to_string(),
                            value: Some("q".to_string()),
                            span: None,
                        },
                        Attribute {
                            name: "itemref".to_string(),
                            value: Some("a".to_string()),
                            span: None,
                        },
                    ],
                    self_closing: false,
                    span,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(MicrodataItemrefConstraints::default()),
            Config::default(),
        )
        .unwrap();

        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.microdata.itemref.circular"));
    }

    #[test]
    fn unreferenced_itemprop_is_reported_at_finish() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![Attribute {
                        name: "itemprop".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    }],
                    self_closing: false,
                    span,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(MicrodataItemrefConstraints::default()),
            Config::default(),
        )
        .unwrap();

        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.microdata.itemprop.not_in_item"));
    }
}
