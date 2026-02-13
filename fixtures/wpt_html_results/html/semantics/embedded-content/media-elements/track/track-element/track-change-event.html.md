# html/semantics/embedded-content/media-elements/track/track-element/track-change-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-change-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A 'change' event is fired when a TextTrack's mode changes</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function(t) {
    var video = document.createElement('video');
    var track = video.addTextTrack('subtitles', 'test', 'en');

    // addTextTrack() defaults to "hidden", so settings
    // mode to "showing" should trigger a "change" event.
    track.mode = 'showing';
    assert_equals(video.textTracks.length, 1);

    video.textTracks.onchange = t.step_func_done(function(event) {
        assert_equals(event.target, video.textTracks);
        assert_true(event instanceof Event, 'instanceof');
        assert_false(event.hasOwnProperty('track'), 'unexpected property found: "track"');
    });
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-change-event.html"
}
```
