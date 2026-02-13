# html/rendering/widgets/field-sizing-placeholder-stretch.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/field-sizing-placeholder-stretch.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Placeholder should stretch input when field-sizing is applied.</title>
<link rel="help" href="https://drafts.csswg.org/css-ui-4/#field-sizing">
<link rel=match href="field-sizing-placeholder-stretch-ref.html">
<link rel="stylesheet" href="/fonts/ahem.css">

<style>
::placeholder {
  font: 30px Ahem;
  color: green;
}
input {
  field-sizing: content;
  padding: 0px;
  border: none;
}
</style>
<input placeholder="PASS if not clipped">
<input style="height: 0px;" placeholder="PASS if fully clipped">
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
  "source_name": "html/rendering/widgets/field-sizing-placeholder-stretch.html"
}
```
