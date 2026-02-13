# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-text.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-text.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>pointer updates (removing text nodes)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var a = 0;
var b = 0;
var c = 0;
</script>
<video
 >x<source onerror=a++
 >x<source onerror=b++ src='resources/delayed-broken-video.py'
 >x<source onerror=c++
 >x</video
>
<script>
async_test(function(t) {
  var video = document.querySelector('video');
  // remove the text nodes
  [].forEach.call(video.childNodes, function(node) {
    if (node.nodeType == node.TEXT_NODE) {
      video.removeChild(node);
    }
  });
  window.onload = t.step_func(function() {
    assert_equals(a, 1, 'error events on a');
    assert_equals(b, 1, 'error events on b');
    assert_equals(c, 1, 'error events on c');
    t.done();
  });
});
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-text.html"
}
```
