# html/semantics/popovers/popover-and-svg.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-and-svg.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover is only effective on HTMLElement, not on svg element</title>
<link rel="author" href="mailto:cathiechen@igalia.com">
<link rel=help href="https://html.spec.whatwg.org/#the-popover-attribute">
<link rel="match" href="popover-and-svg-ref.html">
<style>
svg {
  width: 100px;
  height: 100px;
  background-color:green;
}
[popover] {
  top: 100px;
  bottom: auto;
}
</style>
<svg popover></svg>
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
  "source_name": "html/semantics/popovers/popover-and-svg.html"
}
```
