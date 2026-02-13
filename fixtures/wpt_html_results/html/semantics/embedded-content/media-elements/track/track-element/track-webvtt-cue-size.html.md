# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cue size from settings</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/cue-size.vtt", function(track) {
    var expected = [
        { size: 100 },
        { size: 10  },
        { size: 0   }
    ];

    assert_cues_match(track.cues, expected);
});

check_cues_from_track("resources/cue-size-bad.vtt", function(track) {
    var expected = [
        { size: 100 },
        { size: 100 },
        { size: 100 }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-cue-size.html"
}
```
