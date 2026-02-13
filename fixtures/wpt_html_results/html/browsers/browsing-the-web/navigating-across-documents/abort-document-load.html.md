# html/browsers/browsing-the-web/navigating-across-documents/abort-document-load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Aborting a Document load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/browsing-the-web.html#aborting-a-document-load">
<div id="log"></div>
<script>
var events = [];
onmessage = function(e) {
  events.push(e.data);
};
async_test(test => {
  test.step_timeout(() => {
    const frame = document.querySelector('iframe');
    const child = frame.contentWindow;
    assert_equals(child.document.readyState, 'complete', 'readyState is complete');
    assert_array_equals(events, ["loading", "stop"], 'no load event was fired');
    events = [];
    frame.src = "abort-document-load-2.html";

    test.step_timeout(() => {
      const child = frame.contentWindow;
      assert_equals(child.document.readyState, 'complete', 'readyState is complete');
      assert_array_equals(events, ["loading", "DOMContentLoaded", "stop", "complete"], 'no load event was fired');
      test.done();
    }, 1000);
  }, 1000);
});
</script>
<iframe src="abort-document-load-1.html"></iframe>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/abort-document-load.html"
}
```
