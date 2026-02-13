# html/semantics/forms/the-input-element/range-list-added-repaint.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-list-added-repaint.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class=reftest-wait>
<title>The range is repainted if a list ID is added</title>
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1805105">
<link rel=author href="mailto:zach@zrhoffman.net" title="Zach Hoffman">
<link rel=match href=range-tick-marks-05-ref.html>
<script src=/common/reftest-wait.js></script>
<input type=range step=3 value=1 min=-5 max=5>
<datalist id=tickmarks>
  <option value=4>
  <option value=-2>
</datalist>
<script>
  requestAnimationFrame(() =>
    requestAnimationFrame(() => {
      document.querySelector("input[type=range]").setAttribute("list", "tickmarks");
      takeScreenshot();
    }));
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
  "source_name": "html/semantics/forms/the-input-element/range-list-added-repaint.html"
}
```
