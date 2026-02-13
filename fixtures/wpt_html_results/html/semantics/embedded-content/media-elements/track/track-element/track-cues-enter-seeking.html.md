# html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-seeking.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-seeking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTrack's cue onenter handler called when seeked onto</title>
<meta name="timeout" content="long">
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/cues-chrono-order.vtt" kind="captions" default>
    <script>
    // Check that the onenter event is called for the right cue when seeking on the video element.
    // Based on the spec step 4 [1], after a seek happens, the missed cues should be empty,
    // so any cues before the target time should not receive enter event.
    // [1] https://html.spec.whatwg.org/multipage/media.html#time-marches-on
    async_test(function(t) {
        var video = document.querySelector("video");
        var testTrack = document.querySelector("track");

        video.src = getVideoURI("/media/test");

        video.oncanplaythrough = t.step_func(attemptTests);

        function attemptTests() {
            assert_equals(testTrack.track.cues.length, 3);
            const targetTime = 4.0000000004;

            for (let cue of testTrack.track.cues) {
                if (cue.endTime > targetTime) {
                    cue.onenter = t.step_func(_=>t.done());
                } else {
                    cue.onenter = t.unreached_func("onenter called for wrong cue");
                }
            }

            video.currentTime = targetTime;
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-enter-seeking.html"
}
```
