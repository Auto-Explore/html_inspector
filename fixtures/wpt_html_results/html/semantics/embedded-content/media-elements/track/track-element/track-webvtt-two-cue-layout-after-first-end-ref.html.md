# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>WebVTT two-cue layout after the first cue has ended (reference)</title>
<script src="/common/reftest-wait.js"></script>
<video style="border:1px solid gray">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
</video>
<script>
// Add a single cue at line -2, where it would be if there was a first
// cue at line -1.
var video = document.querySelector("video");
var track = video.addTextTrack("captions");
var cue = new VTTCue(0, 3, "cue 2");
cue.line = -2;
track.addCue(cue);
track.mode = "showing";
video.play();
video.onplaying = function() {
  video.onplaying=null;
  video.pause();
  takeScreenshot();
};
</script>
</html>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end-ref.html"
}
```
