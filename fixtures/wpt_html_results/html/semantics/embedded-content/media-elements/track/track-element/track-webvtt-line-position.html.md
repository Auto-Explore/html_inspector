# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-line-position.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-line-position.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Cue line position from settings</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/line-position.vtt">
    <track src="resources/line-position-bad.vtt">
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
            { line : 0,   snapToLines : false },
            { line : 0,   snapToLines : true  },
            { line : 50,  snapToLines : false },
            { line : 5,   snapToLines : true  },
            { line : 100, snapToLines : false },
            { line : -1,  snapToLines : true  },
            { line : 500, snapToLines : true  }
        ];

        assert_cues_match(video.textTracks[index].cues, expected);
    }

    function testTrackError(index) {
        var expected = [
            { line : "auto", snapToLines : true },
            { line : "auto", snapToLines : true },
            { line : "auto", snapToLines : true },
            { line : "auto", snapToLines : true },
            { line : "auto", snapToLines : true }
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-line-position.html"
}
```
