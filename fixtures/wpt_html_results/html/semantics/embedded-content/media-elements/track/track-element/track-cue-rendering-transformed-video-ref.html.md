# html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<script src="/common/reftest-wait.js"></script>
<style>
.container {
  transform: translate(1px, 0px);
  position: relative;
  display: inline-block;
  width: 320px;
  height: 240px;
}
.cue {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  overflow: hidden;
  text-align: start;
}
.cue > span {
  font-family: sans-serif;
  background: green;
  color: green;
  font-size: 50px;
  padding: 2px;
}
</style>
<div class="container">
  <video autoplay onplaying="this.onplaying = null; this.pause(); takeScreenshot();">
    <source src="/media/white.webm" type="video/webm">
    <source src="/media/white.mp4" type="video/mp4">
  </video>
  <div class="cue"><span>XXX</span></div>
</div>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-rendering-transformed-video-ref.html"
}
```
