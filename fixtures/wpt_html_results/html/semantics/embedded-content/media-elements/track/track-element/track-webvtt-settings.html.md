# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-settings.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-settings.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>WebVTT settings</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/settings.vtt", function(track) {
    var expected = [
        { line: 100, position: "auto", align: "start", vertical: "" },
        { line: 15, position: 40, align: "center", vertical: "rl" },
        { line: "auto", position: 10, align: "center", vertical: "" },
        { line: 95, position: "auto", align: "end", vertical: "lr" }
    ];

    assert_cues_match(track.cues, expected);
});

check_cues_from_track("resources/settings-bad-separation.vtt", function(track) {
    var expected = [
        { line: 43, position: 10, align: "center", vertical: "" },
        { line: "auto", position: 50, align: "end", vertical: "" },
        { line: "auto", position: "auto", align: "center", vertical: "" },
        { line: "auto", position: 90, align: "center", vertical: "lr" }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-settings.html"
}
```
