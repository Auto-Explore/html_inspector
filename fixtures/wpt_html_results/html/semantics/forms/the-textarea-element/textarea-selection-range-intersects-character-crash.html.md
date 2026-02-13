# html/semantics/forms/the-textarea-element/textarea-selection-range-intersects-character-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-selection-range-intersects-character-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A selection range that intersects a character should not cause a crash</title>
<link rel="help" href="https://github.com/servo/servo/issues/42217">

<textarea id="textarea">A</textarea>
<script>
  textarea.setSelectionRange(1, 1);
  textarea.defaultValue = String.fromCodePoint(301146);
  textarea.selectionStart;
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-selection-range-intersects-character-crash.html"
}
```
