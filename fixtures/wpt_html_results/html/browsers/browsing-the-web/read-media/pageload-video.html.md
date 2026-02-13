# html/browsers/browsing-the-web/read-media/pageload-video.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/read-media/pageload-video.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8">
<title>Media documents: video</title>
<link rel="author" title="Michael Ventnor" href="mailto:mventnor@mozilla.com">
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#read-media">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>
async_test(function() {
  var testframe = document.createElement('iframe');
  var url = getVideoURI("/media/A4");
  var contentType = getMediaContentType(url);
  testframe.onload = this.step_func_done(function() {
    assert_equals(testframe.contentDocument.contentType, contentType);
    assert_equals(testframe.contentDocument.compatMode, "CSS1Compat", "Media documents should be in standards mode");
    var testframeChildren = testframe.contentDocument.body.childNodes;
    assert_equals(testframeChildren.length, 1, "Body of image document has 1 child");
    assert_equals(testframeChildren[0].nodeName, "VIDEO", "Only child of body must be an <video> element");
    assert_equals(testframeChildren[0].namespaceURI, "http://www.w3.org/1999/xhtml",
                  "Only child of body must be an HTML element");
  });
  testframe.src = url;
  document.body.appendChild(testframe);
}, "The document for a standalone media file should have one child in the body.");
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
  "source_name": "html/browsers/browsing-the-web/read-media/pageload-video.html"
}
```
