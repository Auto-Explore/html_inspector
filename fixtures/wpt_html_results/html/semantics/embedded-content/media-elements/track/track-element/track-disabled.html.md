# html/semantics/embedded-content/media-elements/track/track-element/track-disabled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Disabling a track</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track kind="subtitles" src="resources/captions.vtt"/>
</video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");
    video.textTracks[0].mode = "disabled";

    // Waiting for the duration of the first cue to elapse.
    video.ontimeupdate = t.step_func(function (event) {
        if (event.target.currentTime < 1)
            return;

        // End test after the duration of the first cue to make sure
        // the test would have gone through the period where this cue
        // would have been rendered if the track was not disabled.
        t.done();
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-disabled.html"
}
```
