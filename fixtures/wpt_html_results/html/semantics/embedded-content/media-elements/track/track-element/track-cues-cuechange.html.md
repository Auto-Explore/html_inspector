# html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange.html",
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
    // Use the cuechange event on TextTrack.
    async_test(function(t) {
        var video = document.querySelector("video");
        var testTrack = document.querySelector("track");

        video.src = getVideoURI("/media/test");
        video.oncanplaythrough = t.step_func(attemptTests);

        function attemptTests() {
            assert_equals(testTrack.track.cues.length, 3);
            testTrack.oncuechange = t.step_func(cueChangedFromTrackElement);
            video.play();
        }

        var currentCueIndex;
        var cueChangeCount = 0;
        function cueChangedFromTrackElement() {
            currentCueIndex = Math.floor(cueChangeCount / 2);
            currentCue = event.target.track.cues[currentCueIndex];
            if (cueChangeCount % 2 == 0) {
                // Cue entered.
                assert_equals(currentCue, testTrack.track.activeCues[0]);
                assert_equals(currentCue.id, (currentCueIndex + 1).toString());
            }

            ++cueChangeCount;
            if (cueChangeCount == testTrack.track.cues.length * 2)
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange.html"
}
```
