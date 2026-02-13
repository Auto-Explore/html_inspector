# html/semantics/embedded-content/media-elements/track/track-element/track-remove-track.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-track.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />

        <script src="/common/media.js"></script>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
    </head>
    <body>
        <script>
            async_test(function(test)
            {
                var video = document.createElement("video");
                var track;

                function trackRemoved()
                {
                    assert_equals(event.target, video.textTracks);
                    assert_equals(event instanceof window.TrackEvent, true);
                    assert_equals(event.track, track);
                    test.done();
                }

                var trackElement = document.createElement('track');
                video.appendChild(trackElement);

                trackElement.src = 'resources/webvtt-file.vtt';
                trackElement.track.mode = 'hidden';

                assert_equals(video.textTracks.length, 1);

                track = video.textTracks[0];
                video.removeChild(trackElement);
                video.textTracks.addEventListener("removetrack", test.step_func(trackRemoved));
            }, "Tests that the 'removetrack' event is fired when an out-of-band TextTrack is removed.");
        </script>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-remove-track.html"
}
```
