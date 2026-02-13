# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking media load or resource selection when removing the src attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var v;
var t = async_test(function(t) {
  v = document.createElement('video');
  v.setAttribute('src', ''); // invokes media load
  assert_equals(v.networkState, v.NETWORK_NO_SOURCE, 'networkState after setting src');
  var s = document.createElement('source');
  s.onerror = this.step_func(function() { assert_unreached(); });
  v.appendChild(s); // src is present so nothing happens here
  onload = this.step_func(function() { t.done(); });
});
</script>
<script>
t.step(function() {
  v.removeAttribute('src'); // nothing should happen
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-remove-src.html"
}
```
