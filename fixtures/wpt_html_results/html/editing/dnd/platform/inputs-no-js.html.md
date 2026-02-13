# html/editing/dnd/platform/inputs-no-js.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/inputs-no-js.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dropping selections with JS disabled</title>
  </head>
  <body>

    <ol>
      <li>Disable JavaScript</li>
      <li>Select some text in <input type="text" value="this input"> and drag it into the following input: <input type="text" value=""> - the text you dragged should appear in there.</li>
      <li>Select some text in this sentence and drag it into the following input: <input type="text" value=""> - the text you dragged should appear in there.</li>
    </ol>

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
  "source_name": "html/editing/dnd/platform/inputs-no-js.html"
}
```
