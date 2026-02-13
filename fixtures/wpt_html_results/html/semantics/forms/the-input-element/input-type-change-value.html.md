# html/semantics/forms/the-input-element/input-type-change-value.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-type-change-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Input type switch from / to color</title>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://html.spec.whatwg.org/#input-type-change">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1833477">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
function runTest(focus) {
  let input = document.createElement("input");
  input.type = "color";
  document.body.appendChild(input);
  if (focus) {
    input.focus();
  }
  assert_equals(input.value, "#000000", "Invalid color should return a non-empty sanitized value");
  input.type = "text";
  assert_equals(input.value, "", "Value dirty flag should remain false");
  input.type = "color";
  input.value = "#ffffff";
  assert_equals(input.value, "#ffffff", "Valid color is returned");
  input.type = "text";
  assert_equals(input.value, "#ffffff", "Value dirty flag should remain true");
  if (focus) {
    assert_equals(document.activeElement, input, "Focus is preserved");
  }
}
test(() => runTest(false), "Without focus");
test(() => runTest(true), "With focus");
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
  "source_name": "html/semantics/forms/the-input-element/input-type-change-value.html"
}
```
