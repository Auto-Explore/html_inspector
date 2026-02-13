# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-fragment-into-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-fragment-into-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking resource selection by inserting document fragment into a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
var t = async_test(function(t) {
  v = document.createElement('video');
  var fragment = document.createDocumentFragment();
  fragment.appendChild(v);
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after fragment.appendChild(v)');
  document.body.appendChild(fragment);
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after document.body.appendChild(fragment)');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-fragment-into-document.html"
}
```
