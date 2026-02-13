# html/semantics/embedded-content/media-elements/track/track-element/track-text-track-cue-list.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-text-track-cue-list.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTrackCueList functionality: length, operator[], and getCueById()</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/settings.vtt" kind="captions" default>
    <script>
    async_test(function(t) {
        var testTrack = document.querySelector("track");

        testTrack.onload = t.step_func_done(function() {
            var cues = testTrack.track.cues;

            // Testing TextTrackCueList length.
            assert_equals(cues.length, 4);

            // Testing TextTrackCueList [] operator.
            assert_equals(cues[0].id, "1");
            assert_equals(cues[3].id, "4");
            assert_equals(cues[4], undefined);

            // Testing TextTrackCueList getCueById().
            assert_equals(cues.getCueById("1").startTime, 0);
            assert_equals(cues.getCueById("4").startTime, 121);
            assert_equals(cues.getCueById("junk"), null);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-text-track-cue-list.html"
}
```
