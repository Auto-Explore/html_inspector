# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-recovery.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-recovery.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A cue is recovered when a line with a "-->" is encountered without blank line separator</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/cue-recovery-header.vtt", testTrack);
check_cues_from_track("resources/cue-recovery-note.vtt", testTrack);
check_cues_from_track("resources/cue-recovery-cuetext.vtt", testTrack);

function testTrack(track) {
    var expected = [
        { startTime: 0, endTime: 1, text: "Valid cue 1" },
        { startTime: 2, endTime: 3, text: "Valid cue 2" }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-recovery.html"
}
```
