# html/semantics/forms/the-input-element/range-tick-marks-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-tick-marks-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>LTR range input with datalist</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=841942">
<link rel="author" href="mailto:zach@zrhoffman.net" title="Zach Hoffman">
<link rel="mismatch" href="range-tick-marks-01-notref.html">
<input type="range" min="-100" max="100" value="0" step="10" name="power" list="powers">
<datalist id="powers">
  <option value="0">
  <option value="-30">
  <option value="30">
  <option value="50">
</datalist>
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
  "source_name": "html/semantics/forms/the-input-element/range-tick-marks-01.html"
}
```
