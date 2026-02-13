# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-normal-descendant-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-normal-descendant-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Change color-scheme meta tag affecting normal descendant</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta id="meta" name="color-scheme" content="dark">
<div style="color-scheme: dark; color: CanvasText" id="dark">
  <div style="color-scheme: normal; color: CanvasText" id="normal"></div>
</div>
<script>
  test(() => {
    assert_equals(getComputedStyle(dark).color, getComputedStyle(normal).color);
  }, "Normal initially dark");

  meta.content = "light";

  test(() => {
    assert_not_equals(getComputedStyle(dark).color, getComputedStyle(normal).color);
  }, "Normal should change to light from page color schemes");
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-normal-descendant-change.html"
}
```
