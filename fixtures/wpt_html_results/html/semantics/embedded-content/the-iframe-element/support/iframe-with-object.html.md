# html/semantics/embedded-content/the-iframe-element/support/iframe-with-object.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe-with-object.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Display:none iframe with object tag</title>
<script>
  // If this document is loaded as a display:none iframe, forcing an update here
  // means we do not need a style or layout update after the object is inserted
  // below because we are not being rendered.
  document.documentElement.offsetTop;
</script>
<object data="data:text/html,frame"></object>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe-with-object.html"
}
```
