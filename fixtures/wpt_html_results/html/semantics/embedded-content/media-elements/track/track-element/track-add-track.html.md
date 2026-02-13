# html/semantics/embedded-content/media-elements/track/track-element/track-add-track.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-add-track.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>'addtrack' event is fired when a TextTrack is created</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
    var video = document.createElement('video');

    var trackElement = document.createElement('track');
    video.appendChild(trackElement);
    var tracks = [];
    tracks.push(trackElement.track);

    // Register the 'addtrack' listener after creating the element
    // to make sure the event is dispatched asynchronously.
    video.textTracks.onaddtrack = t.step_func(function(event) {
        assert_equals(event.target, video.textTracks);
        assert_true(event instanceof TrackEvent, 'instanceof');
        assert_equals(event.track, tracks[video.textTracks.length - 1]);

        if (video.textTracks.length == 1) {
            tracks.push(video.addTextTrack('captions', 'Caption Track', 'en'));
            assert_equals(video.textTracks.length, 2);
        } else {
            t.done();
        }
    });

    trackElement.src = 'resources/webvtt-file.vtt';
    trackElement.track.mode = 'hidden';
    assert_equals(video.textTracks.length, 1);
    assert_equals(trackElement.readyState, HTMLTrackElement.NONE);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-add-track.html"
}
```
