# html/semantics/embedded-content/the-video-element/video_timeupdate_on_seek.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_timeupdate_on_seek.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <title>HTML5 Media Elements: timeupdate is emitted after a seek before the data is received.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
    <link rel="author" title="Alicia Boya García" href="mailto:aboya@igalia.com"/>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body onload="runTests()">
<script>
    const seekTime = 60 * 4;

    function testTimeupdateOnSeek(mediaType) {
        async_test(function (test) {
            const video = document.createElement("video");
            video.src = `timeout_on_seek.py?extension=${mediaType}`;
            video.controls = true;
            video.defaultMuted = true;
            document.body.appendChild(video);

            video.addEventListener("canplay", test.step_func(videoCanPlay), {once: true});

            function videoCanPlay() {
                video.addEventListener("timeupdate", test.step_func(onTimeUpdate));
                video.play().catch(() => test.unreached_func("play() rejected"));
                video.currentTime = seekTime;
            }

            function onTimeUpdate() {
                if (Math.abs(video.currentTime - seekTime) <= 1) {
                    document.body.removeChild(video);
                    test.done();
                }
            }
        }, `timeupdate is emitted after a seek before the data is received: ${mediaType}.`);
    }

    function runTests() {
        const testerVideo = document.createElement("video");
        if (testerVideo.canPlayType("video/mp4"))
            testTimeupdateOnSeek("mp4");
        if (testerVideo.canPlayType("video/webm"))
            testTimeupdateOnSeek("webm");
    }
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_timeupdate_on_seek.html"
}
```
