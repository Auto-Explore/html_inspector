# html/semantics/embedded-content/the-video-element/video_initially_paused.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_initially_paused.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset=UTF-8>
<title>Video elements should initially be paused</title>
<link rel="match" href="video_initially_paused-ref.html">
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-media-paused">
<script src="/common/media.js"></script>
<style>
div#video {
  padding: 6px 3px;
}
</style>
<p>The following video element should be paused. (All clocks at zero).</p>
<div id=video>
<script>
document.write(
  "<video src='"+ getVideoURI('/media/movie_300') + "' >" +
  "Your browser does not support the video element." +
  "<\/video>");
</script>
</div>
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_initially_paused.html"
}
```
