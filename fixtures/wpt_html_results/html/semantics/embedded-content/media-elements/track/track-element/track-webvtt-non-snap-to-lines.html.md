# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Position is not adjusted for non snap-to-lines cues</title>
<link rel="match" href="track-webvtt-non-snap-to-lines-ref.html">
<script src="/common/reftest-wait.js"></script>
<script src="/common/media.js"></script>
<style>
::cue {
  background: green;
}
</style>
<video autoplay onplaying="this.onplaying = null; this.pause(); takeScreenshot();"></video>
<script>
var video = document.querySelector("video");
var track = video.addTextTrack("captions");
var cue = new VTTCue(0, 1, "Bear is Coming!!!!!");
cue.snapToLines = false;
cue.line = 20;
cue.align = "left";
track.addCue(cue);
track.mode = "showing";
video.src = getVideoURI("/media/test");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines.html"
}
```
