# html/semantics/embedded-content/media-elements/track/track-element/track-remove-insert-ready-state.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-insert-ready-state.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Attaching a media element again to the document, having a child track that failed loading doesn't block video from playing</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/no-webvtt.vtt" kind="captions" default>
</video>
<script>
  async_test(function(t) {
    var video = document.querySelector('video');
    video.src = getVideoURI('/media/test');
    video.oncanplaythrough = t.step_func(canplaythrough);

    function canplaythrough() {
      video.oncanplaythrough = null;
      var track = document.querySelector('track');

      // Track should have error as ready state.
      assert_equals(track.readyState, HTMLTrackElement.ERROR);

      // Remove the video element from body.
      document.body.removeChild(video);

      // Reset the video src attribute to re-trigger resource selection for tracks.
      video.src = getVideoURI('/media/test');

      // Append the video element back to the body.
      document.body.appendChild(video);

      assert_equals(track.readyState, HTMLTrackElement.ERROR);

      video.onplaying = t.step_func_done();
      video.play();
      // The video should start playing.
    }
  });
</script>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-insert-ready-state.html"
}
```
