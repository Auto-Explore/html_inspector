# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-in-sync-event.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-in-sync-event.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>await a stable state and sync event handlers</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<video></video>
<script>
var v;
var t = async_test(function(t) {
  v = document.querySelector('video');
  var a = document.createElement('a');
  a.onclick = t.step_func(function() {
    v.setAttribute('src', '#'); // invokes media load which invokes resource selection
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState in onclick handler');
  });
  a.click(); // sync fires click, so sets src
  // now we should still await a stable state because the script hasn't
  // finished, the event handler has just returned
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after click()');
  v.removeAttribute('src');
});
</script>
<!-- now resource selection will continue its sync section (the </script> tag below provides a stable state) -->
<!-- will find neither src nor source, so sets networkState to NETWORK_EMPTY -->
<script>
t.step(function() {
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after src removed');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-in-sync-event.html"
}
```
