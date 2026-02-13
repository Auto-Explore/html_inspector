# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-resumes-onload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-resumes-onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>resource selection should not delay the load event indefinitely</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<video></video>
<script>
async_test(function(t) {
  const v = document.querySelector('video');
  v.onloadstart = t.unreached_func("loadstart event should not be fired when the resource selection algorithm cannot determine mode");
  const s = document.createElement('source');
  v.appendChild(s); // this will trigger resource selection
  v.removeChild(s); // force an early return in resource selection algorithm
  window.onload = t.step_func_done(function() {
    assert_equals(v.networkState, v.NETWORK_EMPTY);
  });
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-resumes-onload.html"
}
```
