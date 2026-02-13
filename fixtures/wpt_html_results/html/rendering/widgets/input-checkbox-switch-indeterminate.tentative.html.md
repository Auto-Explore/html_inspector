# html/rendering/widgets/input-checkbox-switch-indeterminate.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-checkbox-switch-indeterminate.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Checkbox with switch attribute set does not render differently when the indeterminate attribute is set</title>
<link rel=match href="input-checkbox-switch-indeterminate-ref.html">
<input id="input" type=checkbox switch>
<script>
    input.indeterminate = true;
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
  "source_name": "html/rendering/widgets/input-checkbox-switch-indeterminate.tentative.html"
}
```
