# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
<style>
body {
  margin: 0;
}

.fieldset div {
  height:1000px;
}

span {
  background: lime;
  display: block;
  height: 40px;
  position: absolute;
  top: 4px;
  left: 0px;
  width: 40px;
}

.fieldset {
  border: none;
  height: 400px;
  margin: 0;
  overflow: scroll;
  padding: 0;
}
</style>
<div class="fieldset"><div><span></span></div></div>
<script>
document.querySelector('.fieldset').scrollTop = 1000;
</script>
</body>
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
        "byte_end": 30,
        "byte_start": 23,
        "col": 1,
        "line": 3
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content-ref.html"
}
```
