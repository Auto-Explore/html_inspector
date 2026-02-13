# html/semantics/embedded-content/media-elements/playing-the-media-resource/pause-move-to-other-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/playing-the-media-resource/pause-move-to-other-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>paused state when moving to other document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<video hidden></video>
<iframe hidden></iframe>
<script>
async_test(function(t) {
  var v = document.querySelector('video');
  v.src = getVideoURI('/media/movie_300');
  v.play();
  v.onplaying = t.step_func(function() {
    assert_false(v.paused, 'paused after playing');
    document.querySelector('iframe').contentDocument.body.appendChild(v);
    assert_false(v.paused, 'paused after moving');
    t.step_timeout(function() {
      assert_false(v.paused, 'paused after stable state')
      t.done();
    }, 0);
  });
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
  "source_name": "html/semantics/embedded-content/media-elements/playing-the-media-resource/pause-move-to-other-document.html"
}
```
