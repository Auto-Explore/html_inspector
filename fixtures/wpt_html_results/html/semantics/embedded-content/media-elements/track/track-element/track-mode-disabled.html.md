# html/semantics/embedded-content/media-elements/track/track-element/track-mode-disabled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cues are properly removed from the active cue list when their track changes mode to disabled</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/captions-gaps.vtt" kind="captions" default >
    <script>
    async_test(function(t) {
        var video = document.querySelector("video");
        var testTrack = document.querySelector("track");

        video.src = getVideoURI("/media/counting");
        video.oncanplaythrough = t.step_func(startTest);
        video.onseeked = t.step_func_done(seeked);

        function startTest() {
            // Set the mode of the text track to "showing".
            testTrack.track.mode = "showing";
            // Seek to a time with a caption.
            video.currentTime = 1.5;
        }

        function seeked() {
            // Set the mode of the text track to "hidden", then to "showing" again.
            testTrack.track.mode = "hidden";
            testTrack.track.mode = "showing";

            // Set the mode of the text track to "disabled".
            testTrack.track.mode = "disabled";
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-disabled.html"
}
```
