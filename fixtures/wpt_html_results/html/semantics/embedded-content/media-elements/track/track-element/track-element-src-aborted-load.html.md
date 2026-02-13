# html/semantics/embedded-content/media-elements/track/track-element/track-element-src-aborted-load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/track-element-src-aborted-load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLTrackElement 'src' attribute changed, load pending</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#start-the-track-processing-model">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video></video>
<script>
async_test(t => {
  const track = document.createElement('track');
  track.onload = t.unreached_func('first source should not load');
  track.onerror = t.step_func_done();
  track.src = 'resources/settings.vtt?pipe=trickle(d3600)';
  track.track.mode = 'hidden';
  document.querySelector('video').appendChild(track);
  t.step_timeout(() => {
    track.src = 'resources/entities.vtt';
  }, 0);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/track-element-src-aborted-load.html"
}
```
