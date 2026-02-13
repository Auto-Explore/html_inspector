# html/semantics/embedded-content/media-elements/track/track-element/track-default-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-default-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A track with the "default" attribute loads automatically</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
  <track kind="captions" src="resources/default-styles.vtt">
  <track kind="captions" src="resources/metadata-area.vtt">
  <track kind="captions" src="resources/webvtt-file.vtt" id="default" default>
  <script>
  async_test(function(t) {
    var timer = null;
    var tracks = document.querySelectorAll("track");
    for (var track of tracks) {
      track.onload = t.step_func(function() {
        assert_equals(event.target.readyState, HTMLTrackElement.LOADED);
        assert_equals(event.target.id, "default");
        assert_true(event.target.default);
        // End the test after a brief pause so we allow other tracks to load if they will.
        if (timer)
          clearTimeout(timer);
        timer = t.step_timeout(t.step_func_done(), 200);
      });
    }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-default-attribute.html"
}
```
