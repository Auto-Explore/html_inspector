# html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-header-comment.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-header-comment.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Optional comment area under the "WEBVTT" file header is properly ignored and also, default settings and styling are currently ignored (treated as faulty cues)</title>
<script src="track-helpers.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/default-styles.vtt">
    <track src="resources/metadata-area.vtt">
</video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");

    var trackElements = document.querySelectorAll("track");
    for (var i = 0; i < video.textTracks.length; i++)
        trackElements[i].onload = t.step_func(trackLoaded);

    enableAllTextTracks(video.textTracks);

    var numberOfTracksLoaded = 0;
    function trackLoaded() {
        numberOfTracksLoaded++;
        if (numberOfTracksLoaded != 2)
            return;

        testTrack(0);
        testTrack(1);
        t.done();
    }

    function testTrack(index) {
        var expected = [
            {
                id : "1",
                startTime : 0,
                endTime : 30.5,
                text : "Bear is Coming!!!!!"
            },
            {
                id : "2",
                startTime : 31,
                endTime : 1200.5,
                text : "I said Bear is coming!!!!"
            }
        ];

        assert_cues_equal(video.textTracks[index].cues, expected);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-webvtt-header-comment.html"
}
```
