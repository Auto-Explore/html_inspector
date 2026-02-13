# html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed-no-immediate-events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed-no-immediate-events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Adding a missed cue during playback should not fire events</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
</video>
<script>
async_test(t => {
    const video = document.querySelector("video");
    const track = video.addTextTrack("subtitles");
    let cueAdded = false;

    video.ontimeupdate = t.step_func(() => {
        // After 0.3s, add a cue that is completely before currentTime.
        if (!cueAdded && video.currentTime > 0.3) {
            cueAdded = true;
            let missedCue = new VTTCue(0.1, 0.3, "Test");
            missedCue.onenter = t.unreached_func("onenter for missed cue should not fire");
            missedCue.onexit = t.unreached_func("onexit for missed cue should not fire");
            track.addCue(missedCue);
        }

        // We should play past 1s without the events firing.
        if (video.currentTime > 1.0) {
            video.ontimeupdate = null;
            t.done();
        }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed-no-immediate-events.html"
}
```
