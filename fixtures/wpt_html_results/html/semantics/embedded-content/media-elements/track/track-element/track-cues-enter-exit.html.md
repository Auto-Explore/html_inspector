# html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-exit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-exit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTrack's cues are indexed and updated in order during video playback</title>
<meta name="timeout" content="long">
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/cues-chrono-order.vtt" kind="captions" default>
    <script>
    // Use the enter and exit events on TextTrackCue.
    async_test(function(t) {
        var video = document.querySelector("video");
        var testTrack = document.querySelector("track");

        video.src = getVideoURI("/media/test");

        video.oncanplaythrough = t.step_func(attemptTests);

        function attemptTests() {
            assert_equals(testTrack.track.cues.length, 3);
            for (var i = 0; i < testTrack.track.cues.length; i++) {
                testTrack.track.cues[i].onenter = t.step_func(cueEntered);
                testTrack.track.cues[i].onexit = t.step_func(cueExited);
            }
            video.play();
        }

        var cueCount = 0;
        function cueEntered(event) {
            var currentCue = event.target;

            // This cue is the currently active cue.
            assert_equals(currentCue, testTrack.track.activeCues[0]);
            assert_equals(currentCue.id, (cueCount + 1).toString());
        }

        function cueExited() {
            ++cueCount;
            if (cueCount == testTrack.track.cues.length)
                t.done();
        }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-exit.html"
}
```
