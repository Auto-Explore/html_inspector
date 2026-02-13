# html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-01.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-01.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>document.writeln in XHTML</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.writeln%28%29"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<script>
test(function() {
  assert_throws_dom("INVALID_STATE_ERR", function() {
    document.writeln("Failure: document.writeln actually worked");
  }, "document.writeln in XHTML should throw an INVALID_STATE_ERR ");
}, "document.writeln in XHTML");
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-01.xhtml"
}
```
