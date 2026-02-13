# html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Script is not executed after script thread is shutdown</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="testiframe" src="script-not-executed-after-shutdown-child.html"></iframe>
<script>
async_test(function(t) {
  window.script_executed = t.unreached_func("script executed in removed iframe");
  let iframe = document.getElementById("testiframe");
  iframe.onload = function() {
    iframe.parentNode.removeChild(iframe);
  };
  setTimeout(function() {
    t.done();
  }, 5000);
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown.html"
}
```
