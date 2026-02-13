# html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange-dynamically-created-track-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange-dynamically-created-track-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>'cuechange' event on dynamically created track element</title>
<meta name="timeout" content="long">
<script src="/common/media.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video>
<script>
/**
 * 'cuechange' event should be correctly dispatched on the dynamically created
 * track element.
 */
promise_test(function(t) {
  const video = document.querySelector("video");
  const track = document.createElement("track");
  track.src = "resources/cues-chrono-order.vtt";
  track.track.mode = "hidden";
  video.appendChild(track);

  const cueChangedPromise = new Promise(r => track.oncuechange = r);
  video.src = getVideoURI("/media/test");
  // 'TimeMarchesOn' algorithm will be run after calling 'play()', from which
  // the 'cuechange' event would be dispatched.
  video.play();
  return cueChangedPromise;
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-cues-cuechange-dynamically-created-track-element.html"
}
```
