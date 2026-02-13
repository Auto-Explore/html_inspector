# html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-into-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-into-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>NOT invoking resource selection by inserting into other document with src set</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<iframe hidden></iframe>
<script>
async_test(function(t) {
  var v = document.createElement('video');
  v.src = 'data:,';
  v.onerror = t.step_func(function() {
    assert_equals(v.readyState, v.HAVE_NOTHING);
    assert_equals(v.networkState, v.NETWORK_NO_SOURCE);
    var iframe = document.querySelector('iframe');
    iframe.contentDocument.body.appendChild(v);
    v.onloadstart = t.step_func(function() { assert_unreached(); });
    // wait for an event after the above
    var v2 = document.createElement('video');
    v2.src = 'data:,';
    v2.onloadstart = t.step_func(function() { t.done(); });
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
  "source_name": "html/semantics/embedded-content/media-elements/loading-the-media-resource/resource-selection-invoke-insert-into-iframe.html"
}
```
