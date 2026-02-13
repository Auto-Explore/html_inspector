# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-removed-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-removed-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<script src="/common/reftest-wait.js"></script>
<title>Text track cue layout after controls are removed (reference)</title>
<style>
::cue {
  font-size: 50px;
}

video {
  border: 1px solid gray;
}
</style>
<video onloadeddata="this.onloadeddata = null; takeScreenshot();">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
</video>
<script>
// Add a single cue at line -2, where it would be if there were controls visible
// at the bottom. (This assumes that those controls are less than 50px high.)
// cue at line -1.
var video = document.querySelector("video");
var track = video.addTextTrack("captions");
var cue = new VTTCue(0, 1, "text");
cue.line = -2;
track.addCue(cue);
track.mode = "showing";
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-removed-ref.html"
}
```
