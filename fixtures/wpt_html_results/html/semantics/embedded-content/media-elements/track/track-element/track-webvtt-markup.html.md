# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-markup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-markup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cues with &lt;b&gt;, &lt;i&gt;, &lt;u&gt;, &lt;rt&gt; and &lt;ruby&gt; tags</title>
<meta name="timeout" content="long">
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
check_cues_from_track("resources/markup.vtt", function(track) {
    assert_equals(track.cues.length, 4);

    var children = [
        { type: "text", value: "The following bear is bold:\n" },
        { type: "b", value: [ { type: "text", value: "Bear" } ] },
        { type: "text", value: " is Coming!!!!!" }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "text", value: "The following bear is in italics and has a class of \"larger\":\n" },
        { type: "i", value: [ { type: "text", value: "Bear" } ] },
        { type: "text", value: " is Coming!!!!!" }
    ];

    var fragment = createFragment(children);
    fragment.querySelector("i").className = "larger";
    assert_true(fragment.isEqualNode(track.cues[1].getCueAsHTML()));

    children = [
        { type: "text", value: "The following bear is underlined even though the element has a blank:\nI said " },
        { type: "u", value: [ { type: "text", value: "Bear" } ] },
        { type: "text", value: " is coming!!!!" }
    ];
    assert_cue_fragment(track.cues[2], children);

    children = [
        { type: "text", value: "The following bear is ruby annotated:\nI said " },
        {
            type: "ruby",
            value: [
                { type: "text", value: "Bear" },
                {
                    type: "rt",
                    value: [ { type: "text", value: "bear with me" } ]
                }
            ]
        },
        { type: "text", value: " is coming!!!!" }
    ];
    assert_cue_fragment(track.cues[3], children);
});

check_cues_from_track("resources/markup-bad.vtt", function(track) {
    assert_equals(track.cues.length, 4);

    var children = [
        { type: "text", value: "The following bear starts bold but end is broken:\n" },
        {
            type: "b",
            value:
            [
                { type: "text", value: "Bear" },
                { type: "text", value: " is Coming!!!!!" }
            ]
        }
    ];
    assert_cue_fragment(track.cues[0], children);

    children = [
        { type: "text", value: "The following bear is not in italics but the markup is removed:\n" },
        { type: "text", value: "Bear" },
        { type: "text", value: " is Coming!!!!!" }
    ];
    assert_cue_fragment(track.cues[1], children);

    children = [
        { type: "text", value: "The following bear is not underlined and markup is removed:\nI said " },
        { type: "text", value : "Bear" },
        { type: "text", value : " is coming!!!!" }
    ];
    assert_cue_fragment(track.cues[2], children);

    children = [
        { type: "text", value: "The following bear is not ruby annotated and markup is removed:\nI said " },
        { type: "text", value: "Bear" },
        { type: "text", value: "bear with me" },
        { type: "text", value: " is coming!!!!" }
    ];
    assert_cue_fragment(track.cues[3], children);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-markup.html"
}
```
