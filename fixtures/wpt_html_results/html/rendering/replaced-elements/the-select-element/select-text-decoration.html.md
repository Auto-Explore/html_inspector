# html/rendering/replaced-elements/the-select-element/select-text-decoration.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-text-decoration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
  <meta charset="utf-8">
  <title>Text decoration should not propagate into select elements by default</title>
  <link rel="author" title="Simon Wülker" href="mailto:simon.wuelker@arcor.de">
  <link rel="match" href="select-text-decoration-ref.html">
  <link rel="help" href="https://github.com/servo/servo/issues/37895">

  <style>
    body {
        color: red;
        text-decoration: underline;
    }
  </style>
</head>
<body>
<select>
    <option>Foobar</option>
</select>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/the-select-element/select-text-decoration.html"
}
```
