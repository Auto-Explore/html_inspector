# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>the &lt;source> media attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<video><source src="resources/delayed-broken-video.py" media="not all"></video>
<script>
test(function() {
  var v = document.querySelector('video');
  var s = document.querySelector('source');
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE);
  assert_equals(v.currentSrc, '');
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-source-media.html"
}
```
