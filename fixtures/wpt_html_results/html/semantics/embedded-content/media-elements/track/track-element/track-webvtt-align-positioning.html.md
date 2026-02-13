# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-align-positioning.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-align-positioning.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cue text position and alignment from settings</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/align-positioning.vtt">
    <track src="resources/align-positioning-bad.vtt">
</video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");

    var trackElements = document.querySelectorAll("track");
    assert_equals(trackElements.length, video.textTracks.length);
    for (var i = 0; i < trackElements.length; i++)
        trackElements[i].onload = t.step_func(trackLoaded);

    enableAllTextTracks(video.textTracks);

    var numberOfTracksLoaded = 0;
    function trackLoaded() {
        numberOfTracksLoaded++;
        if (numberOfTracksLoaded != 2)
            return;

        testTrack(0);
        testTrackError(1);
        t.done();
    }

    function testTrack(index) {
        var expected = [
            { position : 10, align : "start"  },
            { position : 20, align : "center" },
            { position : 80, align : "end"    }
        ];

        assert_cues_match(video.textTracks[index].cues, expected);
    }

    function testTrackError(index) {
        var expected = [
            { position : 10,     align : "center" },
            { position : "auto", align : "center" },
            { position : "auto", align : "center" }
        ];

        assert_cues_match(video.textTracks[index].cues, expected);
    }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-align-positioning.html"
}
```
