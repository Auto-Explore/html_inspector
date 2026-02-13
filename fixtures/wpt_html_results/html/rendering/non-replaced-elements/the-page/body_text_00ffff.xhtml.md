# html/rendering/non-replaced-elements/the-page/body_text_00ffff.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/body_text_00ffff.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns='http://www.w3.org/1999/xhtml'>
<head>
<title>body - TEXT=00ffff</title>
<link rel="match" href="body_text_00ffff-ref.html"/>
<meta name="assert" content="Test checks that User Agent requirement as per HTML5 spec NOT the author requirement."/>
</head>
<body text="0000ff">
<p>This document should have text color 'Blue' using the RGB Hexadecimal color value of "0000ff". </p>
<p>This test passes if the color of text above matches the image below.</p>
<p><img src="/images/blue.png" /></p>
</body>
</html>
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
        "byte_end": 497,
        "byte_start": 467,
        "col": 4,
        "line": 10
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
  "source_name": "html/rendering/non-replaced-elements/the-page/body_text_00ffff.xhtml"
}
```
