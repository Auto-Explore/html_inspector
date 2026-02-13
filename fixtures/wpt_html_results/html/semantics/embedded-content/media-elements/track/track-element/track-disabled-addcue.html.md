# html/semantics/embedded-content/media-elements/track/track-element/track-disabled-addcue.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-disabled-addcue.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Adding cues to a disabled text track</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
    var cueDuration = 0.1;
    var video = document.createElement("video");
    var track = video.addTextTrack("subtitles");
    track.mode = "disabled";

    for (var i = 0; i < 10; ++i) {
        var start = i * cueDuration;
        var end = start + cueDuration;
        track.addCue(new VTTCue(start, end, "Test Cue " + i));
    }

    // Waiting for 2 cue durations to elapse.
    video.ontimeupdate = t.step_func(function(event) {
        if (event.target.currentTime < (2 * cueDuration))
            return;

        // End test after at least 2 cueDurations to make sure the test
        // would have gone through the period where the first 2 cues would
        // have been rendered if the track was not disabled.
        t.done();
    });

    video.src = getVideoURI("/media/test");
    video.play();
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-disabled-addcue.html"
}
```
