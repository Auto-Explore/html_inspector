# html/semantics/forms/the-textarea-element/selection-after-whitespace-change.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/selection-after-whitespace-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<textarea id="t" style="white-space: nowrap">Hello</textarea>
<script>
promise_test(async () => {
  t.focus();
  if (navigator.userAgent.includes("Mac")) {
    // Meta+ArrowLeft
    await new test_driver.Actions()
      .keyDown("\uE03D")
      .keyDown("\uE058")
      .keyUp("\uE058")
      .keyUp("\uE03D")
      .send();
  } else {
    // Home
    await test_driver.send_keys(t, "\uE011");
  }
  // And then Delete
  await test_driver.send_keys(t, "\uE017");
  t.style.whiteSpace = "pre-line";
  await new Promise(setTimeout);
  assert_equals(t.selectionStart, 0, "selectionStart should remain 0");
  assert_equals(t.selectionEnd, 0, "selectionEnd should remain 0");
}, "Changing white-space should not change selection");
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
  "source_name": "html/semantics/forms/the-textarea-element/selection-after-whitespace-change.html"
}
```
