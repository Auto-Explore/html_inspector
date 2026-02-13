# html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>currentTime</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id=log></div>
<script>
test(function() {
  var v = document.createElement('video');
  assert_equals(v.currentTime, 0);
}, 'currentTime initial value');

test(function() {
  var v = document.createElement('video');
  assert_equals(v.readyState, v.HAVE_NOTHING);
  v.currentTime = Number.MAX_VALUE;
  assert_equals(v.currentTime, Number.MAX_VALUE);
  assert_false(v.seeking);
}, 'setting currentTime when readyState is HAVE_NOTHING');

async_test(function(t) {
  var v = document.createElement('video');
  v.src = getVideoURI('/media/movie_5');
  v.onloadedmetadata = t.step_func(function() {
    assert_greater_than(v.readyState, v.HAVE_NOTHING);
    assert_false(v.seeking);
    v.currentTime = 1;
    assert_true(v.seeking);
    t.done();
  });
}, 'setting currentTime when readyState is greater than HAVE_NOTHING');
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
  "source_name": "html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime.html"
}
```
