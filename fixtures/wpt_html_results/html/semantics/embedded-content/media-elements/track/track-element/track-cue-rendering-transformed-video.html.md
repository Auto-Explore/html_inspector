# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<script src="/common/reftest-wait.js"></script>
<link rel="match" href="track-cue-rendering-transformed-video-ref.html">
<style>
video {
  transform: translate(1px, 0px);
}
video::cue {
  font-size: 50px;
  color: green;
  background-color: green;
}
</style>
<video autoplay onplaying="this.onplaying = null; this.pause(); takeScreenshot();">
  <source src="/media/white.webm" type="video/webm">
  <source src="/media/white.mp4" type="video/mp4">
  <script>
  var video = document.querySelector('video');
  var track = video.addTextTrack('subtitles');
  var cue = new VTTCue(0, 10, 'XXX');
  cue.align = 'start';
  cue.line = 0;
  track.addCue(cue);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video.html"
}
```
