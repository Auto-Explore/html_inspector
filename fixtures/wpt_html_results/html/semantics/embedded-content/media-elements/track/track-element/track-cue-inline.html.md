# html/semantics/embedded-content/media-elements/track/track-element/track-cue-inline.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-inline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Add a track and change its mode through JS</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <source src="/media/test.mp4" type="video/mp4">
    <source src="/media/test.webm" type="video/webm">
</video>
<script>
test(function() {
    var video = document.querySelector('video');
    var track = video.addTextTrack('captions', 'English', 'en');
    track.addCue(new VTTCue(0.0, 10.0, 'wow wow'));
    track.mode = 'showing';
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cue-inline.html"
}
```
