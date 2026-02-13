# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-blank-lines.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-blank-lines.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cues are affected neither by multiple newlines \n, \r, and \r\n nor by the absence of a seperating line</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/cues.vtt", function(track) {
    var expected = [
        { id: "1", startTime: 0, endTime: 30.5, text: "Bear is Coming!!!!!" },
        { id: "2", startTime: 31, endTime: 60.5, text: "I said Bear is coming!!!!" },
        { id: "3", startTime: 61, endTime: 361200.5, text: "I said Bear is coming now!!!!" }
    ];

    assert_cues_match(track.cues, expected);
});

check_cues_from_track("resources/cues-no-separation.vtt", function(track) {
    var expected = [
        { id: "1", startTime: 0, endTime: 30.5, text: "Bear is Coming!!!!!\n2" },
        { id: "", startTime: 31, endTime: 60.5, text: "I said Bear is coming!!!!" },
        { id: "", startTime: 61, endTime: 361200.5, text: "I said Bear is coming now!!!!" }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-blank-lines.html"
}
```
