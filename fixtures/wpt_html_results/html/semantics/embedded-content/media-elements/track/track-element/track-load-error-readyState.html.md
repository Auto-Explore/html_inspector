# html/semantics/embedded-content/media-elements/track/track-element/track-load-error-readyState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-load-error-readyState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Error event on HTMLTrackElement and ERROR readyState on TextTrack</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
  <track src="junk" default>
  <script>
  async_test(function(t) {
    var track = document.querySelector("track");
    track.onerror = t.step_func_done(function() {
      assert_equals(track.readyState, HTMLTrackElement.ERROR);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-load-error-readyState.html"
}
```
