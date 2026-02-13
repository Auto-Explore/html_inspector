# html/semantics/embedded-content/media-elements/autoplay-with-broken-track.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/autoplay-with-broken-track.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#text-track-model">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<div id="log"></div>
<script>
// Media elements have a "list of pending text tracks" which should be populated
// with text tracks with readyState "loading". When the text track src is
// invalid or points to a non-existent resource, it shouldn't be possible to
// block the media element's readyState indefinitely.
function t(trackSrc) {
  const track = document.createElement('track');
  track.src = trackSrc;
  track.default = true;
  async_test(t => {
    const video = document.createElement('video');
    video.autoplay = true;
    video.controls = true; // for visual inspection, not part of test
    video.src = getVideoURI('/media/movie_5');
    video.appendChild(track);
    document.body.appendChild(video);
    // The playing event isn't used because it's fired in Safari even when the
    // playback doesn't actually start.
    video.ontimeupdate = t.step_func(() => {
      if (video.currentTime > 0)
        t.done();
    });
  }, `<video autoplay> with ${track.outerHTML} child`);
}
t("invalid://url");
t("404");
t("");
</script>
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
  "source_name": "html/semantics/embedded-content/media-elements/autoplay-with-broken-track.html"
}
```
