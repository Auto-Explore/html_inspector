# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-line-doesnt-fit.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-line-doesnt-fit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<script src="/common/reftest-wait.js"></script>
<link rel="match" href="track-cue-rendering-line-doesnt-fit-ref.html">
<script>
function addCue(track, cueData) {
  var cue = new VTTCue(0, 10, 'XXX');
  for (var prop in cueData)
    cue[prop] = cueData[prop];
  track.addCue(cue);
}
</script>
<style>
video::cue {
  font-size: 120px;
  color: green;
  background-color: green;
}
</style>
<video autoplay onplaying="this.onplaying = null; this.pause(); takeScreenshot();">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
  <script>
  var video = document.querySelector("video");
  var track = video.addTextTrack('subtitles');
  addCue(track, { line: 0, align: 'start', text: 'PAS' });
  // This cue will not fit, and will not be displayed.
  addCue(track, { line: 1, align: 'start', text: 'FAI' });
  track.mode = 'showing';
  </script>
</video>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-line-doesnt-fit.html"
}
```
