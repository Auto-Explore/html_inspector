# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_003.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_003.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>autofocus on form control</title>
</head>
<body>
    <div>Below form control has autofocus attribute set.</div><br />
    <form action="">
        <span>Textbox: </span><input autofocus="autofocus" type="text" name="movie" />
    </form>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.form.action.empty",
      "message": "Bad value “” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 179,
        "byte_start": 163,
        "col": 5,
        "line": 8
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_003.htm"
}
```
