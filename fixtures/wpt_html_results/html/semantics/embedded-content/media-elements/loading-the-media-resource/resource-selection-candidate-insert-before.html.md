# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-insert-before.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-insert-before.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>inserting another source before the candidate</title>
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
  v.addEventListener('loadstart', t.step_func(function() {
    assert_equals(v.currentSrc.substr(v.currentSrc.lastIndexOf('#')), '#a');
    t.done();
  }), false);
  v.appendChild(createSource('#a')); // invokes resource selection
});
</script>
<!-- now resource selection algorithm will continue its sync section (the </script> tag below provides a stable state) -->
<!-- #a is candidate -->
<!-- pointer is between #a and the end of the list -->
<script>
t.step(function() {
  v.insertBefore(createSource('#b'), v.firstChild); // pointer is unchanged, #a is still candidate
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-candidate-insert-before.html"
}
```
