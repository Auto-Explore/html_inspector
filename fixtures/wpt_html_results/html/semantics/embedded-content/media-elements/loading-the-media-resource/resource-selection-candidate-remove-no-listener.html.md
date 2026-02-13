# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-remove-no-listener.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-remove-no-listener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>removing the candidate source, no listener</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
function createSource(src) {
  var source = document.createElement('source');
  source.src = src;
  return source;
}
var t = async_test(function(t) {
  v = document.createElement('video');
  v.appendChild(createSource('resources/delayed-broken-video.py')); // invokes resource selection
});
</script>
<!-- now resource selection algorithm will continue its sync section (the </script> tag below provides a stable state) -->
<!-- the <source> is candidate -->
<!-- pointer is between the <source> and the end of the list -->
<script>
t.step(function() {
  v.removeChild(v.firstChild); // just tests that we don't crash
  onload = t.step_func(function() { t.done(); });
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-remove-no-listener.html"
}
```
