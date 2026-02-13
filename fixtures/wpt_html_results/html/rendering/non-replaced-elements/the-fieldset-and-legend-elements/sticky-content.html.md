# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="http://crbug.com/1146925">
<link rel="match" href="sticky-content-ref.html">
<body>
<style>
body {
  margin: 0;
}

fieldset div {
  height:1000px;
}

span {
  background: lime;
  display: block;
  height: 40px;
  position: sticky;
  top: 4px;
  width: 40px;
}

fieldset {
  border: none;
  height: 400px;
  margin: 0;
  overflow: scroll;
  padding: 0;
}
</style>
<fieldset><div><span></span></div></fieldset>
<script>
document.querySelector('fieldset').scrollTop = 1000;
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
        "byte_end": 130,
        "byte_start": 123,
        "col": 1,
        "line": 5
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/sticky-content.html"
}
```
