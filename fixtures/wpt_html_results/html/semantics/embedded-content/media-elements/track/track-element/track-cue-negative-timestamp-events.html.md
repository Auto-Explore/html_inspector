# html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp-events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Enter, Exit events for cues with negative timestamps</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
  <script>
  async_test(function(t) {
    var video = document.querySelector("video");
    var track = video.addTextTrack("subtitles");

    // Add cue with negative startTime.
    var cue = new VTTCue(-10, 1, "Sausage?");
    track.addCue(cue);
    assert_equals(track.cues.length, 1);
    cue.onenter = t.step_func(function() {
      cue.onexit = t.step_func_done();
    });

    // Add cue with negative startTime and negative endTime.
    // This cue should never be active.
    var missedCue = new VTTCue(-110, -3.4, "Pepperoni?");
    track.addCue(missedCue);
    assert_equals(track.cues.length, 2);
    missedCue.onenter = t.unreached_func();
    missedCue.onexit = t.unreached_func();

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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp-events.html"
}
```
