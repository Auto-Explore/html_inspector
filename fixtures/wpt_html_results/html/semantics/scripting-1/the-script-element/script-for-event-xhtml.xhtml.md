# html/semantics/scripting-1/the-script-element/script-for-event-xhtml.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-for-event-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>Scripts with for and event attributes</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
  <div id="log"></div>
  <script>
    var run = false;
  </script>
  <script for="window" event="bar">
    // This script should not run, but should not cause a parse error either.
    run = true;
  </script>
  <script>
    test(function() {
      assert_false(run, "Script was unexpectedly run.")
    }, "Scripts with for and event attributes should not run.")
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-for-event-xhtml.xhtml"
}
```
