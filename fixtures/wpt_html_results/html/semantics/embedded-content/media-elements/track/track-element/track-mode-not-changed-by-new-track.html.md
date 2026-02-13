# html/semantics/embedded-content/media-elements/track/track-element/track-mode-not-changed-by-new-track.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-not-changed-by-new-track.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A track appended after the initial track configuration does not change other tracks</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track kind="metadata" src="resources/metadata.vtt">
</video>
<script>
async_test(function(t) {
    var video = document.querySelector('video');

    var track1 = document.querySelectorAll('track')[0];
    assert_equals(track1.readyState, HTMLTrackElement.NONE);
    assert_equals(track1.track.mode, 'disabled');

    video.src = getVideoURI('/media/test');
    video.oncanplaythrough = t.step_func(canplaythrough);
    track1.onload = t.step_func(metadataTrackLoaded);

    function canplaythrough() {
        // check initial metadata track state.
        assert_equals(track1.readyState, HTMLTrackElement.NONE);
        assert_equals(track1.track.mode, 'disabled');
        assert_equals(track1.track.cues, null);
        track1.track.mode = 'hidden';
    }

    function metadataTrackLoaded() {
        // check metadata track state.
        assert_equals(track1.readyState, HTMLTrackElement.LOADED);
        assert_equals(track1.track.mode, 'hidden');
        assert_equals(track1.track.cues.length, 12);
        assert_equals(track1.track.cues[11].startTime, 22);

        // Add a caption track, and explicitly enable it.
        track2 = document.createElement('track');
        track2.setAttribute('kind', 'captions');
        track2.setAttribute('default', 'default');
        track2.setAttribute('src', 'resources/webvtt-file.vtt');
        track2.track.mode = 'showing';
        track2.onload = t.step_func(captionsTrackLoaded);
        video.appendChild(track2);
    }

    function captionsTrackLoaded() {
        // Check that metadata track state has not changed.
        assert_equals(track1.readyState, HTMLTrackElement.LOADED);
        assert_equals(track1.track.mode, 'hidden');
        // and that the caption track state is correct.
        assert_equals(track2.readyState, HTMLTrackElement.LOADED);
        assert_equals(track2.track.mode, 'showing');

        video.textTracks.onaddtrack = t.step_func_done(trackAdded);
        // add a subtitle track with video.addTextTrack().
        track3 = video.addTextTrack('subtitles', 'Subtitle Track', 'en');
        track3.mode = 'showing';
    }

    function trackAdded(event) {
        // Check that metadata track state has not changed.
        assert_equals(track1.readyState, HTMLTrackElement.LOADED);
        assert_equals(track1.track.mode, 'hidden');
        // and that the caption track state has not changed.
        assert_equals(track2.readyState, HTMLTrackElement.LOADED);
        assert_equals(track2.track.mode, 'showing');
        // and that the subtitle track state is correct.
        assert_equals(event.target, video.textTracks);
        assert_true(event instanceof window.TrackEvent);
        assert_equals(event.track, video.textTracks[video.textTracks.length - 1]);
        assert_equals(track3.mode, 'showing');
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-mode-not-changed-by-new-track.html"
}
```
