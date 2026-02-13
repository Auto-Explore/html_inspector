# html/semantics/forms/the-input-element/disabled-attempt-focus-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/disabled-attempt-focus-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>Clicking in a disabled input type=text field should not cause a crash</title>
<link rel="help" href="https://github.com/servo/servo/issues/42074">
<head>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
</head>

<input id=target type=text value="text" disabled>

<script>
    async function test() {
      await new test_driver.Actions()
        .pointerMove(5, 5, { origin: target })
        .pointerDown()
        .pointerUp()
        .send();
    }
    test();
</script>

</html>
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
  "source_name": "html/semantics/forms/the-input-element/disabled-attempt-focus-crash.html"
}
```
