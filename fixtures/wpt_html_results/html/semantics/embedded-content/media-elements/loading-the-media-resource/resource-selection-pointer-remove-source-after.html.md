# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source-after.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source-after.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>pointer updates (removing source element after pointer)</title>
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
 ><source onerror=a++
 ><source onerror=b++ src='resources/delayed-broken-video.py'
 ><source onerror=x1++
 ><source onerror=x2++
 ><source onerror=x3++
 ><source onerror=x4++
 ><source onerror=c++
 ></video
>
<script>
var v;
var t = async_test(function(t) {
  v = document.querySelector('video');
  v.removeChild(document.querySelector('[onerror="x1++"]'));
  window.onload = t.step_func(function() {
    assert_equals(a, 1, 'error events on a');
    assert_equals(b, 1, 'error events on b');
    assert_equals(c, 1, 'error events on c');
    assert_equals(x1, 0, 'error events on x1');
    assert_equals(x2, 0, 'error events on x2');
    assert_equals(x3, 0, 'error events on x3');
    assert_equals(x4, 0, 'error events on x4');
    t.done();
  });
});
</script>
<script>
t.step(function() {
  v.removeChild(document.querySelector('[onerror="x2++"]'));
});
</script>
<script>
t.step(function() {
  v.removeChild(document.querySelector('[onerror="x3++"]'));
});
</script>
<script>
t.step(function() {
  v.removeChild(document.querySelector('[onerror="x4++"]'));
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-remove-source-after.html"
}
```
