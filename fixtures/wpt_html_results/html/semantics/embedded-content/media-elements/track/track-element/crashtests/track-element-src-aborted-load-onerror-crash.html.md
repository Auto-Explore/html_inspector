# html/semantics/embedded-content/media-elements/track/track-element/crashtests/track-element-src-aborted-load-onerror-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/crashtests/track-element-src-aborted-load-onerror-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLTrackElement 'src' attribute changed, load pending, 'error' handler mutates</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#start-the-track-processing-model">
<link rel="help" href="https://crbug.com/1374341">
<video></video>
<script>
  const video = document.querySelector('video');
  video.style.visibility = 'collapse';
  video.setAttribute('crossorigin', 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa');
  const track = document.createElement('track');
  track.src = 'x';
  track.track.mode = 'hidden';
  video.appendChild(track);
  track.onerror = () => {
    for (let i = 0; i < 10; ++i)
      video.setAttribute('foo' + i, 'bar');
  };
  setTimeout(() => {
    track.src = 'y';
  }, 0);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/crashtests/track-element-src-aborted-load-onerror-crash.html"
}
```
