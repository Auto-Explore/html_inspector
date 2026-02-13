# html/browsers/the-window-object/Document-defaultView.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/Document-defaultView.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Document#defaultView</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
test(function() {
  assert_equals(document.defaultView, window);
}, "Document in a browsing context");

test(function() {
  var d = new Document();
  assert_equals(d.defaultView, null);
}, "Document created with the Document constructor");

test(function() {
  var d = document.implementation.createDocument(null, null);
  assert_equals(d.defaultView, null);
}, "Document created with createDocument");

test(function() {
  var d = document.implementation.createHTMLDocument();
  assert_equals(d.defaultView, null);
}, "Document created with createHTMLDocument");

test(function() {
  var parser = new DOMParser();
  var d = parser.parseFromString("<foo\/\>", "application/xml");
  assert_equals(d.defaultView, null);
}, "Document created with XML DOMParser");

test(function() {
  var parser = new DOMParser();
  var d = parser.parseFromString("bar", "text/html");
  assert_equals(d.defaultView, null);
}, "Document created with HTML DOMParser");
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
  "source_name": "html/browsers/the-window-object/Document-defaultView.html"
}
```
