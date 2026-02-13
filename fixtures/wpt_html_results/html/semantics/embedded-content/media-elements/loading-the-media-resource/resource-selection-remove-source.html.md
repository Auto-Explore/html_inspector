# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-source.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-source.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Changes to networkState when inserting and removing a &lt;source></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
var t = async_test(function(t) {
  v = document.createElement('video');
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState when creating the element');
  v.appendChild(document.createElement('source')); // runs resource selection algorithm
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState when inserting a source element');
  v.removeChild(v.firstChild);
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after removing the source element');
});
</script>
<!-- now resource selection will continue its sync section (the </script> tag below provides a stable state) -->
<!-- will find neither src nor source, so sets networkState to NETWORK_EMPTY -->
<script>
t.step(function() {
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after letting the sync section of resource selection run');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-remove-source.html"
}
```
