# html/semantics/embedded-content/media-elements/error-codes/error.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/error-codes/error.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>error</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>
function error_test(tagName, src) {
  test(function() {
    assert_equals(document.createElement(tagName).error, null);
  }, tagName + '.error initial value');

  async_test(function(t) {
    var e = document.createElement(tagName);
    e.src = src;
    e.onerror = t.unreached_func();
    e.onloadeddata = t.step_func(function() {
      assert_equals(e.error, null);
      t.done();
    });
  }, tagName + '.error after successful load');

  // TODO: MEDIA_ERR_ABORTED, MEDIA_ERR_NETWORK, MEDIA_ERR_DECODE

  async_test(function(t) {
    var e = document.createElement(tagName);
    e.src = '';
    e.onerror = t.step_func(function() {
      assert_true(e.error instanceof MediaError);
      assert_equals(e.error.code, 4);
      assert_equals(e.error.code, e.error.MEDIA_ERR_SRC_NOT_SUPPORTED);
      assert_equals(typeof e.error.message, 'string', 'error.message type');
      t.done();
    });
  }, tagName + '.error after setting src to the empty string');
}

error_test('audio', getAudioURI('/media/sound_5'));
error_test('video', getVideoURI('/media/movie_5'));
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
  "source_name": "html/semantics/embedded-content/media-elements/error-codes/error.html"
}
```
