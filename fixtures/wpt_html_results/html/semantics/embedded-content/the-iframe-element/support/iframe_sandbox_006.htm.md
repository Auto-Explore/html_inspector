# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_006.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_006.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Form submission</title>
</head>
<body>
    <form id="form1" action="standalone-pass.htm">
      <span>Name: </span><input type="text" name="name" value="browser" /><br />
      <input id="submitButton" type="submit" value="Submit Form" />
    </form>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_006.htm"
}
```
