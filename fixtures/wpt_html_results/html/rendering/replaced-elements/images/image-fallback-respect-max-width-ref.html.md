# html/rendering/replaced-elements/images/image-fallback-respect-max-width-ref.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/images/image-fallback-respect-max-width-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Image fallback respect max-width</title>
<style>
  div {
    max-width: 200px;
  }
  img {
    width: 200px;
    height: auto;
  }
</style>
<body>
  <div>
    <img src="" width="10000" height="100">
    <img src="" width="10000" height="100"
      alt="Lorem ipsum dolor sit amet, consectetur adipisicing elit.">
    <img src="" width="1000" height="100">
  </div>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 244,
        "byte_start": 205,
        "col": 5,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 244,
        "byte_start": 205,
        "col": 5,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 358,
        "byte_start": 249,
        "col": 5,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.src.empty",
      "message": "Bad value “” for attribute “src” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 401,
        "byte_start": 363,
        "col": 5,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 401,
        "byte_start": 363,
        "col": 5,
        "line": 18
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
  "source_name": "html/rendering/replaced-elements/images/image-fallback-respect-max-width-ref.html"
}
```
