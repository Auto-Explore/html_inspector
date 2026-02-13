# html/rendering/widgets/input-date-baseline.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-date-baseline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>date input baseline shouldn't be insane</title>
<link rel="match" href="input-date-baseline-ref.html">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1667510">
<link rel="stylesheet" href="/fonts/ahem.css">
<style>
  div {
    border: 1px solid black;
    line-height: 0;
  }

  input {
    height: 20px;
    font: 20px/1 Ahem;
    box-sizing: border-box;
    padding: 1px; /* needed to trigger the bug */
    border: 0;
    visibility: hidden;
    -webkit-appearance: none;
    appearance: none;
  }
</style>
<div>
  <input type="date">
</div>
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
  "source_name": "html/rendering/widgets/input-date-baseline.html"
}
```
