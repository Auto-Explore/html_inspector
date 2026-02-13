# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-pause.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-pause.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>invoking resource selection with pause()</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
var t = async_test(function(t) {
  v = document.createElement('video');
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after creating v');
  v.pause();
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after v.pause()');
});
</script>
<script>
t.step(function() {
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState in separate script');
  t.done();
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-pause.html"
}
```
