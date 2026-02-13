# html/semantics/embedded-content/media-elements/track/track-element/track-id.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-id.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTrack "id" attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track id="LoremIpsum" src="resources/captions-fast.vtt" default>
    <script>
    test(function() {
        var video = document.querySelector("video");
        var track = document.querySelector("track");
        var textTrack = track.track;

        // Test default attribute value.
        assert_equals(textTrack.id, "LoremIpsum");
        assert_equals(video.textTracks[0].id, "LoremIpsum");

        // Make sure we can look up tracks by id.
        assert_equals(video.textTracks.getTrackById("LoremIpsum"), textTrack);

        // Test that it's readonly.
        textTrack.id = "newvalue";
        assert_equals(textTrack.id, "LoremIpsum");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-id.html"
}
```
