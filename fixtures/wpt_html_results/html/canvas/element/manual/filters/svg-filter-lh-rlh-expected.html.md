# html/canvas/element/manual/filters/svg-filter-lh-rlh-expected.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/svg-filter-lh-rlh-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="test-wait">
<title>HTML Canvas reference: SVG filter using CSS font-relative line-height units</title>
<link rel="stylesheet" type="text/css" href="/fonts/ahem.css" />
<style>
:root {
  font: 20px Ahem;
}

div {
  display: inline-block;
  width: 100px;
  height: 100px;
  background: green;
}

.r {
  background: red;
}
</style>
<div><div class="r" style="width: 48px;"></div></div><br>
<div><div class="r" style="width: 24px;"></div></div>
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
  "source_name": "html/canvas/element/manual/filters/svg-filter-lh-rlh-expected.html"
}
```
