# html/semantics/embedded-content/media-elements/track/track-element/track-element-dom-change.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-element-dom-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Simple DOM mutations with track element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
    var video = document.createElement("video");
    var testTrack = document.createElement("track");

    // Append the track element to the video element.
    video.appendChild(testTrack);

    // Set the mode of the text track to "showing".
    testTrack.track.mode = "showing";
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-element-dom-change.html"
}
```
