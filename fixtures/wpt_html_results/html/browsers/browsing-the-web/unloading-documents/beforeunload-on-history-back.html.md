# html/browsers/browsing-the-web/unloading-documents/beforeunload-on-history-back.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/beforeunload-on-history-back.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>beforeunload event fires on history navigation back</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
beforeunload_fired = false;
var t = async_test();

var base_count = 0;

onload = function() {setTimeout(t.step_func(function() {
  var iframe = document.getElementsByTagName("iframe")[0]
  iframe.onload = t.step_func(function() {
    iframe.onload = null;
    history.go(-1);
  });

  iframe.src = "beforeunload-on-history-back-1.html";
}), 100)};

base_show = t.step_func(function() {
  base_count++;
  if (base_count > 1) {
    assert_true(beforeunload_fired);
    t.done();
  }
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/beforeunload-on-history-back.html"
}
```
