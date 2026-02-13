# html/browsers/windows/document-domain-removed-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/document-domain-removed-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>document.domain and removed iframe interaction</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- This is a test for https://crbug.com/1095145 where window
     properties become undefined for document.domain-using removed
     iframes -->

<div id="log"></div>

<script>
"use strict";

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    iframe.onload = t.step_func(() => {
      const { contentWindow } = iframe;
      assert_equals(contentWindow.status, "");
      resolve();
    });
    iframe.src = "/common/blank.html";
    document.body.append(iframe);
  });
}, "No removal, no document.domain");

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    iframe.onload = t.step_func(() => {
      const { contentWindow } = iframe;
      iframe.remove();
      assert_equals(contentWindow.status, "");
      resolve();
    });
    iframe.src = "/common/blank.html";
    document.body.append(iframe);
  });
}, "Removal, no document.domain");

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    iframe.onload = t.step_func(() => {
      document.domain = document.domain;
      const { contentWindow } = iframe;
      assert_equals(contentWindow.status, "");
      resolve();
    });
    iframe.src = "resources/document-domain-setter.html";
    document.body.append(iframe);
  });
}, "No removal, document.domain");

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    iframe.onload = t.step_func(() => {
      document.domain = document.domain; // technically we already did this above
      const { contentWindow } = iframe;
      iframe.remove();
      assert_equals(contentWindow.status, "");
      resolve();
    });
    iframe.src = "resources/document-domain-setter.html";
    document.body.append(iframe);
  });
}, "Removal, document.domain");

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
  "source_name": "html/browsers/windows/document-domain-removed-iframe.html"
}
```
