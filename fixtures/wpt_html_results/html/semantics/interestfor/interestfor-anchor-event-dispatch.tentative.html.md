# html/semantics/interestfor/interestfor-anchor-event-dispatch.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-anchor-event-dispatch.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<link rel="author" title="Keith Cirkel" href="mailto:keithamus@github.com" >
<link rel="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com" >
<link rel=author href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id="interestee"></div>
<a id="nohref" interestfor="interestee">Anchor</a>
<a href="/" id="interestanchor" interestfor="interestee">Anchor</a>
<button id="otherbutton">Other Button</button>
<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
  promise_test(async function (t) {
    t.add_cleanup(async () => {
      await hoverOver(otherbutton);
    });
    let gotEvent = false;
    interestee.addEventListener("interest", () => (gotEvent = true));
    await hoverOver(nohref);
    assert_false(gotEvent, "InterestEvent should not get fired");
    nohref.setAttribute('href','foo');
    await hoverOver(nohref);
    assert_false(gotEvent, "adding href while the element is already hovered should not fire interest");
    await hoverOver(otherbutton);
    await hoverOver(nohref);
    assert_true(gotEvent, "interest should now be fired");
  }, "InterestEvent is not dispatched unless the anchor has an href");

  promise_test(async function (t) {
    t.add_cleanup(async () => {
      await hoverOver(otherbutton);
    });
    let event = null;
    interestee.addEventListener("interest", (e) => (event = e), { once: true });
    await hoverOver(interestanchor);
    assert_true(!!event, "InterestEvent is fired");
    assert_true(event instanceof InterestEvent, "event is InterestEvent");
    assert_equals(event.type, "interest", "type");
    assert_equals(event.bubbles, false, "bubbles");
    assert_equals(event.composed, true, "composed");
    assert_equals(event.isTrusted, true, "isTrusted");
    assert_equals(event.target, interestee, "target");
    assert_equals(event.source, interestanchor, "source");
  }, "InterestEvent dispatches on anchor hover");
</script>

<br>
<a href="/" id="parentanchor" interestfor="childdiv"><span>Anchor</span>
  <div id="childdiv">Child div</div>
</a>

<script>
  promise_test(async function (t) {
    let events = [];
    childdiv.addEventListener("interest", (e) => (events.push('interest')));
    childdiv.addEventListener("loseinterest", (e) => (events.push('loseinterest')));
    const justAnchor = parentanchor.firstChild;
    assert_true(justAnchor instanceof HTMLSpanElement);
    await hoverOver(justAnchor);
    assert_array_equals(events,['interest'],'Hovering anchor should show interest');
    await hoverOver(childdiv);
    assert_array_equals(events,['interest'],'Hovering the target shouldn\'t fire any new events');
    await hoverOver(justAnchor);
    assert_array_equals(events,['interest'],'Hovering back on the anchor shouldn\'t fire new events');
    await hoverOver(otherbutton);
    assert_array_equals(events,['interest','loseinterest'],'De-hovering both should fire loseinterest');
  }, "Nested invoker and invokee");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 849,
        "byte_start": 842,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interestfor/interestfor-anchor-event-dispatch.tentative.html"
}
```
