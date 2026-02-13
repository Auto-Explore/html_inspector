# html/semantics/forms/the-input-element/checkbox-click-events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/checkbox-click-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Checkbox click events</title>
<link rel="author" title="jeffcarp" href="mailto:gcarpenterv@gmail.com">
<link rel=help href="https://html.spec.whatwg.org/#checkbox-state-(type=checkbox)">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {

  const input = document.createElement("input");
  input.type = "checkbox";

  const values = [];

  input.addEventListener("click", e => {
    values.push(input.checked);
    e.preventDefault();
    values.push(input.checked);
  });

  input.click();

  values.push(input.checked);
  assert_array_equals(values, [true, true, false]);

}, "clicking and preventDefaulting a checkbox causes the checkbox to be checked during the click handler but reverted");

test(() => {
  const input = document.createElement("input");
  input.type = "checkbox";
  document.body.appendChild(input);
  const events = [];

  input.addEventListener("change", () => {
    events.push("change");
  });
  input.addEventListener("click", () => {
    events.push("click");
  });
  input.addEventListener("input", () => {
    events.push("input");
  });

  assert_false(input.checked);

  input.click();

  assert_true(input.checked);
  assert_array_equals(events, ["click", "input", "change"]);

}, "a checkbox input emits click, input, change events in order after synthetic click");

test(() => {
  const input = document.createElement("input");
  input.type = "checkbox";
  document.body.appendChild(input);
  const events = [];

  input.addEventListener("change", () => {
    events.push("change");
  });
  input.addEventListener("click", () => {
    events.push("click");
  });
  input.addEventListener("input", () => {
    events.push("input");
  });

  assert_false(input.checked);

  const event = new MouseEvent("click", { bubbles: true, cancelable: true });
  input.dispatchEvent(event);

  assert_true(input.checked);
  assert_array_equals(events, ["click", "input", "change"]);

}, "a checkbox input emits click, input, change events in order after dispatching click event");

test(() => {
  const input = document.createElement("input");
  input.type = "checkbox";
  document.body.appendChild(input);
  const events = [];

  input.addEventListener("change", () => {
    events.push("change");
  });
  input.addEventListener("click", e => {
    e.preventDefault();
    events.push("click");
  });
  input.addEventListener("input", () => {
    events.push("input");
  });

  assert_false(input.checked);

  input.click();

  assert_false(input.checked);
  assert_array_equals(events, ["click"]);
}, "checkbox input respects cancel behavior on synthetic clicks");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/checkbox-click-events.html"
}
```
