# html/semantics/embedded-content/media-elements/track/track-element/track-large-timestamp.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-large-timestamp.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Very large timestamp is parsed correctly</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/large-timestamp.vtt" default>
    <script>
    async_test(function(t) {
        var testTrack = document.querySelector("track");
        testTrack.onload = t.step_func_done(function() {
            assert_equals(testTrack.track.cues.length, 1);
            var cue = testTrack.track.cues[0];
            assert_equals(parseInt(cue.id), 1);
            assert_equals(cue.startTime / 3600, 1234567);
            assert_equals(cue.endTime / 3600, 1234567890);
        });
    });
    </script>
</video>
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-large-timestamp.html"
}
```
