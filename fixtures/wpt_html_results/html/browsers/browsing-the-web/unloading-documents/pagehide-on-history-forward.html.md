# html/browsers/browsing-the-web/unloading-documents/pagehide-on-history-forward.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/pagehide-on-history-forward.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>pagehide event fires on history navigation forward</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var t = async_test();

onload = function() {setTimeout(t.step_func(function() {
  var iframe = document.getElementsByTagName("iframe")[0]

  iframe.src = "pagehide-on-history-forward-1.html";
}), 100)};

base_hide = t.step_func(function() {
  t.done()
});
</script>
<iframe src="base.html"></iframe>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/pagehide-on-history-forward.html"
}
```
