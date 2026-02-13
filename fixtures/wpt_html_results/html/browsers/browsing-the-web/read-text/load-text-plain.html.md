# html/browsers/browsing-the-web/read-text/load-text-plain.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/read-text/load-text-plain.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Page load processing model for text files</title>
<link rel="author" title="Ms2ger" href="ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#read-text">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var t = async_test("Checking document metadata for text file");
var tD = async_test("Checking DOM for text file");
var tC = async_test("Checking contents for text file");
var iframe = document.body.appendChild(document.createElement("iframe"));
iframe.onload = function(e) {
  var doc = iframe.contentDocument;
  t.step(function() {
    assert_equals(doc.compatMode, "CSS1Compat");
    assert_equals(doc.contentType, "text/plain");
    assert_equals(doc.doctype, null);
    t.done();
  })
  tD.step(function() {
    assert_equals(doc.childNodes.length, 1, "Document should have 1 child")
    assert_equals(doc.documentElement.tagName, "HTML");
    assert_equals(doc.documentElement.childNodes.length, 2,
                  "Root element should have 2 children")
    assert_equals(doc.documentElement.firstChild.tagName, "HEAD");
    assert_equals(doc.documentElement.lastChild.tagName, "BODY");
    assert_equals(doc.documentElement.lastChild.childNodes.length, 1,
                  "Body element should have 1 child")
    assert_equals(doc.documentElement.lastChild.firstChild.tagName, "PRE");
    tD.done();
  })
  tC.step(function() {
    assert_equals(doc.documentElement.lastChild.firstChild.firstChild.data,
                  "This is a sample text/plain document.\n\nThis is not an HTML document.\n\n");
    tC.done();
  })
};
iframe.src = "../../../../common/text-plain.txt";
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
  "source_name": "html/browsers/browsing-the-web/read-text/load-text-plain.html"
}
```
