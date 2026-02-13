# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-identifiers.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-identifiers.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Any text other than "-->" is recognized as optional cue identifier</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/cue-id.vtt", function(track) {
    var expected = [
        { id: "random_id", startTime: 0, endTime: 30.5, text: "Bear is Coming!!!!!" },
        { id: "another random identifier", startTime: 31, endTime: 60.5, text: "I said Bear is coming!!!!" },
        { id: "identifier--too", startTime: 61, endTime: 120.5, text: "I said Bear is coming now!!!!" },
        { id: "identifier--too", startTime: 121, endTime: 180.5, text: "Duplicate identifier" }
    ];

    assert_cues_match(track.cues, expected);
});

check_cues_from_track("resources/cue-id-error.vtt", function(track) {
    var expected = [
        { id: "", startTime: 0, endTime: 30.5, text: "Bear is Coming!!!!!" },
        { id: "", startTime: 31, endTime: 60.5, text: "I said Bear is coming!!!!" },
        { id: "", startTime: 61, endTime: 1200.5, text: "I said Bear is coming now!!!!" }
    ];

    assert_cues_match(track.cues, expected);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-identifiers.html"
}
```
