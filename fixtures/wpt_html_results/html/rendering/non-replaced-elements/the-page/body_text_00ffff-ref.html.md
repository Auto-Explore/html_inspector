# html/rendering/non-replaced-elements/the-page/body_text_00ffff-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body_text_00ffff-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[body - TEXT=00ffff] Reference file</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<style>
  body {
    color: blue;
  }
</style>
<body>
  <p>This document should have text color 'Blue' using the RGB Hexadecimal color value of "0000ff". </p>
  <p>This test passes if the color of text above matches the image below.</p>
  <p><img src="/images/blue.png"/></p>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 423,
        "byte_start": 394,
        "col": 6,
        "line": 13
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body_text_00ffff-ref.html"
}
```
