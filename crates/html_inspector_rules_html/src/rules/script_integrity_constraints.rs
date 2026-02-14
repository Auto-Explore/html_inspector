use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ScriptIntegrityConstraints;

impl Rule for ScriptIntegrityConstraints {
    fn id(&self) -> &'static str {
        "html.script.integrity"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };
        if !is(ctx, name, "script") {
            return;
        }
        let integrity = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("integrity"),
                html_inspector::InputFormat::Xhtml => a.name == "integrity",
            })
            .and_then(|a| a.value.as_deref());
        let Some(integrity) = integrity else { return };

        let ok = integrity.split_ascii_whitespace().all(is_integrity_token);

        if !ok {
            out.push(Message::new(
                "html.script.integrity.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{integrity}” for attribute “integrity” on element “script”."),
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn is_integrity_token(token: &str) -> bool {
    let token = token.trim();
    let Some((alg, b64)) = token.split_once('-') else {
        return false;
    };
    let alg_ok = matches!(alg, "sha256" | "sha384" | "sha512")
        || alg.eq_ignore_ascii_case("sha256")
        || alg.eq_ignore_ascii_case("sha384")
        || alg.eq_ignore_ascii_case("sha512");
    if !alg_ok || b64.is_empty() {
        return false;
    }
    if !b64
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
    {
        return false;
    }
    true
}
