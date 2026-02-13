# html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.writeln with multiple arguments</title>
<link rel="author" title="Sebmaster" href="mailto:wpt@smayr.name">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-writeln%28%29">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#documents-in-the-dom">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var iframe = document.createElement("iframe");
  document.body.appendChild(iframe);
  var doc = iframe.contentDocument;
  doc.open();
  doc.writeln('a', 'b');
  doc.close();
  assert_equals(doc.documentElement.textContent, "ab\n");
}, "Calling document.writeln with multiple arguments");
</script>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-writeln/document.writeln-03.html"
}
```
