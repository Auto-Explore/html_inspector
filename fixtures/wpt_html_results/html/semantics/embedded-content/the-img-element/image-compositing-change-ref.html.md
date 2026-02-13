# html/semantics/embedded-content/the-img-element/image-compositing-change-ref.html

Counts:
- errors: 2
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-compositing-change-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<style>
#change {
  height:75px;
  width:75px;
}
</style>
<img id="change" src="/images/green-16x16.png"></img>
<img src="/images/green-16x16.png"></img>
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
        "byte_end": 128,
        "byte_start": 81,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 134,
        "byte_start": 128,
        "col": 48,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 170,
        "byte_start": 135,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 176,
        "byte_start": 170,
        "col": 36,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/image-compositing-change-ref.html"
}
```
