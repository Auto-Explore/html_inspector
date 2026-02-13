# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-parent-into-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-parent-into-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking resource selection by inserting parent into a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
async_test(function(t) {
  var v = document.createElement('video');
  var div = document.createElement('div');
  div.appendChild(v);
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after div.appendChild(v)');
  document.body.appendChild(div);
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after document.body.appendChild(div)');
  window.onload = t.step_func(function() {
    assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState in window.onload');
    t.done();
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-parent-into-document.html"
}
```
