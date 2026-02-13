# html/semantics/embedded-content/the-video-element/video_crash_empty_src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_crash_empty_src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <title>HTML5 Media Elements: An empty src should not crash the player.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
    <link rel="author" title="Alicia Boya García" href="mailto:aboya@igalia.com"/>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
    function makeCrashTest(src) {
        async_test((test) => {
            const video = document.createElement("video");
            video.src = src;
            video.controls = true;
            video.addEventListener("error", () => {
                document.body.removeChild(video);
                test.done();
            });
            document.body.appendChild(video);
        }, `src="${src}" does not crash.`);
    }

    makeCrashTest("about:blank");
    makeCrashTest("");
</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_crash_empty_src.html"
}
```
