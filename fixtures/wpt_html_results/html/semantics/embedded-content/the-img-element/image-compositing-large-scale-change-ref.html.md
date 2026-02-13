# html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change-ref.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<meta name="viewport" content="width=device-width,initial-scale=1,minimum-scale=1">
<style>
html { overflow: hidden; }
#change {
  will-change:transform;
  width:200vw;
  height:200vh;
  position:absolute;
  top: 0px;
  left: 0px;
}
</style>
<img id="change" src="image.png"></img>
<div id="placeholder" style="position:relative">div</div>
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
        "byte_end": 298,
        "byte_start": 265,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “img”.",
      "severity": "Error",
      "span": {
        "byte_end": 304,
        "byte_start": 298,
        "col": 34,
        "line": 15
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
  "source_name": "html/semantics/embedded-content/the-img-element/image-compositing-large-scale-change-ref.html"
}
```
