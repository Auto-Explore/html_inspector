# html/semantics/embedded-content/media-elements/playing-the-media-resource/playbackRate.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/playing-the-media-resource/playbackRate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>playbackRate</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  var v = document.createElement('video');
  assert_equals(v.playbackRate, 1);
}, 'playbackRate initial value');

function testPlaybackRateHelper(t, newPlaybackRate) {
  var v = document.createElement('video');
  var initialRate = v.playbackRate;

  v.addEventListener('ratechange', t.step_func_done(function() {
    assert_equals(v.playbackRate, newPlaybackRate);
  }));

  try {
    v.playbackRate = newPlaybackRate;
  } catch(e) {
    assert_equals(e.name, 'NotSupportedError');
    assert_equals(v.playbackRate, initialRate);
    t.done();
  }
}

async_test(function(t) {
  testPlaybackRateHelper(this, 3);
}, "playbackRate set to small positive value");

async_test(function(t) {
  testPlaybackRateHelper(this, 100);
}, "playbackRate set to large positive value");

async_test(function(t) {
  testPlaybackRateHelper(this, -3);
}, "playbackRate set to small negative value");

async_test(function(t) {
  testPlaybackRateHelper(this, -100);
}, "playbackRate set to large negative value");

async_test(function(t) {
  testPlaybackRateHelper(this, 0);
}, "playbackRate set to 0");

async_test(function(t) {
  testPlaybackRateHelper(this, -1);
}, "playbackRate set to -1");

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
  "source_name": "html/semantics/embedded-content/media-elements/playing-the-media-resource/playbackRate.html"
}
```
