# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-unsupported-markup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-unsupported-markup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Unsupported markup is properly ignored</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var getCueAsHTMLContent = function(cue) {
    return cue.getCueAsHTML().textContent;
};

check_cues_from_track("resources/unsupported-markup.vtt", function(track) {
    var expected = [
        {
            innerHTML: getCueAsHTMLContent,
            expected: "Bear is Coming!!!!!\nAnd what kind of a bear it is - just have look."
        },
        {
            innerHTML: getCueAsHTMLContent,
            expected: "\n  I said Bear is coming!!!!\n  I said Bear is still coming!!!!\n",
        },
        {
            innerHTML: getCueAsHTMLContent,
            expected: "\n  I said Bear is coming now!!!!\n  \n  \n",
        }
    ];

    assert_cues_html_content(track.cues, expected);

    var expected_text = [
        { text: "<h1>Bear is Coming!!!!!</h1>\n<p>And what kind of a bear it is - just have <a href=\"webpage.html\">look</a>.</p>" },
        { text: "<ul>\n  <li>I said Bear is coming!!!!</li>\n  <li>I said Bear is still coming!!!!</li>\n</ul>" },
        { text: "<ol>\n  <li>I said Bear is coming now!!!!</li>\n  <li><img src=\"bear.png\" alt=\"mighty bear\"></li>\n  <li><video src=\"bear_ad.webm\" controls></video></li>\n</ol>" }
    ];

    assert_cues_match(track.cues, expected_text);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-unsupported-markup.html"
}
```
