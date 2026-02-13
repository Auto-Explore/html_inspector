# html/semantics/forms/textfieldselection/textarea-setRangeText-utf16-code-unit-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/textarea-setRangeText-utf16-code-unit-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>setRangeText: Should not crash when setting text range intersects UTF-16 code units</title>
<link rel="help" href="https://github.com/servo/servo/issues/36719">
<script>
  textarea = document.createElement("textarea");
  textarea.defaultValue = String.fromCodePoint(806453);
  textarea.setRangeText("", 0, 1);
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
  "source_name": "html/semantics/forms/textfieldselection/textarea-setRangeText-utf16-code-unit-crash.html"
}
```
