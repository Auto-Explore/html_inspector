# html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <title>Verifies that video poster is shown even if video element has 'preload="auto"' attribute</title>
  <video preload="none" poster="/media/poster.png" src="/media/video.webm" width="100" height="100"></video>
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
  "source_name": "html/semantics/embedded-content/the-video-element/video-poster-shown-preload-auto-ref.html"
}
```
