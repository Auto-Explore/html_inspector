# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-newlines.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-newlines.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A cue with no newline at eof is parsed properly</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/no-newline-at-eof.vtt" default>
    <script>
        async_test(function(t) {
            var track = document.querySelector("track");

            track.onload = t.step_func_done(function() {
                var expected = [
                    {
                        id : "1",
                        startTime : 0,
                        endTime : 30.5,
                        text : "Bear is Coming!!!!!"
                    }
                ];

                assert_cues_equal(track.track.cues, expected);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-newlines.html"
}
```
