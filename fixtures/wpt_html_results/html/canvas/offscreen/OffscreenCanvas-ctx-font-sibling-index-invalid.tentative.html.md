# html/canvas/offscreen/OffscreenCanvas-ctx-font-sibling-index-invalid.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/OffscreenCanvas-ctx-font-sibling-index-invalid.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>OffscreenCanvas ctx.font should ignore element-dependent calc and preserve previous value</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#dom-context-2d-font">
<link rel="help" href="https://github.com/w3c/csswg-drafts/issues/10982">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(() => {
  var offscreen = new OffscreenCanvas(64, 64);
  var ctx = offscreen.getContext('2d');
  ctx.font = "17px Arial";
  var before = ctx.font;
  ctx.font = "calc(10px * sibling-index()) serif";
  assert_equals(ctx.font, before);
}, 'OffscreenCanvas ctx.font should ignore element-dependent calc and preserve previous value');
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
  "source_name": "html/canvas/offscreen/OffscreenCanvas-ctx-font-sibling-index-invalid.tentative.html"
}
```
