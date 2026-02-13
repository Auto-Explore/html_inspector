# html/semantics/embedded-content/media-elements/track/track-element/track-mode-triggers-loading.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-triggers-loading.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A "metadata" track does not load automatically, but it does load when the mode is changed</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track kind="metadata" src="resources/metadata.vtt">
</video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");

    // Check initial metadata track state.
    var track = document.querySelectorAll("track")[0];
    assert_equals(track.readyState, HTMLTrackElement.NONE);
    assert_equals(video.textTracks[0].mode, "disabled");

    video.src = getVideoURI("/media/test");
    video.oncanplaythrough = t.step_func(canplaythrough);
    track.onload = t.step_func_done(trackLoaded);

    function trackLoaded() {
        assert_equals(track.readyState, HTMLTrackElement.LOADED);
        assert_equals(track.track.mode, "hidden");
        assert_equals(video.textTracks[0].cues.length, 12);
        assert_equals(video.textTracks[0].cues[11].startTime, 22);
    }

    function canplaythrough() {
        assert_equals(track.readyState, HTMLTrackElement.NONE);
        assert_equals(video.textTracks[0].mode, "disabled");
        assert_equals(video.textTracks[0].cues, null);
        // Change metadata track mode so it loads.
        video.textTracks[0].mode = "hidden";
    }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-triggers-loading.html"
}
```
