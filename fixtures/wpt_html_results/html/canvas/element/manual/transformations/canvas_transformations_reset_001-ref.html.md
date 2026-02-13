# html/canvas/element/manual/transformations/canvas_transformations_reset_001-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/transformations/canvas_transformations_reset_001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<body>
<style>
  html, body, div {
    margin: 0;
    padding: 0;
  }
  div {
    width: 75px;
    height: 75px;
    float: left;
  }
</style>

<div style="background-color:red"></div>
<div style="clear:left"></div>
<div style="background-color:blue"></div>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 37,
        "byte_start": 30,
        "col": 1,
        "line": 4
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
  "source_name": "html/canvas/element/manual/transformations/canvas_transformations_reset_001-ref.html"
}
```
