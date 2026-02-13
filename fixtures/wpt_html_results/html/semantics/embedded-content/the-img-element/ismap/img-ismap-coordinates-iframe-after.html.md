# html/semantics/embedded-content/the-img-element/ismap/img-ismap-coordinates-iframe-after.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/ismap/img-ismap-coordinates-iframe-after.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>img ismap attribute coordinate origin</title>
    <style>
    #bg { background-color: lightgray; position: relative; }
    #target { position: absolute; width: 48px; height: 48px; border: 2px dashed green; pointer-events: none; }
    .after { top: 246px; left: 246px; }
    img { margin: 50px; border: 50px solid white; padding: 50px; }
    </style>
  </head>
  <body>
    <div id="bg">
      <div id="target" class="after"></div>
      <a href="/somewhere/">
        <img src="/images/blue96x96.png" ismap>
      </a>
    </div>
    <h1>Click inside the dashed rectangle</h1>
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
        "byte_end": 550,
        "byte_start": 511,
        "col": 9,
        "line": 16
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
  "source_name": "html/semantics/embedded-content/the-img-element/ismap/img-ismap-coordinates-iframe-after.html"
}
```
