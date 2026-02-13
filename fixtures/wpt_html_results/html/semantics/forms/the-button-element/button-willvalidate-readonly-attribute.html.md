# html/semantics/forms/the-button-element/button-willvalidate-readonly-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-willvalidate-readonly-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Button element with "readonly" attribute shouldn't be barred from constraint validation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id="implicitSubmitButton" readonly>1</button>
<button id="explicitSubmitButton" readonly type="submit">2</button>
<script>
  test(() => {
    assert_true(implicitSubmitButton.willValidate);
    assert_true(explicitSubmitButton.willValidate);
  });
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
  "source_name": "html/semantics/forms/the-button-element/button-willvalidate-readonly-attribute.html"
}
```
