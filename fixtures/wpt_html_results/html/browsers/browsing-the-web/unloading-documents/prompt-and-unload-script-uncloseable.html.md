# html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-uncloseable.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-uncloseable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>beforeunload and unload events do not fire after window.close() in script-uncloseable browsing context</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var beforeunload_fired = false;
var unload_fired = false;
var t = async_test();

onload = t.step_func(function() {
  var iframe = document.getElementsByTagName("iframe")[0]
  iframe.onload = t.step_func(function() {
    iframe.contentWindow.close()
    t.step_timeout(function() {
      assert_false(beforeunload_fired);
      assert_false(unload_fired);
      t.done();
    }, 1000);
  });
  iframe.src = "prompt-and-unload-script-uncloseable-1.html";
});
</script>
<iframe></iframe>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-uncloseable.html"
}
```
