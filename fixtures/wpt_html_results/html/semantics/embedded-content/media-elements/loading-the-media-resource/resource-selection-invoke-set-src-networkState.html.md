# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-set-src-networkState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-set-src-networkState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>invoking load by setting src when networkState is not NETWORK_EMPTY</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function(t) {
  var v = document.createElement('video');
  v.play().catch(() => {}); // invokes resource selection and sets .paused to false
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState');
  assert_false(v.paused, 'paused');
  v.setAttribute('src', ''); // invokes media load which sets .paused to true
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after setting src');
  assert_true(v.paused, 'paused after setting src');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-set-src-networkState.html"
}
```
