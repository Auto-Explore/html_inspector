# html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Negative timestamps</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/settings.vtt" default>
    <script>
    async_test(function(t) {
        var testTrack = document.querySelector("track");

        testTrack.onload = t.step_func_done(function() {
            var cues = testTrack.track.cues;
            assert_equals(testTrack.track.cues.length, 4);
            // Add cue with negative startTime.
            var cue = new VTTCue(-3439332606, 3.4, "Sausage?");
            testTrack.track.addCue(cue);
            assert_equals(cues.length, 5);

            // Add cue with negative startTime and negative endTime.
            cue = new VTTCue(-110, -3.4, "Pepperoni?");
            testTrack.track.addCue(cue);
            assert_equals(cues.length, 6);

            // Set startTime and endTime to negative values.
            var testCue = cues[2];
            assert_equals(testCue.startTime, 0);
            testCue.startTime = -5;
            assert_equals(testCue.startTime, -5);
            assert_equals(testCue.endTime, 30.5);
            testCue.endTime = -3439332606;
            assert_equals(testCue.endTime, -3439332606);

            // Check negative cues ordering.
            testCue = cues[3];
            assert_equals(testCue.startTime, 31);
            testCue.startTime = -200;
            // Verify that this cue is moved to 2nd position.
            assert_equals(cues[1].startTime, -200);
        });
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-negative-timestamp.html"
}
```
