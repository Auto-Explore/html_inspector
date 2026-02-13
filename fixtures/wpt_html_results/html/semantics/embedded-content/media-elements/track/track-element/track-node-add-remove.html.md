# html/semantics/embedded-content/media-elements/track/track-element/track-node-add-remove.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-node-add-remove.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Add and remove track node</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
test(function() {
  var video = document.createElement('video');
  var tracka = document.createElement('track');
  video.appendChild(tracka);
  var trackb = document.createElement('track');
  video.appendChild(trackb);

  // Adding tracks outside the DOM tree.
  assert_array_equals(video.textTracks, [tracka.track, trackb.track]);

  // Inserting the parent video element into the document.
  document.body.appendChild(video);
  assert_array_equals(video.textTracks, [tracka.track, trackb.track]);

  // Inserting and removing another track in the document.
  var trackc = document.createElement('track');
  video.appendChild(trackc);
  assert_array_equals(video.textTracks, [tracka.track, trackb.track, trackc.track]);

  trackb.parentNode.removeChild(trackb);
  assert_array_equals(video.textTracks, [tracka.track, trackc.track]);

  // Removing the video from the document.
  document.body.removeChild(video);
  assert_array_equals(video.textTracks, [tracka.track, trackc.track]);

  tracka.parentNode.removeChild(tracka);
  assert_array_equals(video.textTracks, [trackc.track]);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-node-add-remove.html"
}
```
