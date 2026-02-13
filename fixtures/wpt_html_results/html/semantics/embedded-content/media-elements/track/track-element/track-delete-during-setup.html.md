# html/semantics/embedded-content/media-elements/track/track-element/track-delete-during-setup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-delete-during-setup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Track deletion during setup</title>
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
    <track src="resources/metadata.vtt">
</video>
<script>
async_test(function(t) {
    var video = document.querySelector("video");
    var track = document.querySelector("track");
    t.step_timeout(function() {
        video.parentNode.removeChild(video);
    }, 61);

    track.onload = t.step_func(function() {
        var track2 = document.createElement("track");
        video.appendChild(track2);
        t.step_timeout(t.step_func_done(), 100);
    });

    assert_equals(track.readyState, HTMLTrackElement.NONE);
    assert_equals(track.track.mode, "disabled");
    track.track.mode = "hidden";

    video.src = getVideoURI("/media/test");
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-delete-during-setup.html"
}
```
