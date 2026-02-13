# html/semantics/forms/the-input-element/range-tick-marks-05.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-tick-marks-05.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>no range tick marks for range tick marks that are step mismatches</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)">
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1803303">
<link rel=author href="mailto:zach@zrhoffman.net" title="Zach Hoffman">
<link rel=match href=range-tick-marks-05-ref.html>
<input type=range step=3 value=1 min=-5 max=5 list=degrees>
<datalist id=degrees>
  <option value=-4>
  <option value=-2>
  <option value=0>
  <option value=2>
  <option value=4>
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
  "source_name": "html/semantics/forms/the-input-element/range-tick-marks-05.html"
}
```
