# html/semantics/interestfor/interestevent-interface.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestevent-interface.tentative.html",
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
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id="div"></div>
<button id="button"></button>

<script>
  test(function () {
    const event = new InterestEvent("test");
    assert_equals(event.source, null);
    assert_readonly(event, "source", "readonly attribute value");
  }, "source is readonly defaulting to null");

  test(function () {
    const eventInit = { source: document.body };
    const event = new InterestEvent("test", eventInit);
    assert_equals(event.source, document.body);
  }, "InterestEventInit properties set value (manual event)");

  test(function () {
    const eventInit = {
      source: document.getElementById("div"),
    };
    const event = new InterestEvent("beforetoggle", eventInit);
    assert_equals(event.source, document.getElementById("div"));
  }, "InterestEventInit properties set value (beforetoggle event)");

  test(function () {
    const eventInit = {
      source: document.getElementById("button"),
    };
    const event = new InterestEvent("toggle", eventInit);
    assert_equals(event.source, document.getElementById("button"));
  }, "InterestEventInit properties set value (toggle event)");

  test(function () {
    const event = new InterestEvent("test", { source: undefined });
    assert_equals(event.source, null);
  }, "source set to undefined");

  test(function () {
    const event = new InterestEvent("test", { source: null });
    assert_equals(event.source, null);
  }, "source set to null");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        new InterestEvent("test", { source: false });
      },
      "source is not an object",
    );
  }, "source set to false");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        const event = new InterestEvent("test", { source: true });
      },
      "source is not an object",
    );
  }, "source set to true");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        const event = new InterestEvent("test", { source: {} });
      },
      "source is not an object",
    );
  }, "source set to {}");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        const eventInit = { source: new XMLHttpRequest() };
        const event = new InterestEvent("toggle", eventInit);
      },
      "source is not an Element",
    );
  }, "source set to non-Element EventTarget");
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
  "source_name": "html/semantics/interestfor/interestevent-interface.tentative.html"
}
```
