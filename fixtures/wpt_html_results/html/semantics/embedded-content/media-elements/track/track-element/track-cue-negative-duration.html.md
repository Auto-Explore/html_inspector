# html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-duration.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-duration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Enter, Exit events for a cue with negative duration</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
  <script>
  async_test(function(t) {
    var video = document.querySelector("video");
    var track = video.addTextTrack("subtitles");

    // Add a cue with negative duration.
    var cue = new VTTCue(1, -10, "Sausage?");
    track.addCue(cue);
    assert_equals(track.cues.length, 1);

    // Verify that enter and exit events are fired.
    var enterEvent = false;
    cue.onenter = t.step_func(function() {
      enterEvent = true;
    });
    cue.onexit = t.step_func_done(function() {
      assert_true(enterEvent);
    });

    video.src = getVideoURI("/media/test");
    video.play();
  });
  </script>
</video>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-duration.html"
}
```
