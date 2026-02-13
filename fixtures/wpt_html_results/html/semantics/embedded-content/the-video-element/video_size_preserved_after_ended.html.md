# html/semantics/embedded-content/the-video-element/video_size_preserved_after_ended.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/video_size_preserved_after_ended.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
    <title>HTML5 Media Elements: The size of the video shouldn't be lost after an 'ended' event.</title>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type">
    <link rel="author" title="Alicia Boya García" href="mailto:aboya@igalia.com"/>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
<video id="video">
    <source src="/media/test-1s.mp4" type="video/mp4">
    <source src="/media/test-1s.webm" type="video/webm">
</video>
<script>
    promise_test(async (test) => {
        const eventWatcher = new EventWatcher(test, video, ["loadedmetadata", "ended"]);
        await eventWatcher.wait_for("loadedmetadata");
        assert_equals(video.videoWidth, 320, "width when the video is loaded");
        assert_equals(video.videoHeight, 240, "height when the video is loaded");
        video.play();
        await eventWatcher.wait_for(["ended"]);
        assert_equals(video.videoWidth, 320, "width after playback");
        assert_equals(video.videoHeight, 240, "height after playback");
        if (video.videoTracks)
            assert_equals(video.videoTracks.length, 1);
    }, "Video dimensions are preserved at the end of the video.");
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
  "source_name": "html/semantics/embedded-content/the-video-element/video_size_preserved_after_ended.html"
}
```
