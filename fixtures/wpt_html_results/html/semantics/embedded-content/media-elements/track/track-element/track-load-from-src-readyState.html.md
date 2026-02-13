# html/semantics/embedded-content/media-elements/track/track-element/track-load-from-src-readyState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-load-from-src-readyState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Load event on HTMLTrackElement and LOADED readyState on TextTrack when src is set from JavaScript</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track>
</video>
<script>
async_test(function(t) {
    var track = document.querySelector("track");
    assert_equals(track.readyState, HTMLTrackElement.NONE);

    track.onload = t.step_func_done(function() {
        assert_equals(track.readyState, HTMLTrackElement.LOADED);
    });

    track.src = "resources/webvtt-file.vtt";
    track.track.mode = "hidden";
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-load-from-src-readyState.html"
}
```
