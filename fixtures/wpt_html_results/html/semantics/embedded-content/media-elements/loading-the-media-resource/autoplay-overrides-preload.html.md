# html/semantics/embedded-content/media-elements/loading-the-media-resource/autoplay-overrides-preload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/autoplay-overrides-preload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>autoplay overrides preload</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id=log></div>
<script>
['none', 'metadata'].forEach(function(preload) {
  ['first', 'last'].forEach(function(order) {
    async_test(function(t) {
      var a = document.createElement('audio');
      a.src = getAudioURI('/media/sound_5');
      if (order == 'first') {
        a.autoplay = true;
        a.preload = preload;
      } else {
        a.preload = preload;
        a.autoplay = true;
      }
      a.addEventListener('error', t.unreached_func());
      a.addEventListener('playing', t.step_func(function() {
        assert_equals(a.readyState, a.HAVE_ENOUGH_DATA);
        assert_false(a.paused);
        t.done();
      }));
    }, 'autoplay (set ' + order + ') overrides preload "' + preload + '"');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/autoplay-overrides-preload.html"
}
```
