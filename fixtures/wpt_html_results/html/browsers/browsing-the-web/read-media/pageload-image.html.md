# html/browsers/browsing-the-web/read-media/pageload-image.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/read-media/pageload-image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
  <title>Media documents: image</title>
  <link rel="author" title="Michael Ventnor" href="mailto:mventnor@mozilla.com">
  <link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/#read-media">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>

<script>
  var t = async_test("The document for a standalone media file should have one child in the body.");

  function frameLoaded() {
    var testframe = document.getElementById('testframe');
    assert_equals(testframe.contentDocument.contentType, "image/png");
    assert_equals(testframe.contentDocument.compatMode, "CSS1Compat", "Media documents should be in standards mode");
    var testframeChildren = testframe.contentDocument.body.childNodes;
    assert_equals(testframeChildren.length, 1, "Body of image document has 1 child");
    assert_equals(testframeChildren[0].nodeName, "IMG", "Only child of body must be an <img> element");
    assert_equals(testframeChildren[0].namespaceURI, "http://www.w3.org/1999/xhtml",
                  "Only child of body must be an HTML element");
    t.done();
  }
</script>
</head>
<body>
  <div id="log"></div>
  <iframe id="testframe" onload="t.step(frameLoaded)" src="/images/blue.png"></iframe>
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
  "source_name": "html/browsers/browsing-the-web/read-media/pageload-image.html"
}
```
