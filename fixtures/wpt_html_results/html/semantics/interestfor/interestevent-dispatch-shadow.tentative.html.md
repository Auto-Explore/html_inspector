# html/semantics/interestfor/interestevent-dispatch-shadow.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestevent-dispatch-shadow.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" title="Keith Cirkel" href="mailto:keithamus@github.com" >
<link rel="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com" >
<link
  rel="help"
  href="https://open-ui.org/components/interest-invokers.explainer/"
/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<body>
  <script>
  test((t) => {
    const host = document.createElement("div");
    t.add_cleanup(() => host.remove());
    document.body.append(host);
    const shadow = host.attachShadow({ mode: "closed" });
    const slot = shadow.appendChild(document.createElement("slot"));
    let childEvent = null;
    let childEventTarget = null;
    let childEventSource = null;
    let hostEvent = null;
    let hostEventTarget = null;
    let hostEventSource = null;
    slot.addEventListener("interest", (e) => {
        childEvent = e;
        childEventTarget = e.target;
        childEventSource = e.source;
      }, { once: true });
    host.addEventListener("interest", (e) => {
        hostEvent = e;
        hostEventTarget = e.target;
        hostEventSource = e.source;
      }, { once: true });
    const event = new InterestEvent("interest", {
      bubbles: true,
      source: slot,
      composed: true,
    });
    slot.dispatchEvent(event);
    assert_true(childEvent instanceof InterestEvent, "slot saw interest event");
    assert_equals(childEventTarget, slot, "target is child inside shadow boundary");
    assert_equals(childEventSource, slot, "source is child inside shadow boundary");
    assert_equals(hostEvent, childEvent, "event dispatch propagates across shadow boundary");
    assert_equals(hostEventTarget, host, "target is retargeted to shadowroot host");
    assert_equals(hostEventSource, host, "source is retargeted to shadowroot host");
  }, "InterestEvent propagates across shadow boundaries retargeting invoker source");

  promise_test(async (t) => {
    const host = document.createElement("div");
    t.add_cleanup(() => host.remove());
    document.body.append(host);
    const shadow = host.attachShadow({ mode: "open" });
    const button = shadow.appendChild(document.createElement("button"));
    const interestee = host.appendChild(document.createElement("div"));
    button.interestForElement = interestee;
    button.style = "interest-delay: 0s";
    let event = null;
    let eventTarget = null;
    let eventSource = null;
    interestee.addEventListener("interest", (e) => {
        event = e;
        eventTarget = e.target;
        eventSource = e.source;
      },{ once: true });
    await hoverOver(button);
    assert_true(!!event,"InterestEvent gets fired");
    assert_true(event instanceof InterestEvent);
    assert_equals(eventTarget, interestee, "target is interestee");
    assert_equals(eventSource, host, "interestee is host");
  }, "cross shadow InterestEvent retargets interestee to host element");
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/interestfor/interestevent-dispatch-shadow.tentative.html"
}
```
