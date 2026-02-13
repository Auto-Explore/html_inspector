# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timestamp.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timestamp.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cues with &lt;timestamps&gt; tags</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/timestamp.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    // TODO(srirama.m): Timestamps are handled as ProcessingInstructions,
    // but because ProcessingInstructions are used in XML and not HTML,
    // they are ignored here. This should later be tested with oncuechange events.

    var children = [ { type: "text", value: "This cue is painted on." } ];
    assert_cue_fragment_as_textcontent(track.cues[0], children);

    children = [ { type: "text", value: "I said Bear is coming!!!!" } ];
    assert_cue_fragment_as_textcontent(track.cues[1], children);

    children = [ { type: "text", value: "I said Bear is coming now!!!!" } ];
    assert_cue_fragment_as_textcontent(track.cues[2], children);
});

check_cues_from_track("resources/timestamp-bad.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    var children = [ { type: "text", value: "This cue is painted on.\nBut since the last two timestamps are out of order, they are ignored." } ];
    assert_cue_fragment_as_textcontent(track.cues[0], children);

    children = [ { type: "text", value: "I said Bear is coming!!!!\nAll of these timestamps are before the start of the cue, so get ignored." } ];
    assert_cue_fragment_as_textcontent(track.cues[1], children);

    children = [ { type: "text", value: "I said Bear is coming now!!!!\nAll of these timestamps are after the end of the cue, so get ignored." } ];
    assert_cue_fragment_as_textcontent(track.cues[2], children);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-timestamp.html"
}
```
