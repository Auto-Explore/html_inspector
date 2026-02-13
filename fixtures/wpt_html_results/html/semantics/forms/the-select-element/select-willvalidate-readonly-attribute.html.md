# html/semantics/forms/the-select-element/select-willvalidate-readonly-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-willvalidate-readonly-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Select element with "readonly" attribute shouldn't be barred from constraint validation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<select id="singleSelect" readonly>
  <option>1
  <option>2
</select>

<select id="multiSelect" readonly multiple>
  <option>a
  <option>b
  <option>c
  <option>d
</select>

<script>
  test(() => {
    assert_true(singleSelect.willValidate);
    assert_true(multiSelect.willValidate);
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
  "source_name": "html/semantics/forms/the-select-element/select-willvalidate-readonly-attribute.html"
}
```
