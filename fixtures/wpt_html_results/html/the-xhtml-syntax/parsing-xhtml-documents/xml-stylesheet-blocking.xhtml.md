# html/the-xhtml-syntax/parsing-xhtml-documents/xml-stylesheet-blocking.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/xml-stylesheet-blocking.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/css" href="support/simple-style.css?pipe=trickle(d2)"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <head>
    <script><![CDATA[
      window.observedZIndex = getComputedStyle(document.documentElement).zIndex;
    ]]></script>
    <title>xml-stylesheet blocks script execution and rendering</title>
    <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1986042" />
    <link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io" />
    <link rel="author" title="Mozilla" href="https://mozilla.org" />
    <script src="/resources/testharness.js"/>
    <script src="/resources/testharnessreport.js"/>
  </head>
  <body>
    <script><![CDATA[
    test(function() {
      assert_equals(window.observedZIndex, "3", "XML processing instruction should've blocked script execution and rendering");
    });
    ]]></script>
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/xml-stylesheet-blocking.xhtml"
}
```
