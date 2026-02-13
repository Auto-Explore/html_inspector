# html/semantics/embedded-content/media-elements/track/track-element/track-cues-sorted-before-dispatch.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-sorted-before-dispatch.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>All events are triggered in chronological order</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/sorted-dispatch.vtt" default>
    <script>
    async_test(function(t) {
        var video = document.querySelector("video");
        video.src = getVideoURI("/media/test");
        var track = document.querySelector("track");

        track.onload = t.step_func(function() {
            var cues = track.track.cues;
            assert_equals(cues.length, 8);

            for (var i = 0; i < cues.length; ++i) {
                cues[i].onenter = t.step_func(cueEnteredOrExited);
                cues[i].onexit = t.step_func(cueEnteredOrExited);
            }

            video.play();
        });

        var cueTimings = [];
        function cueEnteredOrExited(event) {
            var currentCue = event.target;

            if (event.type == "exit")
                cueTimings.push(currentCue.endTime);
            else
                cueTimings.push(currentCue.startTime);
        }

        video.onended = t.step_func_done(function() {
            assert_equals(cueTimings.length, 14);
            var time = 0;
            for (var i = 0; i < cueTimings.length; ++i) {
                assert_less_than_equal(time, cueTimings[i], "cueTimings[" + i + "]");
                time = cueTimings[i];
            }
        });

        video.currentTime = 5;
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-sorted-before-dispatch.html"
}
```
