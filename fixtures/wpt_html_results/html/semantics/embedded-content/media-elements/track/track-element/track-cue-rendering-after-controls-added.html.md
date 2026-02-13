# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-added.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-added.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<script src="/common/reftest-wait.js"></script>
<link rel="match" href="track-cue-rendering-after-controls-added-ref.html">
<title>Text track cue layout after controls are added</title>
<style>
::cue {
  font-size: 50px;
}
</style>
<!-- Width should be large enough to display all of the media controls. -->
<video style="border:1px solid gray; width: 500px;">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
</video>
<script>
// Add a cue that will overlap with the video controls.
var video = document.querySelector("video");
var track = video.addTextTrack("captions");
track.addCue(new VTTCue(0, 1, "text"));
track.mode = "showing";

video.onloadeddata = function() {
  // Double nesting of requestAnimationFrame to
  // make sure cue layout and paint happens.
  window.requestAnimationFrame(function() {
    window.requestAnimationFrame(function() {
      video.controls = true;
      // Wait for the relayout before screenshot.
      window.requestAnimationFrame(function() {
        takeScreenshot();
      });
    });
  });
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-after-controls-added.html"
}
```
