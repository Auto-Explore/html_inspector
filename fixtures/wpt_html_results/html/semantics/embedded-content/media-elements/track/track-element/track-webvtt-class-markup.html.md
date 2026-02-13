# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-class-markup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-class-markup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests cues with class markup &lt;c&gt;.</title>
<meta name="timeout" content="long">
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/class.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    var children = [
        { type: "span", style: { className: "black" },
            value: [ { type: "text", value: "Bear is Coming!!!!!" } ] }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "span", style: { className: "green" },
            value: [ { type: "text", value: "I said Bear is coming!!!!" } ] }
    ];
    assert_cue_fragment(track.cues[1], children);

    children = [
        { type: "text", value: "I said " },
        { type: "span", style: { className: "red uppercase" },
            value: [ { type: "text", value: "Bear is coming now" } ] },
        { type: "text", value: "!!!!" }
    ];
    assert_cue_fragment(track.cues[2], children);
});

check_cues_from_track("resources/class-bad.vtt", function(track) {
    assert_equals(track.cues.length, 3);

    var children = [
        { type: "span", value: [ { type: "text", value: "Bear is Coming!!!!!" } ] },
        { type: "text", value: "\nThe space signified an annotation start." }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "span", style: { className: "red&large" },
            value: [ { type: "text", value: "I said Bear is coming!!!!" } ] },
        { type: "text", value: "\nProbably should only allow characters that CSS allows in class names." }
    ];
    assert_cue_fragment(track.cues[1], children);

    children = [
        { type: "text", value: "I said " },
        { type: "span", style: { className: "9red upper+case" },
            value: [ { type: "text", value: "Bear is coming now" } ] },
        { type: "text", value: "!!!!\nProbably should only allow characters that CSS allows in class names." }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-class-markup.html"
}
```
