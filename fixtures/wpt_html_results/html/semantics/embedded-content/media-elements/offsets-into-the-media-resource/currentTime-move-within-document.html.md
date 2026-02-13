# html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime-move-within-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime-move-within-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>playback should not reset when moving within a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<video autoplay muted hidden></video>
<div id="elsewhere"></div>
<script>
async_test(t => {
  const v = document.querySelector('video');
  v.src = getVideoURI('/media/movie_300');
  v.currentTime  = 0;
  v.onplaying = t.step_func(() => {
    v.currentTime = 10;
    t.step_timeout(() => {
      assert_greater_than_equal(v.currentTime, 10);
      document.getElementById('elsewhere').appendChild(v);
      assert_false(v.paused, 'paused after moving');
      assert_greater_than_equal(v.currentTime, 10);
      t.step_timeout(() => {
        assert_greater_than_equal(v.currentTime, 10);
        t.done();
      }, 0);
    }, 0);

  });
  v.play();
  v.onpause = t.step_func(function() { assert_unreached(); });
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
  "source_name": "html/semantics/embedded-content/media-elements/offsets-into-the-media-resource/currentTime-move-within-document.html"
}
```
