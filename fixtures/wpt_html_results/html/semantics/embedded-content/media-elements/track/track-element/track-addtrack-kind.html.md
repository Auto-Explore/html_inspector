# html/semantics/embedded-content/media-elements/track/track-element/track-addtrack-kind.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-addtrack-kind.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>addTextTrack() only accepts known "kind" values</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
    var trackCount = 0;

    function addTrack(type) {
        video.addTextTrack(type);
        assert_equals(video.textTracks.length, ++trackCount);
    }

    var video = document.createElement("video");
    assert_equals(video.textTracks.length, 0);
    assert_throws_js(TypeError, function() { video.addTextTrack("kaptions"); });
    assert_equals(video.textTracks.length, 0);

    addTrack("subtitles");
    addTrack("captions");
    addTrack("descriptions");
    addTrack("chapters");
    addTrack("metadata");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-addtrack-kind.html"
}
```
