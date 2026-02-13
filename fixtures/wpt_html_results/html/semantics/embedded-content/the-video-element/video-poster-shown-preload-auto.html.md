# html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
  <title>Verifies that video poster is shown even if video element has 'preload="auto"' attribute</title>
  <link rel="match" href="video-poster-shown-preload-auto-ref.html">
  <video preload="auto" poster="/media/poster.png" src="/media/video.webm" width="100" height="100"></video>
  <script>
    const video = document.querySelector("video");
    video.oncanplaythrough = () => document.documentElement.classList.remove("reftest-wait");
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
  "source_name": "html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto.html"
}
```
