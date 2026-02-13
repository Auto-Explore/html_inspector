# html/browsers/browsing-the-web/unloading-documents/navigation-within-beforeunload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/navigation-within-beforeunload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Triggering navigation from within beforeunload event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
beforeunload_fired = false;
var t = async_test();

var base_count = 0;
var counter = 0;

onload = function() {setTimeout(function() {
  var iframe = document.getElementsByTagName("iframe")[0]

  iframe.onload = function() {
                    setTimeout(function() {iframe.contentWindow.location="navigation-within-beforeunload-2.html";}, 100);
                    // Step 4 of https://html.spec.whatwg.org/multipage/browsing-the-web.html#navigating-across-documents
                    // doesn't seem to allow navigation within a beforeunload handler,
                    // so the counter should not go beyond 1.
                    iframe.onload = t.step_func(function() {assert_equals(counter, 1); t.done()});
                  };

  iframe.src = "navigation-within-beforeunload-1.html?" + Math.random();

}, 100)};

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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/navigation-within-beforeunload.html"
}
```
