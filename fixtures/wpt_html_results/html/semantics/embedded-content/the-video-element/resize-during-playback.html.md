# html/semantics/embedded-content/the-video-element/resize-during-playback.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-video-element/resize-during-playback.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<title>video element resizing during playback</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#concept-video-intrinsic-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<script>
for (const format of ['mp4', 'webm']) {
  promise_test(async (t) => {
    const video = document.createElement('video');
    assert_implements_optional(video.canPlayType(`video/${format}`), `${format} supported`);

    const eventWatcher = new EventWatcher(t, video, ['resize', 'playing', 'error', 'ended']);

    // Load the video and wait for initial resize event.
    video.muted = true;
    video.preload = 'auto';
    video.onerror = t.unreached_func("error during playback");
    video.src = `/media/400x300-red-resize-200x150-green.${format}`;
    document.body.appendChild(video);

    await eventWatcher.wait_for(['resize']);
    assert_equals(video.videoWidth, 400, 'width after first resize event');
    assert_equals(video.videoHeight, 300, 'height after first resize event');

    // Now play and wait for a second resize event.
    const playPromise = video.play();
    if (playPromise) {
      playPromise.catch(t.unreached_func("play rejected"));
    }
    await eventWatcher.wait_for(['playing', 'resize']);
    assert_equals(video.videoWidth, 200, 'width after second resize event');
    assert_equals(video.videoHeight, 150, 'height after second resize event');
  }, `${format} video`);
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
  "source_name": "html/semantics/embedded-content/the-video-element/resize-during-playback.html"
}
```
