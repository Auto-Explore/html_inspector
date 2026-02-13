# html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-expected.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <title></title>
  <style type="text/css">
    div {
      width: 300px;
      height: 150px;
      background: conic-gradient(
        from 90deg at 100px 50px,
        red 0.2turn,
        orange 0.2turn 0.4turn,
        yellow 0.4turn 0.6turn,
        green 0.6turn 0.8turn,
        blue 0.8turn 1.0turn
      );
  </style>
</head>
<body>
  <div id="output"></div>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 39,
        "byte_start": 32,
        "col": 3,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 73,
        "byte_start": 50,
        "col": 3,
        "line": 5
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
  "source_name": "html/canvas/element/manual/fill-and-stroke-styles/conic-gradient-expected.html"
}
```
