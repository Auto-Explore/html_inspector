# html/semantics/embedded-content/media-elements/track/track-element/src-empty-string.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/track/track-element/src-empty-string.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Setting HTMLTrackElement.src to the empty string fires 'error' and sets readyState to ERROR</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/media.html#sourcing-out-of-band-text-tracks">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<video></video>
<script>
async_test(t => {
  let track = document.createElement("track");
  track.src = '';
  track.default = true;
  track.onerror = t.step_func_done(() => {
    assert_equals(track.readyState, HTMLTrackElement.ERROR);
  });
  track.onload = t.unreached_func('fired load');

  assert_equals(track.readyState, HTMLTrackElement.NONE);

  document.querySelector('video').appendChild(track);
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
  "source_name": "html/semantics/embedded-content/media-elements/track/track-element/src-empty-string.html"
}
```
