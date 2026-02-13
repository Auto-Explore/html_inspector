# html/semantics/embedded-content/media-elements/location-of-the-media-resource/currentSrc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/location-of-the-media-resource/currentSrc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>currentSrc</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
['audio', 'video'].forEach(function(tagName) {
  test(function() {
    assert_equals(document.createElement(tagName).currentSrc, '');
  }, tagName + '.currentSrc initial value');

  ['', '.', ' ', 'data:,'].forEach(function(src) {
    async_test(function(t) {
      var e = document.createElement(tagName);
      e.src = src;
      assert_equals(e.currentSrc, '');
      e.addEventListener('loadstart', function () {
        t.step_timeout(function () {
          if (src == '') {
            assert_equals(e.currentSrc, '');
          } else {
            assert_equals(e.currentSrc, e.src);
          }
          t.done();
        }, 0);
      })
    }, tagName + '.currentSrc after setting src attribute "' + src + '"');

    async_test(function(t) {
      var e = document.createElement(tagName);
      var s = document.createElement('source');
      s.src = src;
      e.appendChild(s);
      assert_equals(e.currentSrc, '');
      e.addEventListener('loadstart', function() {
        t.step_timeout(function () {
          if (src == '') {
            assert_equals(e.currentSrc, '');
          } else {
            assert_equals(e.currentSrc, s.src);
          }
          t.done();
        }, 0);
      });
    }, tagName + '.currentSrc after adding source element with src attribute "' + src + '"');
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
  "source_name": "html/semantics/embedded-content/media-elements/location-of-the-media-resource/currentSrc.html"
}
```
