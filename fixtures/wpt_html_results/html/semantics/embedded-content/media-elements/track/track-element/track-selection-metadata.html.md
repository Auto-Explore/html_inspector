# html/semantics/embedded-content/media-elements/track/track-element/track-selection-metadata.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-selection-metadata.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Multiple 'metadata' tracks with 'default'</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track kind="metadata" src="resources/default-styles.vtt" id="t1">
    <track kind="metadata" src="resources/class.vtt" default id="t2hidden">
    <track kind="metadata" src="resources/metadata-area.vtt" id="t3">
    <track kind="metadata" src="resources/webvtt-file.vtt" default id="t4hidden">
</video>
<script>
async_test(function() {
    var video = document.querySelector('video');
    video.onloadstart = this.step_func_done(function() {
        assert_equals(video.textTracks.length, 4);
        for (var track of video.textTracks) {
            assert_equals(track.kind, 'metadata');

            var trackElement = document.getElementById(track.id);
            if (track.id.indexOf('hidden') != -1) {
                assert_true(trackElement.default);
                assert_equals(track.mode, 'hidden');
            } else {
                assert_false(trackElement.default);
                assert_equals(track.mode, 'disabled');
            }
        }
    });

    video.src = getVideoURI("/media/test");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.track.default.multiple",
      "message": "The “default” attribute must not occur on more than one “track” element within the same “audio” or “video” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 526,
        "byte_start": 449,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-selection-metadata.html"
}
```
