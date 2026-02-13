# html/semantics/embedded-content/media-elements/track/track-element/track-remove-active-cue.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-active-cue.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Removing an active cue</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video></video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");
    video.src = getVideoURI("/media/test");

    // Add a text track to the video element.
    video.addTextTrack("captions", "regular captions track", "en");

    // Add a cue to the track with enter event listener.
    var cue = new VTTCue(0, 4, "Random");
    cue.onenter = t.step_func_done(removeActiveCue);

    var track = video.textTracks[0];
    track.addCue(cue);

    function removeActiveCue() {
        assert_equals(track.activeCues.length, 1);

        // Remove the cue while it is active.
        track.removeCue(track.activeCues[0]);

        // No crash. PASS.
    }

    // Play the video and remove cue when it becomes active.
    video.play();
    track.mode = "showing";
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-active-cue.html"
}
```
