# html/semantics/embedded-content/media-elements/resources/no-autoplay-audio-history-back-does-not-play-b.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/resources/no-autoplay-audio-history-back-does-not-play-b.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>History back page B</title>

<body>
  <script>
    "use strict";

    document.body.textContent = "Page B";
    window.addEventListener("load", () => {
      if (window.parent) {
        window.parent.postMessage({ type: "loaded_b" }, "*");
      }
    });
  </script>
</body>
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
  "source_name": "html/semantics/embedded-content/media-elements/resources/no-autoplay-audio-history-back-does-not-play-b.html"
}
```
