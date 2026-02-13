# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>pointer updates (removing source elements)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var a = 0;
var b = 0;
var c = 0;
var x1 = 0;
var x2 = 0;
var x3 = 0;
var x4 = 0;
</script>
<video
 ><source onerror=x1++
 ><source onerror=a++
 ><source onerror=x2++
 ><source onerror=b++ src='resources/delayed-broken-video.py'
 ><source onerror=x3++
 ><source onerror=c++
 ><source onerror=x4++
 ></video
>
<script>
async_test(function(t) {
  var video = document.querySelector('video');
  // remove the xn elements
  [].forEach.call(document.querySelectorAll('[onerror^="x"]'), function(elm) {
    video.removeChild(elm);
  });
  window.onload = t.step_func(function() {
    assert_equals(a, 1, 'error events on a');
    assert_equals(b, 1, 'error events on b');
    assert_equals(c, 1, 'error events on c');
    assert_equals(x1, 1, 'error events on x1');
    assert_equals(x2, 1, 'error events on x2');
    assert_equals(x3, 0, 'error events on x3');
    assert_equals(x4, 0, 'error events on x4');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source.html"
}
```
