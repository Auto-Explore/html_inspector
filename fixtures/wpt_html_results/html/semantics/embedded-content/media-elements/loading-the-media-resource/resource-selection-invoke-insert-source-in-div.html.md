# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-source-in-div.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-source-in-div.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking resource selection by inserting &lt;source> in &lt;div> in &lt;video></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<video><div></div></video>
<script>
async_test(function(t) {
  var v = document.querySelector('video');
  v.onloadstart = t.step_func(function() { assert_unreached(); });
  v.firstChild.appendChild(document.createElement('source'));
  window.onload = t.step_func(function() { t.done(); });
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-source-in-div.html"
}
```
