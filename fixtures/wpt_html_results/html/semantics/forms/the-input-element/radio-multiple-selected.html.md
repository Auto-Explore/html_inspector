# html/semantics/forms/the-input-element/radio-multiple-selected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/radio-multiple-selected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Multiple required input radio elements</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<form id='testForm'>
  <input type=radio name=foo required checked>
  <input type=radio name=foo required>
  <input type=submit>
</form>
<script>
  test(function() {
    assert_true(document.getElementById('testForm').reportValidity());
  }, "Form should be valid since one of the radio elements is checked");
</script>
</body>
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
  "source_name": "html/semantics/forms/the-input-element/radio-multiple-selected.html"
}
```
