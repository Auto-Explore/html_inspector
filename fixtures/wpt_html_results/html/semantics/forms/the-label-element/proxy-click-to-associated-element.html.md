# html/semantics/forms/the-label-element/proxy-click-to-associated-element.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/proxy-click-to-associated-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>label element click proxying via "for" attribute or nested labelable element</title>
<link rel="author" title="yaycmyk" href="mailto:evan@yaycmyk.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#the-label-element:the-label-element-10">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<form id="test">
    <input id="foo" type="checkbox" />
    <label id="foo-label" for="foo">foo</label>

    <label id="bar-label">
        <input id="bar" type="checkbox" /> bar
        <input id="baz" type="checkbox" /> baz
    </label>

    <input id="baz" type="checkbox" />
    <label id="baz-label" for="baz">baz</label>
</form>
<script>
  "use strict";

  async_test(t => {
    const label = document.getElementById("foo-label");
    const input = document.getElementById("foo");

    input.addEventListener("click", t.step_func_done());

    label.click();

  }, "label with for attribute should proxy click events to the associated element");

  async_test(t => {
    const label = document.getElementById("bar-label");
    const input = document.getElementById("bar");

    input.addEventListener("click", t.step_func_done());

    label.click();

  }, "label without for attribute should proxy click events to the first labelable child");

  async_test(t => {

    const label = document.getElementById("baz-label");
    const input = document.getElementById("baz");

    input.addEventListener("click", t.unreached_func("Input should not receive click"));
    label.addEventListener("click", t.step_func(ev => {
      ev.preventDefault();
      t.step_timeout(() => t.done(), 500);
    }));

    label.click();

  }, "clicking a label that prevents the event's default should not proxy click events");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.label.multiple_controls",
      "message": "The “label” element may contain at most one “button”, “input”, “meter”, “output”, “progress”, “select”, or “textarea” descendant.",
      "severity": "Error",
      "span": {
        "byte_end": 637,
        "byte_start": 603,
        "col": 9,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “baz”.",
      "severity": "Error",
      "span": {
        "byte_end": 694,
        "byte_start": 660,
        "col": 5,
        "line": 17
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-label-element/proxy-click-to-associated-element.html"
}
```
