# html/semantics/embedded-content/media-elements/track/track-element/track-texttracks.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-texttracks.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>TextTracks in a TextTrackList are kept in the correct order</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track kind="captions" src="resources/webvtt-file.vtt">
</video>
<script>
test(function() {
    var video = document.querySelector("video");

    // Add a track with video.addTextTrack().
    video.addTextTrack("descriptions", "Descriptions Track", "en");

    // Add a track element with DOM API.
    var trackElement = document.createElement("track");
    trackElement.setAttribute("kind", "chapters");
    video.appendChild(trackElement);

    // Verify track order.
    assert_equals(video.textTracks.length, 3);
    assert_equals(video.textTracks[0].kind, "captions");
    assert_equals(video.textTracks[1].kind, "chapters");
    assert_equals(video.textTracks[2].kind, "descriptions");

    // Verify the default parameters of the text track object
    // returned by addTextTrack().
    assert_equals(video.textTracks[2].mode, "hidden");
    assert_not_equals(video.textTracks[2].cues, null);
    assert_equals(video.textTracks[2].cues.length, 0);

    // Add another track element, it should insert
    // before the addTextTrack() track.
    trackElement = document.createElement("track");
    trackElement.setAttribute("kind", "metadata");
    video.appendChild(trackElement);

    assert_equals(video.textTracks.length, 4);
    assert_equals(video.textTracks[0].kind, "captions");
    assert_equals(video.textTracks[1].kind, "chapters");
    assert_equals(video.textTracks[2].kind, "metadata");
    assert_equals(video.textTracks[3].kind, "descriptions");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-texttracks.html"
}
```
