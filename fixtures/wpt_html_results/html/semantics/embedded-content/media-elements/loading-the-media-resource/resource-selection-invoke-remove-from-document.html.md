# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-from-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-from-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking resource selection by removing from document with NETWORK_EMPTY</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
test(function() {
  v = document.createElement('video');
  document.body.appendChild(v);
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after appending v to document');
  v.parentNode.removeChild(v); // search for "When a media element is removed from a Document,"
  assert_equals(v.networkState, v.NETWORK_EMPTY, 'networkState after removing v');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-from-document.html"
}
```
