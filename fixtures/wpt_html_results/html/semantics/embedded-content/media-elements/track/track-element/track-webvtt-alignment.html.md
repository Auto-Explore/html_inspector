# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-alignment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-alignment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cue alignment from settings</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/alignment.vtt", testTrack);
check_cues_from_track("resources/alignment-ltr.vtt", testTrack);

check_cues_from_track("resources/alignment-bad.vtt", function(track) {
    var expected = [
        { align: "center" },
        { align: "center" },
        { align: "center" },
        { align: "center" }
    ];

    assert_cues_match(track.cues, expected);
});

function testTrack(track) {
    var expected = [
        { align: "start" },
        { align: "center" },
        { align: "end" },
        { align: "center" }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-alignment.html"
}
```
