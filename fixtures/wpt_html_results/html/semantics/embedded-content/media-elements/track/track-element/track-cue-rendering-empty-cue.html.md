# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-empty-cue.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-empty-cue.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Empty cues</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
    var video = document.createElement("video");
    video.src = getVideoURI("/media/test");
    video.addTextTrack("captions", "regular captions track", "en");
    video.textTracks[0].addCue(new VTTCue(0, 4, ""));

    video.onplaying = t.step_func_done();
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-empty-cue.html"
}
```
