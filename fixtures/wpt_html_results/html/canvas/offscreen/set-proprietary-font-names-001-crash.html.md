# html/canvas/offscreen/set-proprietary-font-names-001-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/set-proprietary-font-names-001-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE>
<title>Setting font of offscreen</title>
<script src="/css/css-fonts/support/font-family-keywords.js"></script>
<link rel="help" href="https://drafts.csswg.org/css-fonts-4/#font-family-prop">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1056386">
<script>
  let ctx = (new OffscreenCanvas(1024, 50)).getContext('2d');
  function setFont(keyword) { ctx.font = `12px ${keyword}` };
  kNonGenericFontFamilyKeywords.forEach(setFont);
  kGenericFontFamilyKeywords.forEach(keyword => {
    setFont(`-webkit-${keyword}`);
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.doctype.missing_space_before_name",
      "message": "Missing space before doctype name.",
      "severity": "Warning",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/offscreen/set-proprietary-font-names-001-crash.html"
}
```
