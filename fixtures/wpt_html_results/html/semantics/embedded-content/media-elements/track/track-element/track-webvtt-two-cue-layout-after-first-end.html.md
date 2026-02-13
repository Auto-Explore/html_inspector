# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>WebVTT two-cue layout after the first cue has ended</title>
<link rel="match" href="track-webvtt-two-cue-layout-after-first-end-ref.html">
<script src="/common/reftest-wait.js"></script>
<video style="border:1px solid gray">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
</video>
<script>
// Add two cues, where the first cue ends before the second.
var video = document.querySelector("video");
var track = video.addTextTrack("captions");
let cue1 = new VTTCue(-1, 1, "cue 1");
track.addCue(cue1);
// As video's duration is 10s, it ensures that this cue would always be displayed.
track.addCue(new VTTCue(0, 10, "cue 2"));
track.mode = "showing";
video.play();
cue1.onexit = () => {
  cue1.onexit = null;
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-two-cue-layout-after-first-end.html"
}
```
