# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-voice.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-voice.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cues with voice markup &lt;v&gt;</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/voice.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    var children = [
        { type: "span", style: { className: "blue", title: "Speaker" },
            value: [ { type: "text", value: "Bear is Coming!!!!!" } ] },
        { type: "text", value: "\nText span with a class and an annotation." }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "span", style: { title: "Doe Hunter" },
            value: [ { type: "text", value: "I said Bear is coming!!!!" } ] }
    ];
    assert_cue_fragment(track.cues[1], children);

    children = [
        { type: "text", value: "I said " },
        { type: "span", style: { className: "blue", title: "Speaker" },
            value: [ { type: "text", value: "Bear is coming now" } ] },
        { type: "text", value: "!!!!" }
    ];
    assert_cue_fragment(track.cues[2], children);
});

check_cues_from_track("resources/voice-bad.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    var children = [
        { type: "text", value: "Bear is Coming!!!!!" },
        { type: "text", value: "\nThis is two annotations for an empty tag." }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "text", value: "I said Bear is coming!!!!" },
        { type: "text", value: "\nThis does not parse as a voice tag." }
    ];
    assert_cue_fragment(track.cues[1], children);

    children = [
        { type: "text", value: "I said " },
        { type: "text", value: "Bear is coming now" },
        { type: "text", value: "!!!!\nThis does not parse as a voice tag." }
    ];
    assert_cue_fragment(track.cues[2], children);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-voice.html"
}
```
