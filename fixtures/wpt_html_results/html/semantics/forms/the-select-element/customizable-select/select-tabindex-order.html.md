# html/semantics/forms/the-select-element/customizable-select/select-tabindex-order.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-tabindex-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<input id="input1" tabindex="1">
<select id="select" tabindex="3">
  <option>one</option>
  <option>two</option>
  <option>three</option>
</select>
<input id="input3" tabindex="2">

<style>
  select, ::picker(select) {
    appearance: base-select;
  }
</style>

<script>
promise_test(async () => {
  const TAB_KEY = "\uE004";

  const input1 = document.getElementById("input1");
  const input3 = document.getElementById("input3");

  input1.focus();
  assert_equals(document.activeElement.id, "input1", "input1 should be active");

  await test_driver.send_keys(input1, TAB_KEY);
  assert_equals(document.activeElement.id, "input3", "input3 should be active");

  await test_driver.send_keys(input3, TAB_KEY);
  assert_equals(document.activeElement.id, "select", "select should be active");
}, "Check that tabindex applies to <select>");
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
        "byte_end": 474,
        "byte_start": 467,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-tabindex-order.html"
}
```
