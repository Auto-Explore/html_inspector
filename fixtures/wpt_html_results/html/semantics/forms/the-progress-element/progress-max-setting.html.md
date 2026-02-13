# html/semantics/forms/the-progress-element/progress-max-setting.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-progress-element/progress-max-setting.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://crbug.com/41496971">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-progress-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
test(function() {
  const progress = document.createElement("progress");
  progress.max = 42;
  assert_equals(progress.max, 42);
  assert_equals(progress.getAttribute("max"), "42");

  // Attempt to set 0
  progress.max = 0;
  assert_equals(progress.max, 42, "Setting max to 0 should be ignored (IDL)");
  assert_equals(progress.getAttribute("max"), "42", "Setting max to 0 should be ignored (content attribute)");

  // Attempt to set negative value
  progress.max = -1000;
  assert_equals(progress.max, 42, "Setting max to -1000 should be ignored (IDL)");
  assert_equals(progress.getAttribute("max"), "42", "Setting max to -1000 should be ignored (content attribute)");

  // Attempt to set another valid value
  progress.max = 10;
  assert_equals(progress.max, 10, "Setting max to 10 should work");
  assert_equals(progress.getAttribute("max"), "10", "Attribute should update to 10");

}, "Setting max to non-positive value should be ignored");
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
  "source_name": "html/semantics/forms/the-progress-element/progress-max-setting.html"
}
```
