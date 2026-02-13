# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>invoking resource selection by setting src; await stable state</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
var t = async_test(function(t) {
  v = document.createElement('video');
  v.onloadstart = t.step_func(function() { assert_unreached(); });
  v.setAttribute('src', ''); // runs resource selection algorithm, but it will wait running the sync section until this script has finished
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE);
  v.removeAttribute('src'); // will make resource selection algorithm revert to NETWORK_EMPTY and abort (in the sync section)
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE);
  window.onload = t.step_func(function() { t.done(); });
});
</script>
<script>
t.step(function() {
  assert_equals(v.networkState, v.NETWORK_EMPTY);
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-src.html"
}
```
