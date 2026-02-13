# html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/bailout-order-xml-with-synchronous-script-frame.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/bailout-order-xml-with-synchronous-script-frame.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
  <head><title>XHTML document with hook to run script from a script tag</title></head>
  <body>
    <p>Text</p>
    <script>
      parent.testSynchronousScript();
    </script>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/resources/bailout-order-xml-with-synchronous-script-frame.xhtml"
}
```
