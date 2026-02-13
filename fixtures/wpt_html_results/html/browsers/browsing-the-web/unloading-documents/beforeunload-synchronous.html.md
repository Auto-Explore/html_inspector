# html/browsers/browsing-the-web/unloading-documents/beforeunload-synchronous.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/beforeunload-synchronous.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>beforeunload event is emitted synchronously</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#the-event-handler-processing-algorithm">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
'use strict';
// "navigate a browsing context" synchronously calls "prompt to unload", which
// synchronously calls "dispatch an event".

async_test(function(t) {
  var iframe = document.createElement('iframe');

  iframe.onload = t.step_func(function() {
    var callCount = 0;

    iframe.contentWindow.onbeforeunload = function() {
      callCount += 1;
    };

    iframe.contentWindow.location.href = '/common/blank.html';

    assert_equals(callCount, 1, 'invoked synchronously exactly once');

    t.done();
  });

  document.body.appendChild(iframe);
});
</script>
</body>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/beforeunload-synchronous.html"
}
```
