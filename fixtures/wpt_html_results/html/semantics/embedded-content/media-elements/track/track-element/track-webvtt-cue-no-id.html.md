# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-no-id.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-no-id.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Empty cue identifiers, but having "-->" leads to discarded cue</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/cue-no-id.vtt", testTrack);
check_cues_from_track("resources/cue-no-id-error.vtt", testTrack);

function testTrack(track) {
    var expected = [
        { id: "", startTime: 0, endTime: 30.5, text: "Bear is Coming!!!!!" },
        { id: "", startTime: 31, endTime: 60.5, text: "I said Bear is coming!!!!" },
        { id: "", startTime: 61, endTime: 1200.5, text: "I said Bear is coming now!!!!" }
    ];

    assert_cues_match(track.cues, expected);
}
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-no-id.html"
}
```
