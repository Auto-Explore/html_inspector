# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-insert-text.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-insert-text.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>pointer updates (adding text nodes)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var a = 0;
var b = 0;
var c = 0;
</script>
<video
 ><source onerror=a++
 ><source onerror=b++ src='resources/delayed-broken-video.py'
 ><source onerror=c++
 ></video
>
<script>
async_test(function(t) {
  var video = document.querySelector('video');
  // add text nodes
  var text = document.createTextNode('x');
  video.insertBefore(text, video.querySelector('[onerror="a++"]'));
  video.insertBefore(text.cloneNode(false), video.querySelector('[onerror="b++"]'));
  video.insertBefore(text.cloneNode(false), video.querySelector('[onerror="c++"]'));
  video.appendChild(text.cloneNode(false));
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-pointer-insert-text.html"
}
```
