# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Reference test for track-webvtt-non-snap-to-lines.html</title>
<script src="/common/reftest-wait.js"></script>
<script src="/common/media.js"></script>
<style>
.container {
  position: relative;
  display: inline-block;
}
.cue {
  position: absolute;
  top: 30px;
  left: 0px;
  font-family: sans-serif;
  background: green;
  color: rgba(255, 255, 255, 1);
  font-size: 7.5px;
}
</style>
<div class="container">
  <video autoplay onplaying="this.onplaying = null; this.pause(); takeScreenshot();">
    <script>
    document.currentScript.parentNode.src = getVideoURI("/media/test");
    </script>
  </video>
  <span class="cue">Bear is Coming!!!!!</span>
</div>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-non-snap-to-lines-ref.html"
}
```
