# html/semantics/embedded-content/media-elements/track/track-element/track-cues-seeking.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-seeking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTrack's activeCues are indexed and updated during video playback</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/cues-overlapping.vtt" kind="subtitles" default>
    <script>
    async_test(function(t) {
        var video = document.querySelector("video");
        var track = document.querySelector("track");
        track.onload = t.step_func(function() {
            assert_equals(track.track.cues.length, 3);
            video.src = getVideoURI("/media/test");
            video.currentTime = 0.5;
        });

        var seekedCount = 0;
        video.onseeked = t.step_func(function() {
            ++seekedCount;

            assert_equals(video.currentTime, seekedCount * 0.5);
            assert_equals(track.track.activeCues.length, seekedCount - 1);
            video.currentTime = (seekedCount + 1) * 0.5;

            if (seekedCount == 4)
                t.done();
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-seeking.html"
}
```
