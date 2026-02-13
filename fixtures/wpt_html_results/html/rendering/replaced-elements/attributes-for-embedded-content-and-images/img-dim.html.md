# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-dim.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-dim.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>img width/height</title>
<link rel=match href=img-dim-ref.html>
<style>
p { width: 50px; height: 50px; }
</style>
<p><img src=/images/green.png>
<p><img src=/images/green.png width=10>
<p><img src=/images/green.png height=10>
<p><img src=/images/green.png width=10%>
<p><img src=/images/green.png height=10%>
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
        "byte_end": 188,
        "byte_start": 161,
        "col": 4,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 228,
        "byte_start": 192,
        "col": 4,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 269,
        "byte_start": 232,
        "col": 4,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.width.bad_value",
      "message": "Bad value “10%” for attribute “width” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 273,
        "col": 4,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 273,
        "col": 4,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.img.height.bad_value",
      "message": "Bad value “10%” for attribute “height” on element “img”.",
      "severity": "Warning",
      "span": {
        "byte_end": 352,
        "byte_start": 314,
        "col": 4,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 352,
        "byte_start": 314,
        "col": 4,
        "line": 12
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-dim.html"
}
```
