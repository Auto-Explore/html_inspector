# html/semantics/interestfor/interestfor-button-event-dispatch.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-button-event-dispatch.tentative.html",
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
<button id="interestbutton" interestfor="interestee">Button</button>
<button id="otherbutton">Other Button</button>
<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
  promise_test(async function (t) {
    let event = null;
    interestee.addEventListener("interest", (e) => (event = e), { once: true });
    await hoverOver(interestbutton);
    assert_true(!!event, "InterestEvent is fired");
    assert_true(event instanceof InterestEvent, "event is InterestEvent");
    assert_equals(event.type, "interest", "type");
    assert_equals(event.bubbles, false, "bubbles");
    assert_equals(event.composed, true, "composed");
    assert_equals(event.isTrusted, true, "isTrusted");
    assert_equals(event.target, interestee, "target");
    assert_equals(event.source, interestbutton, "source");
  }, "InterestEvent dispatches on button hover");

  promise_test(async function (t) {
    t.add_cleanup(async () => {
      interestbutton.removeAttribute('disabled');
      await hoverOver(otherbutton);
    });
    let called = false;
    interestee.addEventListener(
      "interest",
      () => {
        called = true;
      },
      { once: true },
    );
    interestbutton.setAttribute("disabled", "");
    await hoverOver(interestbutton);
    assert_false(called, "event was not called");
  }, "event does not dispatch if invoker is disabled");

  promise_test(async function (t) {
    svgInterestee = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    t.add_cleanup(async () => {
      interestbutton.interestForElement = interestee;
      svgInterestee.remove();
      await hoverOver(otherbutton);
    });
    document.body.append(svgInterestee);
    let called = false;
    assert_false(svgInterestee instanceof HTMLElement);
    assert_true(svgInterestee instanceof Element);
    let event = null;
    svgInterestee.addEventListener("interest", (e) => (event = e), { once: true });
    interestbutton.interestForElement = svgInterestee;
    await hoverOver(interestbutton);
    assert_true(!!event, "InterestEvent is fired");
    assert_equals(event.source, interestbutton, "event.source is set to right element");
    assert_equals(event.target, svgInterestee, "event.target is set to right element");
  }, "event dispatches if interestee is non-HTML Element");
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
        "byte_end": 799,
        "byte_start": 792,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/interestfor/interestfor-button-event-dispatch.tentative.html"
}
```
