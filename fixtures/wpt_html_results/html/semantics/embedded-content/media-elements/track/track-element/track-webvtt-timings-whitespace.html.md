# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timings-whitespace.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timings-whitespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>"Skip whitespace" step around cue-timings separator</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/timings-whitespace.vtt", function(track) {
    var expected = [
        { id: "1", startTime: 0.1, endTime: 1.5, text: "Single U+0020 SPACE left of cue-timings separator" },
        { id: "2", startTime: 0.1, endTime: 1.5, text: "Single U+0020 SPACE right of cue-timings separator" },
        { id: "3", startTime: 0.1, endTime: 1.5, text: "Single U+0009 TAB left of cue-timings separator" },
        { id: "4", startTime: 0.1, endTime: 1.5, text: "Single U+0009 TAB right of cue-timings separator" },
        { id: "5", startTime: 0.1, endTime: 1.5, text: "Single U+000C FORM FEED left of cue-timings separator" },
        { id: "6", startTime: 0.1, endTime: 1.5, text: "Single U+000C FORM FEED right of cue-timings separator" },
        { id: "7", startTime: 0.1, endTime: 1.5, text: "Several U+0020 SPACE left of cue-timings separator" },
        { id: "8", startTime: 0.1, endTime: 1.5, text: "Several U+0020 SPACE right of cue-timings separator" },
        { id: "9", startTime: 0.1, endTime: 1.5, text: "Several U+0009 TAB left of cue-timings separator" },
        { id: "10", startTime: 0.1, endTime: 1.5, text: "Several U+0009 TAB right of cue-timings separator" },
        { id: "11", startTime: 0.1, endTime: 1.5, text: "Several U+000C FORM FEED left of cue-timings separator" },
        { id: "12", startTime: 0.1, endTime: 1.5, text: "Several U+000C FORM FEED right of cue-timings separator" }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timings-whitespace.html"
}
```
