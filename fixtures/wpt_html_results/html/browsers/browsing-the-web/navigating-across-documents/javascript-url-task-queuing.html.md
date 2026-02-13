# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-task-queuing.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-task-queuing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>javascript: URL task queuing</title>
<link rel="help" href="https://github.com/whatwg/html/issues/3730">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

testIsAsync(() => {
  const iframe = document.createElement("iframe");
  document.body.append(iframe);
  iframe.contentWindow.location.href = "javascript:window.top.javascriptURLRan = true; window.top.resolveTestPromise();";
}, `Navigating an iframe via location.href to a javascript: URL must queue a task`);

testIsAsync(() => {
  const iframe = document.createElement("iframe");
  iframe.src = "javascript:window.top.javascriptURLRan = true; window.top.resolveTestPromise();";
  document.body.append(iframe);
}, `Navigating an iframe via src="" to a javascript: URL before insertion must queue a task`);

testIsAsync(() => {
  const iframe = document.createElement("iframe");
  document.body.append(iframe);
  iframe.src = "javascript:window.top.javascriptURLRan = true; window.top.resolveTestPromise();";
}, `Navigating an iframe via src="" to a javascript: URL after insertion must queue a task`);

testIsAsync(() => {
  const w = window.open();
  w.location.href = "javascript:window.opener.javascriptURLRan = true; window.opener.resolveTestPromise();";
}, `Navigating an opened window via location.href to a javascript: URL must queue a task`);

testIsAsync(() => {
  window.open("javascript:window.opener.javascriptURLRan = true; window.opener.resolveTestPromise();");
}, `Navigating an opened window as part of creation to a javascript: URL must queue a task`);

function testIsAsync(setupFunc, description) {
  promise_test(async t => {
    t.add_cleanup(() => {
      delete window.resolveTestPromise;
      delete window.javascriptURLRan;
    });

    const ranPromise = new Promise(resolve => {
      window.resolveTestPromise = resolve;
    });

    setupFunc();

    assert_equals(window.javascriptURLRan, undefined, "Must not run sync");

    // Ensure that we do actually run the code, though.
    await ranPromise;
  }, description);
}
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-task-queuing.html"
}
```
