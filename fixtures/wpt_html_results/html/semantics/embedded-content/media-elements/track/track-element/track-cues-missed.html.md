# html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Events are triggered for missed (skipped) cues during normal playback</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/missed-cues.vtt" default>
    <script>
    async_test(function(t) {
        var video = document.querySelector("video");
        var testTrack = document.querySelector("track");

        video.src = getVideoURI("/media/test");

        video.onended = t.step_func_done();

        video.oncanplaythrough = t.step_func(function() {
            video.oncanplaythrough = null;
            video.currentTime = 5.00;
            runTests();
        });

        testTrack.onload = t.step_func(runTests);

        var cueCount;
        var eventCount = 0;
        function runTests() {
            eventCount++;

            if(eventCount != 2)
                return;

            assert_equals(testTrack.track.cues.length, 7);

            for (cueCount = 2; cueCount < testTrack.track.cues.length; cueCount++) {
                var cue = testTrack.track.cues[cueCount];

                cue.onenter = t.step_func(cueEnteredOrExited);
                cue.onexit = t.step_func(cueEnteredOrExited);
            }

            // Test events for missed cues, which are cues with ids
            // from 3 to 7 in the file resources/missed-cues.vtt.
            cueCount = 3;
            video.play();
        }

        function cueEnteredOrExited(event) {
            var currentCue = event.target;
            assert_equals(testTrack.track.cues.getCueById(cueCount).text, currentCue.text);
            assert_equals(currentCue.id, cueCount.toString());

            if (event.type == "exit")
                cueCount++;
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-missed.html"
}
```
